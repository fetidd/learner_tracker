use crate::routes::Route;
use yew::{classes, prelude::*};
use yew_router::prelude::*;

#[function_component(Menu)]
pub fn menu() -> Html {
    html! {
      <div id="menu" class={classes!("container", "mx-auto", "space-y-3", "my-10")}>
        <div class={classes!("bg-green-100", "hover:bg-green-200", "p-2")}>
          <MenuTile route={Route::ManagePupils} title={"Manage pupils"} subtitle={"Add, edit and delete pupils in your year groups"}/>
        </div>
        <div class={classes!("bg-green-100", "hover:bg-green-200", "p-2")}>
          <MenuTile route={Route::ManageUsers} title={"Manage users"} subtitle={"Add, edit and delete users of the tracker"}/>
        </div>
      </div>
    }
}

#[function_component(MenuTile)]
fn menu_tile(p: &MenuTileProps) -> Html {
    html! {
      <Link<Route> to={p.route.clone()} classes={classes!()}>
        <div>
        <h1 class={classes!("text-xl")}>{p.title.to_owned()}</h1>
        <p class={classes!()}>{p.subtitle.to_owned()}</p>
        </div>
      </Link<Route>>
    }
}

#[derive(PartialEq, Properties)]
pub struct MenuTileProps {
    route: Route,
    title: String,
    subtitle: String,
}
