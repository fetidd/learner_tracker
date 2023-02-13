use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/login")]
    Login,
    #[at("/")]
    Menu,
    #[at("/pupils")]
    ManagePupils,
    #[at("/users")]
    ManageUsers,
    #[at("/score1")]
    Score1,
}
