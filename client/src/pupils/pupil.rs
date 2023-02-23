use yew::prelude::*;
use super::types::PupilProps;

#[function_component(Pupil)]
pub fn pupil(props: &PupilProps) -> Html {
    // TODO fetch pupil data from server using id in props - use_effect_deps


    html!{
        <div>
            <span>{props.id.clone()}</span>
        </div>
    }
}