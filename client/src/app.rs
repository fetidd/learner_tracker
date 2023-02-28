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
    let login_handler: Callback<(String, String)> = {
        Callback::from(|(email, pass)| {
            login(email, pass);
        })
    };
    let logout_handler: Callback<()> = {
        Callback::from(|_| {
            logout();
        })
    };
    let routing_callback = {
        let login_handler = login_handler.clone();
        let logout_handler = logout_handler.clone();
        Callback::from(move |route: Route| {
            match route {
                Route::Menu         => html! { 
                    <>
                        <navbar::Navbar logout_handler={logout_handler.clone()} />
                        <menu::Menu />
                    </> 
                },
                Route::ManagePupils => html! { 
                    <>
                        <navbar::Navbar logout_handler={logout_handler.clone()} />
                        <pupils::PupilTable />
                    </> 
                },
                Route::ManageUsers  => html! { 
                    <>
                        <navbar::Navbar logout_handler={logout_handler.clone()} />
                        <pupils::PupilTable />
                    </> 
                },
                Route::Pupil { id } => html! { 
                    <>
                        <navbar::Navbar logout_handler={logout_handler.clone()} />
                        <pupils::PupilDetails {id} />
                    </> 
                },
                Route::Login        => html! { 
                    <>
                        <login::LoginForm login_handler={login_handler.clone()} />
                    </> 
                },
            }
        })
    };
    // check for stored user
    let stored_user = get_stored_user();
    if let Some(user) = stored_user {
        let current_user = Rc::new(user);
        debug!("CURRENT USER = ", &current_user.as_ref().email_address);
        html! {
            <BrowserRouter>
            <ContextProvider<Rc<User>> context={current_user}>
                    <Switch<Route> render={routing_callback} />
            </ContextProvider<Rc<User>>>
            </BrowserRouter>
        }
    } else {
        html!{
            <BrowserRouter>
                <Switch<Route> render={routing_callback} />
            </BrowserRouter>
        }
    }

}

// ====================================================================================================================================================

fn get_stored_user() -> Option<User> {
    match utils::get_current_token() {
        Ok(token) => {
            match utils::decode_auth_token(token) {
                Ok(user) => {
                    debug!(
                        "found user in sessionstorage =>",
                        user.email_address.clone()
                    );
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

fn login(email: String, password: String) {
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
                                if let Err(error) = SessionStorage::set(constant::AUTH_TOKEN_STORAGE_KEY, token.clone()) {
                                    error!(error.to_string());
                                    return;
                                }
                            }
                            _ => error!("no or blank token"),
                        },
                        Some(err) => error!(err.to_string()),
                    }
                }
            }
            Err(err) => error!(err.to_string()),
        }
    });
}

fn logout() {
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
    });
}

#[derive(Deserialize)]
struct LoginResponseJson {
    error: Option<String>,
    token: Option<String>,
}
