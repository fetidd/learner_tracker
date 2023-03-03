use yew::prelude::*;

#[function_component(EditableField)]
pub fn editable_field(props: &EditableFieldProps) -> Html {
    let edit_mode = props.edit_mode;
    let value = props.value.to_owned();
    let onchange = props.onchange.clone();
    let class = if let Some(class) = &props.class {
        class.to_owned()
    } else {
        "".into()
    };
    if edit_mode {
        html!(<input {class} id={props.id.to_owned()} {value} type={props.input_type.to_owned()} {onchange}/>)
    } else {
        html!(<span {class}>{format!("{}", value)}</span>)
    }
}

#[derive(PartialEq, Properties)]
pub struct EditableFieldProps {
    pub edit_mode: bool,
    pub value: String,
    pub onchange: Callback<Event>,
    pub id: String,
    pub input_type: String,
    pub class: Option<String>
}