use yew::prelude::*;

#[function_component(IconButton)]
pub fn icon_button(props: &IconButtonProps) -> Html {
    let icon = match props.icon.as_str() {
        "close" => "X", // TODO make this an actual icon
        _ => "?",
    };

    html!(<button class="" onclick={props.onclick.clone()}><i>{icon}</i></button>)
}

#[derive(PartialEq, Properties)]
pub struct IconButtonProps {
    pub onclick: Callback<MouseEvent>,
    pub icon: String,
}
