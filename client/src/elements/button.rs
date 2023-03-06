use yew::prelude::*;

static GREEN: &str = "hover:bg-green-100 bg-green-200 border-green-200";
static RED: &str = "hover:bg-red-100 bg-red-200 border-red-200";
static BLUE: &str = "hover:bg-blue-100 bg-blue-200 border-blue-200";
static YELLOW: &str = "hover:bg-yellow-100 bg-yellow-200 border-yellow-200";
static PURPLE: &str = "hover:bg-purple-100 bg-purple-200 border-purple-200";

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let visible = {
        if let Some(vis) = props.visible {
            if vis {
                ""
            } else {
                "hidden"
            }
        } else {
            ""
        }
    };

    let class = format!(
        "p-0.5 m-0.5 rounded w-[120px] border-solid border-2 flex items-center justify-center gap-6 {} {}",
        match props.color.as_str() {
            "green" => GREEN,
            "red" => RED,
            "blue" => BLUE,
            "yellow" => YELLOW,
            "purple" => PURPLE,
            _ => GREEN,
        },
        visible
    );
    let icon = match &props.icon {
        Some(icon_html) => icon_html.clone(),
        None => html!()
    };

    html!(<button class={class} onclick={props.onclick.clone()} >{icon}<span>{&props.text}</span></button>)
}

#[derive(PartialEq, Properties)]
pub struct ButtonProps {
    pub onclick: Callback<MouseEvent>,
    pub text: String,
    pub color: String,
    pub visible: Option<bool>,
    pub icon: Option<Html>
}
