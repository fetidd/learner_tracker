use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Menu)]
pub fn menu() -> Html {
    html! {
        <div id="menu" class="flex flex-col justify-between bg-slate-100 h-full">
            <div class="flex flex-col gap-2 m-2">
                <MenuItem route={Route::ManagePupils} title="Manage pupils"/>
                <MenuItem route={Route::ManagePupils} title="General comments"/>
                <MenuItem route={Route::ManagePupils} title="Test results"/>
                <MenuItem route={Route::ManagePupils} title="My concern"/>
                <MenuItem route={Route::ManageUsers} title="Manage users"/>
            </div>

            <span class="text-xs justify-self-end m-2">{"Contact: fetiddius@gmail.com"}</span>
        </div>
    }
}

#[function_component(MenuItem)]
fn menu_item(p: &MenuItemProps) -> Html {
    html! {
        <div class={format!("bg-slate-300 hover:bg-slate-200 border-2 border-slate-300 rounded-md")}>
            <Link<Route> to={p.route.clone()}>
                <p class="text-md text-center">{p.title.to_owned()}</p>
            </Link<Route>>
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct MenuItemProps {
    route: Route,
    title: String,
}
