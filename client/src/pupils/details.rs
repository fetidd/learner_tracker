use yew::prelude::*;
use super::types::PupilDetailsProps;

#[function_component(PupilDetails)]
pub fn pupil_details(props: &PupilDetailsProps) -> Html {
    // TODO fetch pupil data from server using id in props - use_effect_deps


    html!{
        <div>
            <span>{props.id.clone()}</span>
        </div>
    }
}