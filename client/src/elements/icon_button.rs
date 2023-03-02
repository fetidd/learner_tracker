use yew::prelude::*;

#[function_component(IconButton)]
pub fn icon_button(props: &IconButtonProps) -> Html {
    html!(<button class="" onclick={props.onclick.clone()}><i>{"X"}</i></button>)
}

#[derive(PartialEq, Properties)]
pub struct IconButtonProps {
    pub onclick: Callback<MouseEvent>,
    pub icon: String,
}