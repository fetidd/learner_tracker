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
        if let Some(user) = &p.current_user {
        <nav class={classes!("w-full", "flex", "justify-between", "bg-slate-100", "h-16",  "items-center", "px-3")}>
            <div class={classes!("flex", "items-center", "space-x-10")}>
                <Link<Route> to={Route::Menu}><div class={classes!("px-4")}><span class={classes!("text-2xl", "font-bold")}>{"Menu"}</span></div></Link<Route>>
                <div><input class={classes!("w-96")} type={"text"} /></div>
            </div>
            <div>
                <div class={classes!("flex", "items-center", "space-x-5")}>
                    <span>{&format!("Hi, {}!", user.first_names)}</span>
                    <button class={classes!("bg-red-300")} onclick={logout}>{"Log out"}</button>
                </div>
            </div>
            </nav>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct NavbarProps {
    pub current_user: Option<User>,
    pub logout_handler: Callback<()>,
}
