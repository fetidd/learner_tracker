use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use crate::{constant, utils, error::*};

use super::{types::PupilDetailsProps, pupil::Pupil};

#[function_component(PupilDetails)]
pub fn pupil_details(props: &PupilDetailsProps) -> Html {
    let pupil: UseStateHandle<Option<Pupil>> = use_state(|| None);
    {
        let pupil = pupil.clone();
        let id = props.id.clone();
        use_effect_with_deps(move |_| {
            spawn_local(async move {
                if let Ok(fetched) = fetch_pupil_data(&id).await {
                    pupil.set(Some(fetched));
                }
            });
        },());
    }
    if let Some(pupil) = pupil.as_ref() {
        html!{
            <div class={classes!("container", "mx-auto", "flex", "w-full", "justify-around")}>
                <div class={classes!()}>
                    <table>
                        <tr>
                            <td>{"Name"}</td>
                            <td>{format!("{} {}", pupil.first_names, pupil.last_name)}</td>
                        </tr>
                        <tr>
                            <td>{"Year"}</td>
                            <td>{format!("{}", pupil.year)}</td>
                        </tr>
                        <tr>
                            <td>{"Gender"}</td>
                            <td>{format!("{}", pupil.gender)}</td>
                        </tr>
                        <tr>
                            <td>{"Start date"}</td>
                            <td>{format!("{}", pupil.start_date)}</td>
                        </tr>
                        <tr>
                            <td>{"Leave date"}</td>
                            <td>{format!("{}", pupil.end_date)}</td>
                        </tr>
                    </table>
                </div>
                <div class={classes!("flex", "flex-col", "space-x-1")}>
                    if pupil.more_able_and_talented {
                        <span class={classes!("bg-purple-200", "tag")}>{"More able and talented"}</span>
                    }
                    if pupil.english_as_additional_language {
                        <span class={classes!("bg-yellow-200", "tag")}>{"English as an additional language"}</span>
                    }
                    if pupil.additional_learning_needs {
                        <span class={classes!("bg-orange-200", "tag")}>{"Additional learning needs"}</span>
                    }
                    if pupil.free_school_meals {
                        <span class={classes!("bg-green-200", "tag")}>{"Free school meals"}</span>
                    }
                    if pupil.looked_after_child {
                        <span class={classes!("bg-blue-200", "tag")}>{"Looked after child"}</span>
                    }
                </div>
            </div>
        }
    } else {
        html! {}
    }
}

async fn fetch_pupil_data(id: &str) -> Result<Pupil> {
    let token = utils::get_current_token();
    if token.is_err() {
        return Err(StorageError!("failed to get token"));
    }
    match Request::get(&format!("{}/{}", constant::PUPILS_PATH, id)).header("Authorization", &format!("Bearer {}", &token.unwrap())).send().await {
        Ok(res) => {
            let pupil = res.json::<Pupil>().await?.into();
            Ok(pupil)
        },
        Err(err) => Err(ResponseParseError!(err.to_string()))
    }
}
