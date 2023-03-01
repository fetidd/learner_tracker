use yew::prelude::*;

static GREEN: &str =    "bg-green-200";
static RED: &str =      "bg-red-200";
static BLUE: &str =     "bg-blue-200";
static YELLOW: &str =   "bg-yellow-200";
static PURPLE: &str =   "bg-purple-200";
static ORANGE: &str =   "bg-orange-200";

#[function_component(Tag)]
pub fn tag(props: &TagProps) -> Html {
    let color = match props.color.as_str() {
        "green" => GREEN,
        "red" => RED,
        "blue" => BLUE,
        "yellow" => YELLOW,
        "purple" => PURPLE,
        "orange" => ORANGE,
        _ => "bg-white"
    };
    let class = format!("px-1 py-0.5 rounded text-xs {color}");
    html!(<span class={class}>{&props.text}</span>)
}

#[derive(PartialEq, Properties)]
pub struct TagProps {
    pub text: String,
    pub color: String
}