use crate::models::User;
use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn Navbar(p: &NavbarProps) -> Html {
    let navigator = use_navigator().expect("didn't get a navigator");
    let logout_handler = p.logout_handler.clone();
    let logout = Callback::from(move |_| {
        logout_handler.emit(());
        navigator.replace(&Route::Login);
    });
    html! {
        <nav class={classes!("w-full", "flex", "justify-between", "bg-gray-100")}>
        if let Some(user) = &p.current_user {
            <div><input type={"text"} /></div>
            <div class={classes!()}>
                <span>{&format!("Hi, {}!", user.first_names)}</span>
                <button class={classes!("bg-red-300")} onclick={logout}>{"Log out"}</button>
            </div>
            }
        </nav>
    }
}

#[derive(Properties, PartialEq)]
pub struct NavbarProps {
    pub current_user: Option<User>,
    pub logout_handler: Callback<()>,
}
