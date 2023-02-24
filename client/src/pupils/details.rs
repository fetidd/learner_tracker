use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use crate::{constant, utils, error::*};

use super::{types::PupilDetailsProps, pupil::Pupil};

#[function_component(PupilDetails)]
pub fn pupil_details(props: &PupilDetailsProps) -> Html {
    let curr_pupil: UseStateHandle<Option<Pupil>> = use_state(|| None);
    {
        let curr_pupil = curr_pupil.clone();
        let id = props.id.clone();
        use_effect_with_deps(move |_| {
            spawn_local(async move {
                if let Ok(pupil) = fetch_pupil_data(&id).await {
                    curr_pupil.set(Some(pupil));
                }
            });
        },());
    }
    if let Some(pupil) = curr_pupil.as_ref() {
        html!{
            <div class={classes!("flex", "items-center", "space-x-4")}>
                <span class={classes!("text-3xl")}>{format!("{} {}", pupil.first_names.clone(), pupil.last_name.clone())}</span>
                <div class={classes!("flex", "space-x-5" )}>
                    <span class={classes!("text-xl")}>{format!("Year {}", pupil.year.clone())}</span>
                    <span class={classes!("text-md", "text-slate-500")}>{format!("{} -> {}", pupil.start_date.clone(), pupil.end_date.clone())}</span>
                </div>
                <span class={classes!("text-xl")}>{format!("{}", pupil.gender.clone())}</span>
                <div class={classes!("flex", "space-x-1")}>
                    if pupil.more_able_and_talented {
                        <span class={classes!("bg-purple-200", "tag")}>{"MAT"}</span>
                    }
                    if pupil.english_as_additional_language {
                        <span class={classes!("bg-yellow-200", "tag")}>{"EAL"}</span>
                    }
                    if pupil.additional_learning_needs {
                        <span class={classes!("bg-orange-200", "tag")}>{"ALN"}</span>
                    }
                    if pupil.free_school_meals {
                        <span class={classes!("bg-green-200", "tag")}>{"FSM"}</span>
                    }
                    if pupil.looked_after_child {
                        <span class={classes!("bg-blue-200", "tag")}>{"LAC"}</span>
                    }
                </div>
            </div>
        }
    } else {
        html! {
            <span>{"ERROR"}</span>
        }
    }
}

async fn fetch_pupil_data(id: &str) -> Result<Pupil> {
    match Request::get(&format!("{}/{}", constant::PUPILS_PATH, id)).header("Authorization", &format!("Bearer {}", &utils::get_current_token())).send().await {
        Ok(res) => {
            let pupil = res.json::<Pupil>().await?.into();
            Ok(pupil)
        },
        Err(err) => Err(ResponseParseError!(err.to_string()))
    }
}
