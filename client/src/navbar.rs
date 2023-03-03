use std::rc::Rc;
use crate::{
    app::AppContext,
    routes::Route,
    search::SearchBar, elements::Button,
};
use yew::prelude::*;
use yew_router::{prelude::*};

#[function_component]
pub fn Navbar(_: &NavbarProps) -> Html {
    let navigator = use_navigator().expect("didn't get a navigator");
    let ctx = use_context::<Rc<AppContext>>().expect("NO CONTEXT IN NAVBAR");
    let logout_callback = ctx.logout_callback.clone();
    let logout = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            logout_callback.emit(());
            navigator.clone().replace(&Route::Login);
        })
    };
    html! {
        <nav id="navbar" class="w-full flex justify-between bg-slate-100 h-full items-center px-3">
            <SearchBar />
            <div class="hidden md:block">
                <div class="flex items-center space-x-5">
                    <span>{&format!("Hi, {}!", ctx.current_user.first_names)}</span>
                    <Button color="red" onclick={logout} text="Log out"/>
                </div>
            </div>
        </nav>
    }
}

#[derive(Properties, PartialEq)]
pub struct NavbarProps {}
