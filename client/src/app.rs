use crate::utils;
use crate::{constant, debug, error, log, login, menu, models::User, navbar, pupils, routes::Route};
use gloo_net::http::Request;
use gloo_storage::{SessionStorage, Storage};
use serde::Deserialize;
use std::collections::HashMap;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let current_user: UseStateHandle<Option<User>> = use_state(|| None);
    {
        let current_user = current_user.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    if let Ok(stored_token) = SessionStorage::get::<String>(constant::AUTH_TOKEN_STORAGE_KEY) {
                        match utils::decode_auth_token(stored_token) {
                            Ok(user) => {
                                debug!("found user in sessionstorage =>", user.email_address.clone());
                                current_user.set(Some(user));
                            }
                            Err(error) => error!(error.to_string()),
                        }
                    }
                });
                || ()
            },
            (),
        );
    }

    let logout: Callback<()> = {
        let current_user = current_user.clone();
        Callback::from(move |_| {
            spawn_local(async move {
                match SessionStorage::get::<String>(constant::AUTH_TOKEN_STORAGE_KEY) {
                    Ok(token) => match Request::get(constant::LOGOUT_PATH).header("Authorization", &format!("Bearer {token}")).send().await {
                        Ok(_) => {
                            log!("logged out");
                        }
                        Err(error) => {
                            error!(error.to_string());
                        }
                    },
                    Err(error) => {
                        error!(error.to_string());
                    }
                }
                SessionStorage::delete(constant::AUTH_TOKEN_STORAGE_KEY);
            });
            current_user.set(None);
        })
    };

    let login: Callback<(String, String)> = {
        let current_user = current_user.clone();
        Callback::from(move |(email, pass)| {
            let current_user = current_user.clone();
            login(email, pass, current_user);
        })
    };

    let routing_callback = {
        // TODOCLIENT turn this into a ContextProvider
        let current_user = current_user.clone();
        Callback::from(move |route: Route| {
            let current_user = (*current_user).clone();
            // current_route.set(route.clone());
            let login = login.clone();
            match (route, current_user.is_some()) {
                (Route::Menu, true) => html! { <menu::Menu />},
                (Route::ManagePupils, true) => {
                    html! { <pupils::PupilTable current_user={current_user} />}
                }
                (Route::ManageUsers, true) => todo!(),
                _ => html! { <login::LoginForm  login_handler={login}/> },
            }
        })
    };

    html! {
        <div class={classes!("p-2")}>
        <BrowserRouter>
        <navbar::Navbar
            current_user={(*current_user).clone()}
            logout_handler={logout}
        />
        <div class={classes!()}>
            <Switch<Route> render={routing_callback} />
        </div>
        </BrowserRouter>
        </div>
    }
}

fn login(email: String, password: String, user_handle: UseStateHandle<Option<User>>) {
    // TEST try fantoccini
    spawn_local(async move {
        let response = Request::post(constant::LOGIN_PATH)
            .json(&HashMap::from([("email_address", email.to_owned()), ("hashed_password", password.to_owned())]))
            .unwrap()
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
                                }
                                match utils::decode_auth_token(token) {
                                    Ok(user) => {
                                        user_handle.set(Some(user));
                                    }
                                    Err(_) => todo!(),
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

#[derive(Deserialize)]
struct LoginResponseJson {
    error: Option<String>,
    token: Option<String>,
}
