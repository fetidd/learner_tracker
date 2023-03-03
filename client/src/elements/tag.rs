use yew::prelude::*;

static GREEN: &str =    "bg-green-200";
static RED: &str =      "bg-red-200";
static BLUE: &str =     "bg-blue-200";
static YELLOW: &str =   "bg-yellow-200";
static PURPLE: &str =   "bg-purple-200";
static ORANGE: &str =   "bg-orange-200";
static GRAY: &str =     "bg-slate-100 text-slate-400";

#[function_component(Tag)]
pub fn tag(props: &TagProps) -> Html {
    let mut color = match props.color.as_str() {
        "green" => GREEN,
        "red" => RED,
        "blue" => BLUE,
        "yellow" => YELLOW,
        "purple" => PURPLE,
        "orange" => ORANGE,
        _ => "bg-white"
    };
    let mut active = "true";

    if let Some(is_active) = &props.active {
        if !is_active {
            color = GRAY;
            active = "false";
        }
    }

    let class = format!("px-1 py-0.5 rounded text-xs {color}");

    if let Some(onclick) = &props.onclick {
        html!{
            <span id={props.id.to_owned()} class={class} {onclick} {active}>
                {&props.text}
            </span>
        }
    } else {
        html!{
            <span id={props.id.to_owned()} class={class}>
                {&props.text}
            </span>
        }
    }
    
}

#[derive(PartialEq, Properties)]
pub struct TagProps {
    pub text: String,
    pub color: String,
    pub active: Option<bool>,
    pub id: String,
    pub edit_mode: Option<bool>,
    pub onclick: Option<Callback<MouseEvent>>
}