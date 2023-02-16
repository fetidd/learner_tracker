use crate::{
    constant, debug, error, log, login, menu, models::User, navbar, pupils, routes::Route,
};
use gloo_net::http::Request;
use gloo_storage::{SessionStorage, Storage};
use serde::Deserialize;
use std::collections::HashMap;
use uuid::Uuid;
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
                    if let Ok(user_value) =
                        SessionStorage::get(constant::CURRENT_USER_SESSIONSTORAGE_KEY)
                    {
                        let user_value: User = user_value;
                        debug!(
                            "found user in sessionstorage =>",
                            user_value.email_address.clone()
                        );
                        // TODOCLIENT verify user with backend to ensure the user has a valid session
                        current_user.set(Some(user_value));
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
                match SessionStorage::get::<Uuid>(constant::SESSION_ID_SESSIONSTORAGE_KEY) {
                    Ok(session_id) => {
                        match Request::post(constant::LOGOUT_PATH).json(&session_id) {
                            Ok(logout_response) => {
                                if let Err(error) = logout_response.send().await {
                                    error!(error.to_string())
                                }
                            },
                            Err(error) => {
                                error!(error.to_string());
                            }
                        }
                    },
                    Err(error) => {
                        error!(error.to_string());
                    }
                }
                SessionStorage::delete(constant::CURRENT_USER_SESSIONSTORAGE_KEY);
                SessionStorage::delete(constant::SESSION_ID_SESSIONSTORAGE_KEY);
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
                (Route::Score1, true) => html! {"score1"},
                _ => html! { <login::LoginForm  login_handler={login}/> },
            }
        })
    };

    html! {
        <BrowserRouter>
        <navbar::Navbar
            current_user={(*current_user).clone()}
            logout_handler={logout}
        />
        <div class={classes!("mx-6", "my-4")}>
            <Switch<Route> render={routing_callback} />
        </div>
        </BrowserRouter>
    }
}

fn login(email: String, password: String, user_handle: UseStateHandle<Option<User>>) {
    // TEST try fantoccini
    spawn_local(async move {
        let response = Request::post(constant::LOGIN_PATH)
            .json(&HashMap::from([
                ("email_address", email.to_owned()),
                ("hashed_password", password.to_owned()),
            ]))
            .unwrap()
            .send()
            .await;
        match response {
            Ok(res) => {
                if let Ok(login_response) = res.json::<LoginResponseJson>().await {
                    match login_response.error {
                        None => {
                            let logged_in_user = login_response.user;
                            let session_id = login_response.session_id;
                            user_handle.set(logged_in_user.clone());
                            match (SessionStorage::set(constant::CURRENT_USER_SESSIONSTORAGE_KEY, logged_in_user.clone()), SessionStorage::set(constant::SESSION_ID_SESSIONSTORAGE_KEY, session_id.clone())) {
                                (Ok(_), Ok(_)) => debug!("saved login response to sessionstorage =>", logged_in_user.expect("this should be here if there was no error in the response...").email_address, session_id.expect("this should be here if there was no error in the response...").to_string()),
                                (Err(err1), Err(err2)) => error!(err1.to_string(), "and", err2.to_string()),
                                (Ok(_), Err(err)) => error!( err.to_string()),
                                (Err(err), Ok(_)) => error!(err.to_string())
                            }
                        }
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
    session_id: Option<Uuid>,
    user: Option<User>,
}
