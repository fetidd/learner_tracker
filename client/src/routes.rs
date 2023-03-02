use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[not_found]
    #[at("/login")]
    Login,
    #[at("/pupils")]
    ManagePupils,
    #[at("/users")]
    ManageUsers,
}
