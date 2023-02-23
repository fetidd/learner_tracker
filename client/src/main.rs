mod app;
mod constant;
mod login;
mod macros;
mod menu;
mod models;
mod navbar;
mod pupils;
mod routes;
mod utils;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
