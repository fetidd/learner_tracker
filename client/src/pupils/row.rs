use super::types::PupilRowProps;
use crate::{routes::Route, elements::Tag};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(PupilRow)]
pub fn pupil_row(p: &PupilRowProps) -> Html {
    html! { if p.pupil.active {
        <li key={p.pupil.id.expect("pupil should always have id here").to_string()} class="snap-start">
        <Link<Route> to={Route::Pupil {id: p.pupil.id.expect("pupil should always have id here").to_string()}}>
        <div class="h-[42px] hover:bg-slate-100 w-full flex justify-between flex-no-wrap rounded items-center px-2">
            <span>{format!("{} {}", p.pupil.first_names, p.pupil.last_name)}</span>
            // <div class="text-center hidden lg:inline w-[45px]">{format!("{}", p.pupil.year)}</div>
            // <div class="text-center hidden lg:inline w-[45px]">{format!("{}", p.pupil.gender)}</div>
            <div class="hidden lg:flex justify-start items-center space-x-1 w-[170px]">
                if p.pupil.more_able_and_talented {
                    <Tag color="purple" text="MAT" />
                }
                if p.pupil.english_as_additional_language {
                    <Tag color="yellow" text="EAL" />
                }
                if p.pupil.additional_learning_needs {
                    <Tag color="orange" text="ALN" />
                }
                if p.pupil.free_school_meals {
                    <Tag color="green" text="FSM" />
                }
                if p.pupil.looked_after_child {
                    <Tag color="blue" text="LAC" />
                }
            </div>
        </div>
        </Link<Route>>
        </li>
    }}
}
