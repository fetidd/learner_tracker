use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/login")]
    Login,
    #[at("/")]
    Menu,
    #[at("/pupils")]
    ManagePupils,
    #[at("/pupils/:id")]
    Pupil { id: String },
    #[at("/users")]
    ManageUsers,
}
