use yew::prelude::*;
use yew_feather::*;

#[function_component(IconButton)]
pub fn icon_button(props: &IconButtonProps) -> Html {
    let icon = match props.icon.as_str() {
        "close" => html!(<X />),
        "filter" => html!(<Filter />),
        "add" => html!(<PlusSquare />),
        "refresh" => html!(<RefreshCcw />),
        "search" => html!(<Search />),
        "logout" => html!(<LogOut />),
        unknown => panic!("{unknown} not a recognised button, maybe needs adding from yew_feather?")
    };

    html!(<button onclick={props.onclick.clone()}>{icon}</button>)
}

#[derive(PartialEq, Properties)]
pub struct IconButtonProps {
    pub onclick: Callback<MouseEvent>,
    pub icon: String,
}
