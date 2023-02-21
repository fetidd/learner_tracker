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
        <nav class={classes!("navbar")}>
            <div class={classes!("navbar-brand")}>
                <Link<Route> to={Route::Menu}>
                    <a class={classes!("navbar-item")}>{"Main Menu"}</a>
                </Link<Route>>
            </div>
            <div class={classes!("navbar-menu")}>
                <div class={classes!("navbar-start")}></div>
                <div class={classes!("navbar-end")}>
                    if let Some(user) = &p.current_user {
                        <div class={classes!("navbar-item")}>
                           <span>{&format!("Hi, {}!", user.first_names)}</span>
                        </div>
                        <div class={classes!("navbar-item")}>
                           <a class={classes!("button", "is-danger")} onclick={logout}>{"Log out"}</a>
                        </div>
                    }
                </div>
            </div>
        </nav>
    }
}

#[derive(Properties, PartialEq)]
pub struct NavbarProps {
    pub current_user: Option<User>,
    pub logout_handler: Callback<()>,
}
