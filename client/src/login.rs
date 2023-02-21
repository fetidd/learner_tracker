use web_sys::HtmlInputElement;
use yew::function_component;

use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

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
            let hashed_password = password;
            login_callback.emit((entered_email.cast::<HtmlInputElement>().unwrap().value(), hashed_password));
            navigator.replace(&Route::Menu);
        })
    };
    html! {
        <form class={classes!()}>
            <div class={classes!()}>
                <div class={classes!()}>
                    <input class={classes!()} type={"text"} placeholder={"Email address"} autocomplete={"username"} ref={entered_email}/>
                </div>
            </div>
            <div class={classes!()}>
                <div class={classes!()}>
                    <input class={classes!("input")} type={"password"} placeholder={"Password"} autocomplete={"current-password"} ref={entered_password}/>
                </div>
            </div>
            <div class={classes!()}>
                <button class={classes!("bg-green-100")} onclick={login_callback}>{"Submit"}</button>
            </div>
        </form>
    }
}

#[derive(Properties, PartialEq)]
pub struct LoginFormProps {
    pub login_handler: Callback<(String, String)>,
}
