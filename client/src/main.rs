mod login;
mod menu;
mod models;
mod navbar;
mod pupils;
mod routes;
mod scores;
mod constant;
mod macros;
mod app;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
