use crate::routes::Route;
use yew::{classes, prelude::*};
use yew_router::prelude::*;

#[function_component(Menu)]
pub fn menu() -> Html {
  html! {
    <div class={classes!("tile", "is-ancestor")}>
      <div class={classes!("tile", "is-parent", "is-vertical", "is-6")}>
        <MenuTile route={Route::ManagePupils} title={"Manage pupils"} subtitle={"Add, edit and delete pupils in your year groups"}/>
        <MenuTile route={Route::Score1} title={"Score One"} subtitle={"Manage score one"}/>
        <MenuTile route={Route::Score1} title={"Score Two"} subtitle={"Manage score one"}/>
        <MenuTile route={Route::Score1} title={"Score Three"} subtitle={"Manage score one"}/>
      </div>
      <div class={classes!("tile", "is-parent", "is-vertical", "is-6")}>
        <div class={classes!("tile","box", "is-child")}>
          {"Welcome to the Pupil Tracker!"}
        </div>
      </div>
    </div>
  }
}

#[function_component(MenuTile)]
fn menu_tile(p: &MenuTileProps) -> Html {
  html! {
    <Link<Route> to={p.route.clone()} classes={classes!("tile", "notification", "is-child", "is-primary")}>
      <div>
      <p class={classes!("title")}>{p.title.to_owned()}</p>
      <p class={classes!("subtitle")}>{p.subtitle.to_owned()}</p>
      </div>
    </Link<Route>>
  }
}

#[derive(PartialEq, Properties)]
pub struct MenuTileProps {
  route: Route,
  title: String,
  subtitle: String
}
