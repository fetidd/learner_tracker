use argon2::Argon2;
use password_hash::SaltString;
use rand::thread_rng;
use web_sys::HtmlInputElement;
use yew::function_component;

use yew::prelude::*;
use yew_router::prelude::*;
use crate::{routes::Route};

#[function_component(LoginForm)]
pub fn login_form(p: &LoginFormProps) -> Html {
    let navigator = use_navigator().expect("didn't get a navigator");
    let entered_email = use_node_ref();
    let entered_password = use_node_ref();
    let login_callback = p.login_handler.clone();
    let login_callback = {
        let entered_email = entered_email.clone();
        let entered_password = entered_password.clone();
        Callback::from(move |_| {
            let password = entered_password.cast::<HtmlInputElement>().unwrap().value();
            let _salt = SaltString::generate(thread_rng()); // TODOSERVER implement password hashing!!!
            let _argon2 = Argon2::default();
            let hashed_password = password;
            login_callback.emit((
                entered_email.cast::<HtmlInputElement>().unwrap().value(), 
                hashed_password
            ));
            navigator.replace(&Route::Menu);
        })
    };
    html! {
        <div class={classes!("level", "is-flex", "is-justify-content-center")}>
            <form class={classes!{"box", "is-flex", "is-flex-direction-column"}}>
                <div class={classes!("field")}>
                    <div class={classes!("control")}>
                        <input class={classes!("input")} type={"text"} placeholder={"Email address"} autocomplete={"username"} ref={entered_email}/>
                    </div>
                </div>
                <div class={classes!("field")}>
                    <div class={classes!("control")}>
                        <input class={classes!("input")} type={"password"} placeholder={"Password"} autocomplete={"current-password"} ref={entered_password}/>
                    </div>
                </div>
                <div class={classes!("control", "is-flex", "is-justify-content-center")}>
                    <button class={classes!("button", "is-link")} onclick={login_callback}>{"Submit"}</button>
                </div>
            </form>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct LoginFormProps {
    pub login_handler: Callback<(String, String)>,
}