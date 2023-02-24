use chrono::{Utc, NaiveDate};
use gloo_net::http::{Response, Request};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::{scope_ext::RouterScopeExt, prelude::Navigator};

use crate::{constant, utils, error, debug, routes::Route};

use super::{pupil::Pupil, types::PupilCreateBoxProps};

macro_rules! clone_batch {
    ($($to_clone:ident),+) => {
        $(let $to_clone= $to_clone.clone();)*
    };
    ($obj:ident: $($to_clone:ident),+) => {
        $(let $to_clone= $obj.$to_clone.clone();)*
    }
}

pub struct PupilCreateBox {
    name: NodeRef,
    gender: NodeRef,
    year: NodeRef,
    start_date: NodeRef,
    leave_date: NodeRef,
    active: NodeRef,
    mat: NodeRef,
    aln: NodeRef,
    eal: NodeRef,
    lac: NodeRef,
    fsm: NodeRef,
}

impl Component for PupilCreateBox {
    type Message = ();
    type Properties = PupilCreateBoxProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            name: NodeRef::default(),
            gender: NodeRef::default(),
            year: NodeRef::default(),
            start_date: NodeRef::default(),
            leave_date: NodeRef::default(),
            active: NodeRef::default(),
            mat: NodeRef::default(),
            aln: NodeRef::default(),
            eal: NodeRef::default(),
            lac: NodeRef::default(),
            fsm: NodeRef::default(),
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        clone_batch!(self: name,year,gender,start_date,leave_date,active,mat,lac,aln,fsm,eal);
        reset_form(name,year,gender,start_date,leave_date,active,mat,lac,aln,fsm,eal);
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let reset_callback = {
            clone_batch!(self: name,year,gender,start_date,leave_date,active,mat,lac,aln,fsm,eal);
            Callback::from(move |_| {
                clone_batch!(name,year,gender,start_date,leave_date,active,mat,lac,aln,fsm,eal);
                reset_form(name,year,gender,start_date,leave_date,active,mat,lac,aln,fsm,eal);
            })
        };

        let create_callback = {
            clone_batch!(self: name,year,gender,start_date,leave_date,active,mat,lac,aln,fsm,eal);
            let refresh_callback = ctx.props().refresh_callback.clone();
            Callback::from(move |_| {
                clone_batch!(name,year,gender,start_date,leave_date,active,mat,lac,aln,fsm,eal);
                create_pupil(refresh_callback.clone(), name,year,gender,start_date,leave_date,active,mat,lac,aln,fsm,eal);
            })
        };

        html! {
            <div class={classes!("p-5", "w-96", "h-fit", "flex", "justify-start", "flex-col", "space-y-4", "bg-slate-100", "rounded-md")}>
                <span class={classes!("text-3xl")}>{"Add a learner"}</span>
                <input class={classes!("hover:bg-slate-100", "focus:outline-none", "input")} type={"text"} placeholder={"Names"}  ref={self.name.clone()}/>
                <div class={classes!("flex", "justify-between")}>
                    <input class={classes!("hover:bg-slate-100", "focus:outline-none", "w-24", "my-2")} type={"text"} placeholder={"Gender"}  ref={self.gender.clone()}/>
                    <input class={classes!("hover:bg-slate-100", "focus:outline-none", "w-16", "my-2")} type={"number"} placeholder={"Year"}  ref={self.year.clone()}/>
                </div>
                <div class={classes!("flex", "justify-between", "items-center")}>
                    <label><span>{"Start date"}</span></label>
                    <input class={classes!("hover:bg-slate-100", "focus:outline-none", "w-36", "my-2")} type={"date"} placeholder={"Start date"}  ref={self.start_date.clone()}/>
                </div>
    
                <div class={classes!("flex", "justify-between", "items-center")}>
                    <label><span>{"Leave date"}</span></label>
                    <input class={classes!("hover:bg-slate-100", "focus:outline-none", "w-36", "my-2")} type={"date"} placeholder={"Leave date"}  ref={self.leave_date.clone()}/>
                </div>
    
                <div class={classes!("flex", "flex-col", "space-y-4", "my-4")}>
                    <div class={classes!("flex", "justify-between", "items-center")}>
                        <label for={"active"}><span>{"Active"}</span></label>
                        <input id={"active"}type={"checkbox"} ref={self.active.clone()} checked={true} />
                    </div>
                    <div class={classes!("flex", "justify-between", "items-center")}>
                        <label for={"mat"}><span>{"More able and talented"}</span></label>
                        <input id={"mat"}type={"checkbox"} ref={self.mat.clone()}/>
                    </div>
                    <div class={classes!("flex", "justify-between", "items-center")}>
                        <label for={"lac"}><span>{"Looked after child"}</span></label>
                        <input id={"lac"}type={"checkbox"} ref={self.lac.clone()}/>
                    </div>
                    <div class={classes!("flex", "justify-between", "items-center")}>
                        <label for={"eal"}><span>{"English as additional language"}</span></label>
                        <input id={"eal"}type={"checkbox"} ref={self.eal.clone()}/>
                    </div>
                    <div class={classes!("flex", "justify-between", "items-center")}>
                        <label for={"aln"}><span>{"Additional learning needs"}</span></label>
                        <input id={"aln"}type={"checkbox"} ref={self.aln.clone()}/>
                    </div>
                    <div class={classes!("flex", "justify-between", "items-center")}>
                        <label for={"fsm"}><span>{"Free school meals"}</span></label>
                        <input id={"fsm"}type={"checkbox"} ref={self.fsm.clone()}/>
                    </div>
    
                </div>
                
                <div class={classes!("flex")}>
                    <button class={classes!("bg-red-100", "grow")} onclick={reset_callback}>{"Reset"}</button>
                    <button class={classes!("bg-green-100", "grow")} onclick={create_callback}>{"Add learner"}</button>
                </div>
    
            </div>
        }
    }
}

fn reset_form(name: NodeRef,year: NodeRef,gender: NodeRef,start_date: NodeRef,leave_date: NodeRef,active: NodeRef,mat: NodeRef,lac: NodeRef,aln: NodeRef,fsm: NodeRef,eal: NodeRef) {
    name.cast::<HtmlInputElement>().expect("resetting form").set_value("");
    year.cast::<HtmlInputElement>().expect("resetting form").set_value("");
    gender.cast::<HtmlInputElement>().expect("resetting form").set_value("");
    start_date.cast::<HtmlInputElement>().expect("resetting form").set_value(&Utc::now().naive_utc().date().to_string());
    leave_date.cast::<HtmlInputElement>().expect("resetting form").set_value(&Utc::now().naive_utc().date().to_string());
    active.cast::<HtmlInputElement>().expect("resetting form").set_checked(true);
    mat.cast::<HtmlInputElement>().expect("resetting form").set_checked(false);
    lac.cast::<HtmlInputElement>().expect("resetting form").set_checked(false);
    aln.cast::<HtmlInputElement>().expect("resetting form").set_checked(false);
    fsm.cast::<HtmlInputElement>().expect("resetting form").set_checked(false);
    eal.cast::<HtmlInputElement>().expect("resetting form").set_checked(false);
}

fn create_pupil(callback: Callback<()>, name: NodeRef,year: NodeRef,gender: NodeRef,start_date: NodeRef,leave_date: NodeRef,active: NodeRef,mat: NodeRef,lac: NodeRef,aln: NodeRef,fsm: NodeRef,eal: NodeRef) {
    // spin off new thread to post to backend with data from refs
    let name = name.cast::<HtmlInputElement>().expect("adding pupil").value();
    let year = year.cast::<HtmlInputElement>().expect("adding pupil").value().parse::<i32>().unwrap();
    let gender = gender.cast::<HtmlInputElement>().expect("adding pupil").value();
    let start_date = start_date.cast::<HtmlInputElement>().expect("adding pupil").value().parse::<NaiveDate>().unwrap();
    let leave_date = leave_date.cast::<HtmlInputElement>().expect("adding pupil").value().parse::<NaiveDate>().unwrap();
    let active = active.cast::<HtmlInputElement>().expect("adding pupil").checked();
    let mat = mat.cast::<HtmlInputElement>().expect("adding pupil").checked();
    let lac = lac.cast::<HtmlInputElement>().expect("adding pupil").checked();
    let aln = aln.cast::<HtmlInputElement>().expect("adding pupil").checked();
    let fsm = fsm.cast::<HtmlInputElement>().expect("adding pupil").checked();
    let eal = eal.cast::<HtmlInputElement>().expect("adding pupil").checked();
    let name = name.split(" ").collect::<Vec<&str>>();
    let (last_name, first_names) = name.split_last().unwrap();
    let pupil = Pupil::new(first_names.join(" "), last_name.to_string(),year,start_date,leave_date,active,mat,lac,aln,fsm,eal,gender);
    let token = utils::get_current_token();
    spawn_local(async move {
        match Request::put(constant::PUPILS_PATH).json(&pupil).unwrap().header("Authorization", &format!("Bearer {}", token)).send().await {
            Ok(_res) => callback.emit(()),
            Err(err) => error!("failed to add pupil", err.to_string())
        }
    })
}