use super::{pupil::Pupil, types::PupilCreateBoxProps};
use crate::{constant, error, error::*, utils, context::AppContext};
use chrono::{NaiveDate, Utc};
use gloo_net::http::Request;
use std::{rc::Rc};
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlElement, HtmlInputElement};
use yew::prelude::*;

#[function_component(PupilCreateBox)]
pub fn pupil_create_box(props: &PupilCreateBoxProps) -> Html {
    let ctx = use_context::<Rc<AppContext>>().expect("NO CONTEXT IN CREATE BOX");
    let create_box = use_node_ref();
    let input_state = use_state(|| InputState::default());
    let is_expanded = use_state(|| false);

    let reset_callback = {
        let input_state = input_state.clone();
        Callback::from(move |_| {
            let input_state = input_state.clone();
            input_state.set(InputState::default());
        })
    };

    let create_callback = {
        let refresh_callback = props.refresh_callback.clone();
        let input_state = input_state.clone();
        let ctx = ctx.clone();
        Callback::from(move |_| {
            let ctx = ctx.clone();
            let refresh_callback = refresh_callback.clone();
            let name = input_state.name.split(" ").collect::<Vec<&str>>();
            let (last_name, first_names) = name.split_last().expect("returns if name not 2 parts");
            let pupil = Pupil::new(
                first_names.join(" "),
                last_name.to_string(),
                input_state.year,
                input_state.start_date,
                input_state.leave_date,
                input_state.active,
                input_state.mat,
                input_state.eal,
                input_state.fsm,
                input_state.aln,
                input_state.lac,
                input_state.gender.clone(),
            );
            spawn_local(async move {
                match Request::put(constant::PUPILS_PATH)
                    .json(&pupil)
                    .expect("TODO this should be able to convert into our error")
                    .header("Authorization", &format!("Bearer {}", ctx.auth_token))
                    .send()
                    .await
                {
                    Ok(_res) => refresh_callback.emit(()),
                    Err(err) => error!("failed to add pupil", err.to_string()),
                }
            });
            input_state.set(InputState::default());
        })
    };

    let toggle_box_cb = {
        let create_box = create_box.clone();
        let is_expanded = is_expanded.clone();
        Callback::from(move |ev: MouseEvent| {
            let create_box: HtmlElement =
                create_box.cast().expect("cast create_box to htmlelement");
            let ev: HtmlElement = ev.target_unchecked_into();
            let mut classes = Classes::from(create_box.class_name());
            if *is_expanded {
                classes.push("hidden");
                ev.set_text_content(Some("Add new learner"));
                is_expanded.set(false);
            } else {
                classes = classes.into_iter().filter(|cl| cl != "hidden").collect();
                ev.set_text_content(Some("Hide"));
                is_expanded.set(true);
            }
            create_box.set_class_name(&classes.to_string());
        })
    };

    let update_state_cb = {
        let input_state = input_state.clone();
        Callback::from(move |ev: Event| {
            let mut state = (*input_state).clone();
            let target: HtmlInputElement = ev.target_unchecked_into();
            match target.id().as_str() {
                "name" => state.name = target.value(),
                "gender" => state.gender = target.value(),
                "year" => state.year = target.value().parse::<i32>().expect("TODO HANDLE"),
                "start_date" => state.start_date = target.value().parse::<NaiveDate>().expect("TODO HANDLE"),
                "leave_date" => state.leave_date = target.value().parse::<NaiveDate>().expect("TODO HANDLE"),
                "active" => state.active = target.checked(),
                "mat" => state.mat = target.checked(),
                "lac" => state.lac = target.checked(),
                "aln" => state.aln = target.checked(),
                "fsm" => state.fsm = target.checked(),
                "eal" => state.eal = target.checked(),
                _ => panic!("input trying to change non-existent state")
            }
            input_state.set(state);
        })
    };

    html! {
        <>
        <div ref={create_box} class={classes!("hidden", "p-5", "w-full", "h-fit", "justify-start", "flex-col", "space-y-4", "bg-slate-100", "rounded-md")}>
            <span class={classes!("text-3xl")}>{"Add a learner"}</span>
            <input id="name" class={classes!("hover:bg-slate-100", "focus:outline-none", "input")} type="text" placeholder="Names" value={(*input_state).name.clone()} onchange={update_state_cb.clone()}/>
            <div class={classes!("flex", "justify-between")}>
                <input id="gender" class={classes!("hover:bg-slate-100", "focus:outline-none", "w-24", "my-2")} type="text" placeholder="Gender" value={(*input_state).gender.clone()} onchange={update_state_cb.clone()}/>
                <input id="year" class={classes!("hover:bg-slate-100", "focus:outline-none", "w-16", "my-2")} type="number" placeholder="Year" value={(*input_state).year.to_string()} onchange={update_state_cb.clone()}/>
            </div>
            <div class={classes!("flex", "justify-between", "items-center", "hover:bg-slate-200")}>
                <label><span>{"Start date"}</span></label>
                <input id="start_date" class={classes!("hover:bg-slate-100", "focus:outline-none", "w-36", "my-2")} type={"date"} placeholder="Start date" value={(*input_state).start_date.to_string()} onchange={update_state_cb.clone()}/>
            </div>

            <div class={classes!("flex", "justify-between", "items-center", "hover:bg-slate-200")}>
                <label><span>{"Leave date"}</span></label>
                <input id="leave_date" class={classes!("hover:bg-slate-100", "focus:outline-none", "w-36", "my-2")} type={"date"} placeholder="Leave date" value={(*input_state).leave_date.to_string()} onchange={update_state_cb.clone()}/>
            </div>

            <div class={classes!("flex", "flex-col", "space-y-4", "my-4")}>
                <div class={classes!("flex", "justify-between", "items-center", "hover:bg-slate-200")}>
                    <label for="active"><span>{"active"}</span></label>
                    <input id="active" type="checkbox" checked={(*input_state).active} onchange={update_state_cb.clone()} />
                </div>
                <div class={classes!("flex", "justify-between", "items-center", "hover:bg-slate-200")}>
                    <label for="mat"><span>{"More able and talented"}</span></label>
                    <input id="mat" type="checkbox" checked={(*input_state).mat} onchange={update_state_cb.clone()}/>
                </div>
                <div class={classes!("flex", "justify-between", "items-center", "hover:bg-slate-200")}>
                    <label for="lac"><span>{"Looked after child"}</span></label>
                    <input id="lac" type="checkbox" checked={(*input_state).lac} onchange={update_state_cb.clone()}/>
                </div>
                <div class={classes!("flex", "justify-between", "items-center", "hover:bg-slate-200")}>
                    <label for="eal"><span>{"English as additional language"}</span></label>
                    <input id="eal" type="checkbox" checked={(*input_state).eal} onchange={update_state_cb.clone()}/>
                </div>
                <div class={classes!("flex", "justify-between", "items-center", "hover:bg-slate-200")}>
                    <label for="aln"><span>{"Additional learning needs"}</span></label>
                    <input id="aln" type="checkbox" checked={(*input_state).aln} onchange={update_state_cb.clone()}/>
                </div>
                <div class={classes!("flex", "justify-between", "items-center", "hover:bg-slate-200")}>
                    <label for="fsm"><span>{"Free school meals"}</span></label>
                    <input id="fsm" type="checkbox" checked={(*input_state).fsm} onchange={update_state_cb.clone()}/>
                </div>

            </div>

            <div class={classes!("flex")}>
                <button class={classes!("bg-red-100", "grow", "hover:bg-red-200")} onclick={reset_callback}>{"Reset"}</button>
                <button class={classes!("bg-green-100", "grow", "hover:bg-green-200")} onclick={create_callback}>{"Add learner"}</button>
            </div>
        </div>

        <button class={classes!("block")} onclick={toggle_box_cb} >{"Add a learner"}</button>
        </>
    }
}

#[derive(Clone)]
struct InputState {
    name: String,
    gender: String,
    start_date: NaiveDate,
    leave_date: NaiveDate,
    active: bool,
    mat: bool,
    lac: bool,
    fsm: bool,
    eal: bool,
    aln: bool,
    year: i32,
}

impl Default for InputState {
    fn default() -> Self {
        let today = Utc::now().date_naive();
        Self {
            name: Default::default(),
            gender: Default::default(),
            start_date: today,
            leave_date: today,
            active: true,
            mat: Default::default(),
            lac: Default::default(),
            fsm: Default::default(),
            eal: Default::default(),
            aln: Default::default(),
            year: Default::default(),
        }
    }
}
