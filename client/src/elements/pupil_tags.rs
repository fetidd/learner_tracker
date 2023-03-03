use yew::prelude::*;

use crate::{elements::Tag, pupils::PupilInputState};

#[function_component(PupilTags)]
pub fn pupil_tags(props: &PupilTagsProps) -> Html {
    let class = match &props.edit_mode {
        true => "flex space-x-1 cursor-pointer",
        false => "flex space-x-1",
    };
    let edit_mode = &props.edit_mode;
    let onchange = props.onchange.clone();
    let onclick = {
        clone!(edit_mode, onchange);
        Callback::from(move |ev: MouseEvent| {
            if edit_mode {
                onchange.emit(ev.into());
            }
        })
    };

    html! {
        <div {class}>
            <Tag edit_mode={&props.edit_mode} id="mat" active={props.state.mat} color="purple" text="MAT" onclick={&onclick}/>
            <Tag edit_mode={&props.edit_mode} id="eal" active={props.state.eal} color="yellow" text="EAL" onclick={&onclick}/>
            <Tag edit_mode={&props.edit_mode} id="aln" active={props.state.aln} color="orange" text="ALN" onclick={&onclick}/>
            <Tag edit_mode={&props.edit_mode} id="fsm" active={props.state.fsm} color="green" text="FSM" onclick={&onclick}/>
            <Tag edit_mode={&props.edit_mode} id="lac" active={props.state.lac} color="blue" text="LAC" onclick={&onclick}/>
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct PupilTagsProps {
    pub state: PupilInputState,
    pub edit_mode: bool,
    pub onchange: Callback<Event>,
}
