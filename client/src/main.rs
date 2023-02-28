#[macro_use]
mod macros;
#[macro_use]
mod error;
mod app;
mod constant;
mod login;
mod menu;
mod navbar;
mod pupils;
mod users;
mod routes;
mod utils;
mod search;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
