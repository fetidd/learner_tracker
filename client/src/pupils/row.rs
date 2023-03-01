use super::types::PupilRowProps;
use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(PupilRow)]
pub fn pupil_row(p: &PupilRowProps) -> Html {
    html! { if p.pupil.active {
        <Link<Route> to={Route::Pupil {id: p.pupil.id.expect("pupil should always have id here").to_string()}}>
        <div key={p.pupil.id.expect("pupil should always have id here").to_string()} class="h-[42px] hover:bg-slate-100 w-full flex justify-between flex-no-wrap">
            <div class="w-[250px]"><span>{format!("{} {}", p.pupil.first_names, p.pupil.last_name)}</span></div>
            // <div class="text-center hidden lg:inline w-[45px]">{format!("{}", p.pupil.year)}</div>
            // <div class="text-center hidden lg:inline w-[45px]">{format!("{}", p.pupil.gender)}</div>
            <div class="hidden lg:flex justify-start items-center space-x-1 w-[170px]">
                if p.pupil.more_able_and_talented {
                    <span class="bg-purple-200 tag text-xs">{"MAT"}</span>
                }
                if p.pupil.english_as_additional_language {
                    <span class="bg-yellow-200 tag text-xs">{"EAL"}</span>
                }
                if p.pupil.additional_learning_needs {
                    <span class="bg-orange-200 tag text-xs">{"ALN"}</span>
                }
                if p.pupil.free_school_meals {
                    <span class="bg-green-200 tag text-xs">{"FSM"}</span>
                }
                if p.pupil.looked_after_child {
                    <span class="bg-blue-200 tag text-xs">{"LAC"}</span>
                }
            </div>
        </div>
        </Link<Route>>
    }}
}
