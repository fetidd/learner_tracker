#[macro_use]
mod macros;
#[macro_use]
mod error;
mod app;
mod constant;
mod elements;
mod login;
mod menu;
mod navbar;
mod pupils;
mod routes;
mod search;
mod users;
mod utils;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
