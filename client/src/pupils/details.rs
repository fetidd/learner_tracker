use super::pupil::Pupil;
use crate::{
    app::AppContext,
    constant,
    elements::{Button, EditableField, IconButton, PupilTags},
    pupils::PupilInputState,
};
use gloo_net::http::Request;
use std::{rc::Rc, str::FromStr};
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(PupilDetails)]
pub fn pupil_details(props: &PupilDetailsProps) -> Html {
    let ctx = use_context::<Rc<AppContext>>().expect("NO CTX IN PUPIL DETAILS");
    let refresh_callback = props.refresh_callback.clone();
    let close_callback = props.close_callback.clone();
    let edit_mode = use_state(|| false);
    let input_state = use_state_eq(|| {
        if let Some(pupil) = &props.pupil {
            PupilInputState::from(pupil)
        } else {
            PupilInputState::default()
        }
    });

    if let Some(pupil) = &props.pupil {
        let update_state_cb = {
            clone!(input_state);
            Callback::from(move |ev: Event| {
                let mut state = (*input_state).clone();
                let target: HtmlInputElement = ev.target_unchecked_into();
                state.update(target);
                input_state.set(state);
            })
        };

        html! {
            <div class="w-[450px] h-[240px] flex flex-col">
                <div class="flex justify-between mb-3">
                    <EditableField id="name" class={Some("text-2xl")} input_type="text" edit_mode={*edit_mode} value={(*input_state).name.to_string()} onchange={&update_state_cb}/>
                    <IconButton onclick={&props.close_callback} icon="close" />
                </div>
                <div class="flex flex-col justify-between h-full">
                    <div class="flex flex-col h-full">
                        <ul class="flex flex-col justify-between h-full">
                            <li class="flex justify-between">
                                <span class="text-bold w-[120px]">{"Year"}</span>
                                <EditableField id="year" input_type="number" edit_mode={*edit_mode} value={(*input_state).year.to_string()} onchange={&update_state_cb}/>
                            </li>
                            <li class="flex justify-between">
                                <span class="text-bold w-[120px]">{"Gender"}</span>
                                <EditableField id="gender" input_type="text" edit_mode={*edit_mode} value={(*input_state).gender.clone()} onchange={&update_state_cb}/>
                            </li>
                            <li class="flex justify-between">
                                <span class="text-bold w-[120px]">{"Start date"}</span>
                                <EditableField id="start_date" input_type="date" edit_mode={*edit_mode} value={(*input_state).start_date.to_string()} onchange={&update_state_cb}/>
                            </li>
                            <li class="flex justify-between">
                                <label for="active"><span>{"Active?"}</span></label>
                                <input type="checkbox" id="active" checked={(*input_state).active} onchange={&update_state_cb} disabled={!(*edit_mode)}/>
                            </li>
                            {if !(*input_state).active {
                                html!{
                                    <li class="flex justify-between">
                                        <span class="text-bold w-[120px]">{"Leave date"}</span>
                                        <EditableField id="leave_date" input_type="date" edit_mode={*edit_mode} value={(*input_state).leave_date.expect("inactive pupil should have leave date").to_string()} onchange={&update_state_cb}/>
                                    </li>
                                }
                            } else {
                                html!()
                            }}
                            <li class="flex justify-between">
                                <span class="text-bold w-[120px]">{"Tags"}</span>
                                <PupilTags state={(*input_state).clone()} edit_mode={*edit_mode} onchange={&update_state_cb}/>
                            </li>
                        </ul>
                    </div>
                    <div class="flex justify-around my-3">
                        <Button visible={Some(*edit_mode)} color="yellow" text="Cancel" onclick={
                            clone!(edit_mode, pupil, input_state);
                            Callback::from(move |_ev| {
                                clone!(edit_mode, pupil, input_state);
                                spawn_local(async move {
                                    edit_mode.set(!*edit_mode);
                                    input_state.set(PupilInputState::from(&pupil.clone()));
                                })
                        })} />
                        <Button visible={Some(*edit_mode)} color="green" text="Save" onclick={
                            clone!(edit_mode, pupil, input_state, refresh_callback, ctx);
                            Callback::from(move |_ev| {
                                clone!(edit_mode, pupil, input_state, refresh_callback, ctx);
                                spawn_local(async move {
                                    update_pupil(&pupil.id.unwrap().to_string(), &(*input_state), &ctx.auth_token).await;
                                    refresh_callback.emit(true);
                                    edit_mode.set(!*edit_mode);
                                })
                        })} />
                        <Button visible={Some(!*edit_mode)} color="yellow" text="Edit" onclick={
                            clone!(edit_mode);
                            Callback::from(move |_ev| {
                                clone!(edit_mode);
                                spawn_local(async move {
                                    edit_mode.set(!*edit_mode);
                                })
                        })} />
                        <Button visible={Some(!*edit_mode)} color="red" text="Delete" onclick={
                            clone!(pupil, refresh_callback, close_callback, ctx);
                            Callback::from(move |ev| {
                                clone!(pupil, refresh_callback, close_callback, ctx);
                                spawn_local(async move {
                                    delete_pupil(&pupil.id.unwrap().to_string(), &ctx.auth_token).await;
                                    refresh_callback.emit(true);
                                    close_callback.emit(ev);
                                })
                        })} />
                    </div>
                </div>
            </div>
        }
    } else {
        html!({ "NO PUPIL" })
    }
}

#[derive(PartialEq, Properties)]
pub struct PupilDetailsProps {
    pub pupil: Option<Pupil>,
    pub close_callback: Callback<MouseEvent>,
    pub refresh_callback: Callback<bool>,
}

async fn delete_pupil(id: &str, token: &str) {
    debug!("deleting", id);
    match Request::delete(&format!("{}/{}", constant::PUPILS_PATH, id))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
    {
        Ok(_res) => {}
        Err(error) => error!("error deleting pupil:", error.to_string()),
    }
}

async fn update_pupil(id: &str, is: &PupilInputState, token: &str) {
    debug!("updating", id);
    let name = is.name.split(" ").collect::<Vec<&str>>();
    let (last_name, first_names) = name.split_last().expect("returns if name not 2 parts");
    let pupil = Pupil {
        id: Some(Uuid::from_str(id).expect("string was not a valid uuid")),
        first_names: first_names.join(" "),
        last_name: last_name.to_string(),
        year: is.year,
        start_date: is.start_date,
        end_date: is.leave_date,
        active: is.active,
        more_able_and_talented: is.mat,
        english_as_additional_language: is.eal,
        free_school_meals: is.fsm,
        additional_learning_needs: is.aln,
        looked_after_child: is.lac,
        gender: is.gender.clone(),
    };
    match Request::post(&format!("{}/{}", constant::PUPILS_PATH, id))
        .json(&pupil)
        .expect("pupil json")
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
    {
        Ok(_res) => {}
        Err(error) => error!("error updating pupil:", error.to_string()),
    }
}
