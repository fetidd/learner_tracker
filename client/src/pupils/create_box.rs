use super::{pupil::Pupil};
use crate::{clone, constant, error, app::AppContext, elements::{Button, IconButton, PupilTags}, pupils::PupilInputState, error::Result};
use gloo_net::http::Request;
use std::{rc::Rc};
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct PupilCreateBoxProps {
    pub refresh_callback: Callback<()>,
    pub close_callback: Callback<MouseEvent>,
}

#[function_component(PupilCreateBox)]
pub fn pupil_create_box(props: &PupilCreateBoxProps) -> Html {
    let ctx = use_context::<Rc<AppContext>>().expect("NO CONTEXT IN CREATE BOX");
    let input_state = use_state(|| PupilInputState::default());

    let reset_callback = {
        clone!(input_state);
        Callback::from(move |_| {
            clone!(input_state);
            input_state.set(PupilInputState::default());
        })
    };

    let create_callback = {
        let refresh_callback = props.refresh_callback.clone();
        clone!(input_state, ctx);
        Callback::from(move |_| {
            clone!(input_state, ctx, refresh_callback);
            let pupil = Pupil::from(&(*input_state));
            spawn_local(async move {
                if let Err(error) = create_pupil(&pupil, ctx.as_ref(), refresh_callback).await {
                    error!("failed to create a new pupil:", error.to_string());
                }
            });
            input_state.set(PupilInputState::default());
        })
    };

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
        <div class="">
            <div class="flex justify-between">
                <span class="text-3xl">{"Add a learner"}</span>
                <IconButton icon="close" onclick={props.close_callback.clone()}/>
            </div>
            <input id="name" class="hover:bg-slate-100 focus:outline-none input" type="text" placeholder="Names" value={(*input_state).name.clone()} onchange={update_state_cb.clone()}/>
            <div class="flex justify-between">
                <input id="gender" class="hover:bg-slate-100 focus:outline-none w-24 my-2" type="text" placeholder="Gender" value={(*input_state).gender.clone()} onchange={update_state_cb.clone()}/>
                <input id="year" class="hover:bg-slate-100 focus:outline-none w-16 my-2" type="number" placeholder="Year" value={(*input_state).year.to_string()} onchange={update_state_cb.clone()}/>
            </div>
            <div class="flex justify-between items-center hover:bg-slate-200">
                <label><span>{"Start date"}</span></label>
                <input id="start_date" class="hover:bg-slate-100 focus:outline-none w-36 my-2" type={"date"} placeholder="Start date" value={(*input_state).start_date.to_string()} onchange={update_state_cb.clone()}/>
            </div>

            <div class="flex justify-between items-center hover:bg-slate-200">
                <label><span>{"Leave date"}</span></label>
                <input id="leave_date" class="hover:bg-slate-100 focus:outline-none w-36 my-2" type={"date"} placeholder="Leave date" value={(*input_state).leave_date.to_string()} onchange={update_state_cb.clone()}/>
            </div>

            <PupilTags state={(*input_state).clone()} edit_mode=true onchange={&update_state_cb}/>

            <div class="flex justify-between">
                <Button color="red" onclick={reset_callback} text="Reset"/>
                <Button color="green" onclick={create_callback} text="Add learner"/>
            </div>
        </div>
    }
}

async fn create_pupil(pupil: &Pupil, ctx: &AppContext, refresh_callback: Callback<()>) -> Result<()> {
    Request::put(constant::PUPILS_PATH)
        .json(&pupil)
        .expect("TODO this should be able to convert into our error")
        .header("Authorization", &format!("Bearer {}", ctx.auth_token))
        .send()
        .await?;
    refresh_callback.emit(());
    Ok(())
}
