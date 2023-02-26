use super::types::PupilRowProps;
use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(PupilRow)]
pub fn pupil_row(p: &PupilRowProps) -> Html {
    html! { if p.pupil.active {
        <div key={p.pupil.id.expect("pupil should always have id here").to_string()} class={classes!("h-[42px]", "hover:bg-slate-100")}>
        <Link<Route> to={Route::Pupil {id: p.pupil.id.expect("pupil should always have id here").to_string()}}>
            <div class={classes!("align-middle", "h-[40px]")}><span class={classes!()}>{format!("{} {}", p.pupil.first_names, p.pupil.last_name)}</span></div>
        </Link<Route>>
        <div class={classes!("text-center", "hidden", "lg:inline")}>{format!("{}", p.pupil.start_date)}</div>
        <div class={classes!("text-center", "hidden", "lg:inline")}>{format!("{}", p.pupil.end_date)}</div>
        <div class={classes!("text-center", "hidden", "lg:inline")}>{format!("{}", p.pupil.gender)}</div>
        <div class={classes!("hidden", "lg:flex", "justify-start", "items-center", "space-x-1")}>
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
        </div>
        </div>
    }}
}
