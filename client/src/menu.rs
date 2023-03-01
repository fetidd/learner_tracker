use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Menu)]
pub fn menu() -> Html {
    html! {
      <div id="menu" class="space-y-3 p-2 bg-slate-100">
          <MenuTile route={Route::ManagePupils} color="green" title="Manage pupils"/>
          <MenuTile route={Route::ManagePupils} color="green" title="General comments"/>
          <MenuTile route={Route::ManagePupils} color="green" title="Test results"/>
          <MenuTile route={Route::ManagePupils} color="green" title="My concern"/>
          <MenuTile route={Route::ManageUsers} color="red" title="Manage users"/>
      </div>
    }
}

#[function_component(MenuTile)]
fn menu_tile(p: &MenuTileProps) -> Html {
    html! {
        <div class={format!("bg-{}-100 hover:bg-{}-200 p-2 rounded", &p.color, &p.color)}>
            <Link<Route> to={p.route.clone()}>
                <p class="text-md text-center">{p.title.to_owned()}</p>
            </Link<Route>>
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct MenuTileProps {
    route: Route,
    title: String,
    color: String
}
