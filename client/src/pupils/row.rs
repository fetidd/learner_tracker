use super::types::PupilRowProps;
use yew::prelude::*;

#[function_component(PupilRow)]
pub fn pupil_row(p: &PupilRowProps) -> Html {
    html! { if p.pupil.active {
        <tr key={p.pupil.id.to_string()} class={classes!()}>
        <td class={classes!()}>{format!("{} {}", p.pupil.first_names, p.pupil.last_name)}</td>
        <td class={classes!("text-center")}>{format!("{}", p.pupil.start_date)}</td>
        <td class={classes!("text-center")}>{format!("{}", p.pupil.end_date)}</td>
        <td class={classes!("text-center")}>{format!("{}", p.pupil.gender)}</td>
        <td class={classes!("flex", "justify-between")}>
            if p.pupil.more_able_and_talented {
                <span class={classes!("bg-purple-200")}>{"MAT"}</span>
            }
            if p.pupil.english_as_additional_language {
                <span class={classes!("bg-yellow-200")}>{"EAL"}</span>
            }
            if p.pupil.additional_learning_needs {
                <span class={classes!("bg-orange-200")}>{"ALN"}</span>
            }
            if p.pupil.free_school_meals {
                <span class={classes!("bg-green-200")}>{"FSM"}</span>
            }
            if p.pupil.looked_after_child {
                <span class={classes!("bg-blue-200")}>{"LAC"}</span>
            }
        </td>
        </tr>
    }}
}
