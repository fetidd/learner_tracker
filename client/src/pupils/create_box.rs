use super::pupil::Pupil;
use crate::{
    app::AppContext,
    clone, constant,
    elements::{Button, IconButton, PupilTags, EditableField},
    error,
    error::Result,
    pupils::PupilInputState,
};
use gloo_net::http::Request;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
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
            <EditableField class="hover:bg-slate-100 focus:outline-none w-36 my-2" id="name" input_type="text" edit_mode={true} value={(*input_state).name.to_string()} onchange={&update_state_cb}/>
            <div class="flex justify-between">
                <EditableField class="hover:bg-slate-100 focus:outline-none w-36 my-2" id="gender" input_type="text" edit_mode={true} value={(*input_state).gender.to_string()} onchange={&update_state_cb}/>
                <EditableField class="hover:bg-slate-100 focus:outline-none w-36 my-2" id="year" input_type="number" edit_mode={true} value={(*input_state).year.to_string()} onchange={&update_state_cb}/>
            </div>
            <div class="flex justify-between items-center hover:bg-slate-200">
                <label><span>{"Start date"}</span></label>
                <EditableField class="hover:bg-slate-100 focus:outline-none w-36 my-2" id="start_date" input_type="date" edit_mode={true} value={(*input_state).start_date.to_string()} onchange={&update_state_cb}/>
            </div>
            <div class="flex justify-between items-center hover:bg-slate-200">
                <label for="active"><span>{"Active?"}</span></label>
                <input type="checkbox" id="active" checked={(*input_state).active} onchange={&update_state_cb}/>
            </div>
            {if !(*input_state).active {
                html!{
                    <div class="flex justify-between items-center hover:bg-slate-200">
                        <label><span>{"Leave date"}</span></label>
                        <EditableField class="hover:bg-slate-100 focus:outline-none w-36 my-2" id="leave_date" input_type="date" edit_mode={true} value={(*input_state).leave_date.expect("inactive pupil should have leave date").to_string()} onchange={&update_state_cb}/>
                    </div>
                }
            } else {
                html!()
            }}
            <div class="my-3">
                <PupilTags state={(*input_state).clone()} edit_mode=true onchange={&update_state_cb}/>
            </div>

            <div class="flex justify-between">
                <Button color="red" onclick={reset_callback} text="Reset"/>
                <Button color="green" onclick={create_callback} text="Add learner"/>
            </div>
        </div>
    }
}

async fn create_pupil(
    pupil: &Pupil,
    ctx: &AppContext,
    refresh_callback: Callback<()>,
) -> Result<()> {
    Request::put(constant::PUPILS_PATH)
        .json(&pupil)
        .expect("TODO this should be able to convert into our error")
        .header("Authorization", &format!("Bearer {}", ctx.auth_token))
        .send()
        .await?;
    refresh_callback.emit(());
    Ok(())
}
