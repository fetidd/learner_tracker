use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

static GREEN: &str =    "bg-green-100 hover:bg-green-200";
static RED: &str =      "bg-red-100 hover:bg-red-200";
static BLUE: &str =     "bg-blue-100 hover:bg-blue-200";
static YELLOW: &str =   "bg-yellow-100 hover:bg-yellow-200";
static PURPLE: &str =   "bg-purple-100 hover:bg-purple-200";
static ORANGE: &str =   "bg-orange-100 hover:bg-orange-200";

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
    let color = match p.color.as_str() {
        "green" => GREEN,
        "red" => RED,
        "blue" => BLUE,
        "yellow" => YELLOW,
        "purple" => PURPLE,
        "orange" => ORANGE,
        _ => "bg-white"
    };
    html! {
        <div class={format!("{color} p-2 rounded")}>
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
