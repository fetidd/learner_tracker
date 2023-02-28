use crate::utils;
use crate::{
    constant, debug, error, login, menu, models::User, navbar, pupils, routes::Route,
};
use gloo_net::http::Request;
use gloo_storage::{SessionStorage, Storage};
use serde::Deserialize;
use std::{collections::HashMap, rc::Rc};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    console_error_panic_hook::set_once();
    let current_user = use_state(|| get_stored_user());
    let login_handler: Callback<(String, String)> = {
        let user_handle = current_user.clone();
        Callback::from(move |(email, pass)| {
            login(email, pass, user_handle.clone());
        })
    };
    let logout_handler: Callback<()> = {
        let user_handle = current_user.clone();
        Callback::from(move |_| {
            logout(user_handle.clone());
        })
    };

    html! {
        <BrowserRouter>
            <ContextProvider<Rc<Option<User>>> context={Rc::new((*current_user).clone())}>
                <Switch<Route> render={
                    let login_handler = login_handler.clone();
                    let logout_handler = logout_handler.clone();
                    let current_user = current_user.clone();
                    Callback::from(move |route: Route| {
                        if (*current_user).is_some() {
                            html! {
                                <><navbar::Navbar logout_handler={logout_handler.clone()} />
                                <div class={classes!("p-2")}>
                                    {match route {
                                        Route::Menu | Route::Login => html! { <menu::Menu />},
                                        Route::ManagePupils => html! { <pupils::PupilTable />},
                                        Route::ManageUsers => html! { <pupils::PupilTable />},
                                        Route::Pupil { id } => html! { <pupils::PupilDetails {id} />},
                                    }}
                                </div></> 
                            }
                        } else {
                            html!(<login::LoginForm login_handler={login_handler.clone()} />)
                        }
                    })
                } />
            </ContextProvider<Rc<Option<User>>>>
        </BrowserRouter>
    }
}

// ====================================================================================================================================================

fn get_stored_user() -> Option<User> {
    match utils::get_current_token() {
        Ok(token) => {
            match utils::decode_auth_token(token) {
                Ok(user) => {
                    Some(user)
                }
                Err(error) => {
                    error!(error.to_string());
                    None
                }
            }
        }
        Err(error) => {
            error!(error.to_string());
            None
        }
    }
}

fn login(email: String, password: String, user_handle: UseStateHandle<Option<User>>) {
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
                            Some(token) if !token.is_empty() => {
                                match SessionStorage::set(constant::AUTH_TOKEN_STORAGE_KEY, token.clone()) {
                                    Ok(_) => {
                                        match utils::decode_auth_token(token) {
                                            Ok(user) => user_handle.set(Some(user)),
                                            Err(error) => error!("error decoding auth token:", error.to_string())
                                        }
                                    }
                                    Err(error) => error!("error setting auth token in session storage:", error.to_string())
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

fn logout(user_handle: UseStateHandle<Option<User>>) {
    spawn_local(async move {
        match SessionStorage::get::<String>(constant::AUTH_TOKEN_STORAGE_KEY) {
            Ok(token) => if let Err(error) = Request::get(constant::LOGOUT_PATH).header("Authorization", &format!("Bearer {token}")).send().await {
                error!(error.to_string());
            },
            Err(error) => {
                error!(error.to_string());
            }
        }
        SessionStorage::delete(constant::AUTH_TOKEN_STORAGE_KEY);
        user_handle.set(None);
    });
}

#[derive(Deserialize)]
struct LoginResponseJson {
    error: Option<String>,
    token: Option<String>,
}
