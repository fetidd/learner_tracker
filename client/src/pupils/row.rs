use super::types::PupilRowProps;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::Route;

#[function_component(PupilRow)]
pub fn pupil_row(p: &PupilRowProps) -> Html { // FIXME these fucking rows arent all centered 
    html! { if p.pupil.active {
        <tr key={p.pupil.id.to_string()} class={classes!("h-[42px]")}>
        <Link<Route> to={Route::Pupil {id: p.pupil.id.to_string()}}>
            <td class={classes!("align-middle", "h-[40px]")}><span class={classes!()}>{format!("{} {}", p.pupil.first_names, p.pupil.last_name)}</span></td>
        </Link<Route>>
        <td class={classes!("text-center")}>{format!("{}", p.pupil.start_date)}</td>
        // <td class={classes!("text-center")}>{format!("{}", p.pupil.end_date)}</td>
        // <td class={classes!("text-center")}>{format!("{}", p.pupil.gender)}</td>
        <td class={classes!("flex", "justify-start", "items-center")}>
            if p.pupil.more_able_and_talented {
                <span class={classes!("bg-purple-200", "tag")}>{"MAT"}</span>
            }
            if p.pupil.english_as_additional_language {
                <span class={classes!("bg-yellow-200", "tag")}>{"EAL"}</span>
            }
            if p.pupil.additional_learning_needs {
                <span class={classes!("bg-orange-200", "tag")}>{"ALN"}</span>
            }
            if p.pupil.free_school_meals {
                <span class={classes!("bg-green-200", "tag")}>{"FSM"}</span>
            }
            if p.pupil.looked_after_child {
                <span class={classes!("bg-blue-200", "tag")}>{"LAC"}</span>
            }
        </td>
        </tr>
    }}
}
