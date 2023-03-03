use super::pupil::Pupil;
use crate::elements::Tag;
use yew::prelude::*;

#[function_component(PupilRow)]
pub fn pupil_row(p: &PupilRowProps) -> Html {
    let open_pupil_details_callback = p.open_pupil_details_callback.clone();
    let pupil = p.pupil.clone();
    let open_pupil_details = {
        clone!(pupil);
        Callback::from(move |ev: MouseEvent| {
            open_pupil_details_callback.emit((ev, pupil.clone()));
        })
    };

    let id = pupil
        .id
        .expect("pupil should always have id here")
        .to_string();
    if pupil.active {
        html! { 
            <li key={id.clone()} class="snap-start cursor-pointer break-inside-avoid-column" onclick={open_pupil_details}>
                <div class="h-[42px] hover:bg-slate-100 w-full flex justify-between flex-no-wrap rounded items-center px-2">
                    <span>{format!("{} {}", pupil.first_names, pupil.last_name)}</span>
                    <div class="hidden lg:flex justify-start items-center space-x-1 w-[200px]">
                        if pupil.more_able_and_talented {
                            <Tag id="mat" color="purple" text="MAT" />
                        }
                        if pupil.english_as_additional_language {
                            <Tag id="eal" color="yellow" text="EAL" />
                        }
                        if pupil.additional_learning_needs {
                            <Tag id="aln" color="orange" text="ALN" />
                        }
                        if pupil.free_school_meals {
                            <Tag id="fsm" color="green" text="FSM" />
                        }
                        if pupil.looked_after_child {
                            <Tag id="lac" color="blue" text="LAC" />
                        }
                    </div>
                </div>
            </li>
        }
    } else {
        html!{
            <li key={id.clone()} class="snap-start cursor-pointer break-inside-avoid-column" onclick={open_pupil_details}>
                <div class="h-[42px] flex justify-between flex-no-wrap rounded items-center px-2">
                    <span class="text-slate-200" >{format!("{} {}", pupil.first_names, pupil.last_name)}</span>
                </div>
            </li>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct PupilRowProps {
    pub pupil: Pupil,
    pub open_pupil_details_callback: Callback<(MouseEvent, Pupil)>,
}
