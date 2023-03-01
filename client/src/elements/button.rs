use yew::prelude::*;
use crate::{
    clone
};

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let class = format!("bg-{}-100 hover:bg-{}-200 w-[150px] border-solid border-2 border-{}-200", &props.color, &props.color, &props.color);

    html!(<button class={class} onclick={props.onclick.clone()} >{&props.text}</button>)
}

#[derive(PartialEq, Properties)]
pub struct ButtonProps {
    pub onclick: Callback<MouseEvent>,
    pub text: String,
    pub color: String
}