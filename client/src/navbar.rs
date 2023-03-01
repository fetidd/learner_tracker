use std::rc::Rc;

use crate::context::AppContext;
use crate::routes::Route;
use yew::prelude::*;
use yew_router::{prelude::*};

#[function_component]
pub fn Navbar(p: &NavbarProps) -> Html {
    let navigator = use_navigator().expect("didn't get a navigator");
    let logout_handler = p.logout_handler.clone();
    let logout = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            logout_handler.emit(());
            navigator.clone().replace(&Route::Login);
        })
    };
    let ctx = use_context::<Rc<AppContext>>().expect("NO CONTEXT IN NAVBAR");
    html! {
        <nav id="navbar" class={classes!("w-full", "flex", "justify-between", "bg-slate-100", "h-16", "items-center", "px-3")}>
            <div class={classes!("flex", "items-center", "space-x-10")}>
                <div><input class={classes!("md:w-96")} type="text" /></div>
            </div>
            <div class={classes!("hidden", "md:block")}>
                <div class={classes!("flex", "items-center", "space-x-5")}>
                    <span>{&format!("Hi, {}!", ctx.current_user.first_names)}</span>
                    <button class={classes!("bg-red-100", "hover:bg-red-200")} onclick={logout}>{"Log out"}</button>
                </div>
            </div>
        </nav>
    }
}

#[derive(Properties, PartialEq)]
pub struct NavbarProps {
    pub logout_handler: Callback<()>,
}
