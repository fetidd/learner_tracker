use crate::elements::ModalProvider;
use crate::utils;
use crate::{constant, debug, error, login, menu, navbar, pupils, routes::Route, users::User};
use gloo_net::http::Request;
use gloo_storage::{errors::StorageError, SessionStorage, Storage};
use serde::Deserialize;
use std::{collections::HashMap, rc::Rc};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct AppContext {
    pub current_user: User,
    pub auth_token: String,
    pub login_callback: Callback<(String, String)>,
    pub logout_callback: Callback<()>,
}

#[function_component(App)]
pub fn app() -> Html {
    console_error_panic_hook::set_once();

    let state = use_state_eq(|| {
        let grab_token: Result<String, StorageError> =
            SessionStorage::get(constant::AUTH_TOKEN_STORAGE_KEY);
        let grab_user: Result<User, StorageError> = SessionStorage::get(constant::USER_STORAGE_KEY);
        if let (Ok(token), Ok(user)) = (grab_token, grab_user) {
            let decoded = utils::decode_auth_token(token.clone()).expect("failed decoding authtoken when getting stored state, how did it save it when it was invalid?!");
            if decoded.email_address != user.email_address {
                error!("user in token did not match stored user");
                return None;
            }
            Some((user, token))
        } else {
            None
        }
    });

    let login_handler: Callback<(String, String)> = {
        let state_handle = state.clone();
        Callback::from(move |(email, pass)| {
            login(email, pass, state_handle.clone());
        })
    };
    let logout_handler: Callback<()> = {
        let state_handle = state.clone();
        Callback::from(move |_| {
            logout(state_handle.clone());
        })
    };

    html! {
        <div id="app" class="bg-slate-100">
            <BrowserRouter>
                <Switch<Route> render={
                    let login_handler = login_handler.clone();
                    let logout_handler = logout_handler.clone();
                    Callback::from(move |route: Route| {
                        clone!(login_handler, logout_handler);
                        if (*state).is_some() {
                            let state = (*state).clone().unwrap();
                            let context = AppContext {
                                current_user: (state).0,
                                auth_token: (state).1,
                                login_callback: login_handler.clone(),
                                logout_callback: logout_handler.clone()
                            };
                            html! {
                                <ContextProvider<Rc<AppContext>> context={Rc::new(context)}>
                                    <ModalProvider>
                                    // ACTUAL UI ====================================================================

                                        <navbar::Navbar />
                                        <menu::Menu />
                                        <div id="router-area">
                                            {match route {
                                                Route::Login |
                                                Route::ManagePupils  => html! { <pupils::PupilTable />},
                                                Route::ManageUsers   => html! { <pupils::PupilTable />},
                                            }}
                                        </div>

                                        // ACTUAL UI ====================================================================
                                    </ModalProvider>
                                </ContextProvider<Rc<AppContext>>>
                            }
                        } else {
                            debug!("no state, going to login...");
                            html!(<login::LoginForm login_handler={login_handler.clone()} />)
                        }
                    })
                } />
            </BrowserRouter>
        </div>
    }
}

// ====================================================================================================================================================

fn login(email: String, password: String, state_handle: UseStateHandle<Option<(User, String)>>) {
    // TEST try fantoccini
    debug!("logging in with", &email, ":", &password);
    spawn_local(async move {
        let response = Request::post(constant::LOGIN_PATH)
            .json(&HashMap::from([
                ("email_address", email.to_owned()),
                ("hashed_password", password.to_owned()),
            ]))
            .expect("app::login() should not fail")
            .send()
            .await;
        match response {
            Ok(res) => {
                if let Ok(login_response) = res.json::<LoginResponseJson>().await {
                    match login_response.error {
                        None => match login_response.token {
                            Some(auth_token) if !auth_token.is_empty() => {
                                match SessionStorage::set(
                                    constant::AUTH_TOKEN_STORAGE_KEY,
                                    auth_token.clone(),
                                ) {
                                    Ok(_) => match utils::decode_auth_token(auth_token.clone()) {
                                        Ok(current_user) => {
                                            if let Err(error) = SessionStorage::set(
                                                constant::USER_STORAGE_KEY,
                                                current_user.clone(),
                                            ) {
                                                error!(
                                                    "storing user in sessionstorage failed:",
                                                    error.to_string()
                                                );
                                            }
                                            let new_ctx = (current_user, auth_token);
                                            state_handle.set(Some(new_ctx));
                                        }
                                        Err(error) => {
                                            error!("error decoding auth token:", error.to_string())
                                        }
                                    },
                                    Err(error) => error!(
                                        "error setting auth token in session storage:",
                                        error.to_string()
                                    ),
                                }
                            }
                            _ => error!("login response had no or blank token"),
                        },
                        Some(err) => error!("error in login response:", err.to_string()),
                    }
                }
            }
            Err(err) => error!(err.to_string()),
        }
    });
}

fn logout(state_handle: UseStateHandle<Option<(User, String)>>) {
    spawn_local(async move {
        match SessionStorage::get::<String>(constant::AUTH_TOKEN_STORAGE_KEY) {
            Ok(token) => {
                if let Err(error) = Request::get(constant::LOGOUT_PATH)
                    .header("Authorization", &format!("Bearer {token}"))
                    .send()
                    .await
                {
                    error!(error.to_string());
                }
            }
            Err(error) => {
                error!(error.to_string());
            }
        }
        SessionStorage::delete(constant::AUTH_TOKEN_STORAGE_KEY);
        SessionStorage::delete(constant::USER_STORAGE_KEY);
        state_handle.set(None);
    });
}

#[derive(Deserialize)]
struct LoginResponseJson {
    error: Option<String>,
    token: Option<String>,
}
