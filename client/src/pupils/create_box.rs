use super::{pupil::Pupil, types::PupilCreateBoxProps};
use crate::{constant, error, error::*, utils};
use chrono::{NaiveDate, Utc};
use gloo_net::http::Request;
use std::{collections::HashMap, cell::RefCell, rc::Rc};
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlElement, HtmlInputElement, HtmlBaseElement, Element};
use yew::prelude::*;

pub struct PupilCreateBox {
    refs: HashMap<&'static str, NodeRef>,
    expanded: Rc<RefCell<bool>>,
}

impl Component for PupilCreateBox {
    type Message = ();
    type Properties = PupilCreateBoxProps;

    fn create(_ctx: &Context<Self>) -> Self {
        let refs: HashMap<&str, NodeRef> = vec![
            "name",
            "gender",
            "year",
            "start_date",
            "leave_date",
            "active",
            "mat",
            "aln",
            "eal",
            "lac",
            "fsm",
            "create_box",
            "expand_btn",
        ]
        .iter()
        .map(|f| (*f, NodeRef::default()))
        .collect();
        Self {
            refs,
            expanded: Rc::new(RefCell::new(false)),
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        let refs = self.refs.clone();
        reset_form(refs);
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let reset_callback = {
            let refs = self.refs.clone();
            Callback::from(move |_| {
                let refs = refs.clone();
                reset_form(refs);
            })
        };

        let create_callback = {
            let refs = self.refs.clone();
            let refresh_callback = ctx.props().refresh_callback.clone();
            Callback::from(move |_| {
                let refs = refs.clone();
                if let Err(err) = create_pupil(refresh_callback.clone(), refs) {
                    error!("ERROR CREATING PUPIL: ", err.to_string());
                }
            })
        };

        let toggle_box_cb = {
            let refs = self.refs.clone();
            let expanded = self.expanded.clone();
            Callback::from(move |ev: MouseEvent| {
                let create_box: HtmlElement = refs["create_box"].cast().unwrap();
                let ev: HtmlElement = ev.target_unchecked_into();
                let mut expanded = expanded.as_ref().borrow_mut();
                let mut classes = Classes::from(create_box.class_name());
                if *expanded {
                    classes.push("hidden");
                    ev.set_text_content(Some("Add new learner"));
                    *expanded = false;
                } else {
                    classes = classes.into_iter().filter(|cl| cl != "hidden").collect();
                    ev.set_text_content(Some("Hide"));
                    *expanded = true;
                }
                create_box.set_class_name(&classes.to_string());
            })
        };

        html! {
            <>
            <div ref={self.refs["create_box"].clone()} class={classes!("hidden", "md:flex", "p-5", "w-96", "h-fit", "justify-start", "flex-col", "space-y-4", "bg-slate-100", "rounded-md")}>
                <span class={classes!("text-3xl")}>{"Add a learner"}</span>
                <input class={classes!("hover:bg-slate-100", "focus:outline-none", "input")} type={"text"} placeholder={"Names"}  ref={self.refs["name"].clone()}/>
                <div class={classes!("flex", "justify-between")}>
                    <input class={classes!("hover:bg-slate-100", "focus:outline-none", "w-24", "my-2")} type={"text"} placeholder={"Gender"}  ref={self.refs["gender"].clone()}/>
                    <input class={classes!("hover:bg-slate-100", "focus:outline-none", "w-16", "my-2")} type={"number"} placeholder={"Year"}  ref={self.refs["year"].clone()}/>
                </div>
                <div class={classes!("flex", "justify-between", "items-center", "hover:bg-slate-200")}>
                    <label><span>{"Start date"}</span></label>
                    <input class={classes!("hover:bg-slate-100", "focus:outline-none", "w-36", "my-2")} type={"date"} placeholder={"Start date"}  ref={self.refs["start_date"].clone()}/>
                </div>

                <div class={classes!("flex", "justify-between", "items-center", "hover:bg-slate-200")}>
                    <label><span>{"Leave date"}</span></label>
                    <input class={classes!("hover:bg-slate-100", "focus:outline-none", "w-36", "my-2")} type={"date"} placeholder={"Leave date"}  ref={self.refs["leave_date"].clone()}/>
                </div>

                <div class={classes!("flex", "flex-col", "space-y-4", "my-4")}>
                    <div class={classes!("flex", "justify-between", "items-center", "hover:bg-slate-200")}>
                        <label for={"active"}><span>{"Active"}</span></label>
                        <input id={"active"} type={"checkbox"} ref={self.refs["active"].clone()} checked={true} />
                    </div>
                    <div class={classes!("flex", "justify-between", "items-center", "hover:bg-slate-200")}>
                        <label for={"mat"}><span>{"More able and talented"}</span></label>
                        <input id={"mat"} type={"checkbox"} ref={self.refs["mat"].clone()}/>
                    </div>
                    <div class={classes!("flex", "justify-between", "items-center", "hover:bg-slate-200")}>
                        <label for={"lac"}><span>{"Looked after child"}</span></label>
                        <input id={"lac"} type={"checkbox"} ref={self.refs["lac"].clone()}/>
                    </div>
                    <div class={classes!("flex", "justify-between", "items-center", "hover:bg-slate-200")}>
                        <label for={"eal"}><span>{"English as additional language"}</span></label>
                        <input id={"eal"} type={"checkbox"} ref={self.refs["eal"].clone()}/>
                    </div>
                    <div class={classes!("flex", "justify-between", "items-center", "hover:bg-slate-200")}>
                        <label for={"aln"}><span>{"Additional learning needs"}</span></label>
                        <input id={"aln"} type={"checkbox"} ref={self.refs["aln"].clone()}/>
                    </div>
                    <div class={classes!("flex", "justify-between", "items-center", "hover:bg-slate-200")}>
                        <label for={"fsm"}><span>{"Free school meals"}</span></label>
                        <input id={"fsm"} type={"checkbox"} ref={self.refs["fsm"].clone()}/>
                    </div>

                </div>

                <div class={classes!("flex")}>
                    <button class={classes!("bg-red-100", "grow", "hover:bg-red-200")} onclick={reset_callback}>{"Reset"}</button>
                    <button class={classes!("bg-green-100", "grow", "hover:bg-green-200")} onclick={create_callback}>{"Add learner"}</button>
                </div>
            </div>

            <button class={classes!("block", "md:hidden")} onclick={toggle_box_cb} >{"Add a learner"}</button>
            </>
        }
    }
}

fn reset_form(refs: HashMap<&str, NodeRef>) {
    for (field, n_ref) in refs {
        match field {
            "name" | "gender" | "year" => n_ref
                .cast::<HtmlInputElement>()
                .expect("cast input ref")
                .set_value(""),
            "start_date" | "leave_date" => n_ref
                .cast::<HtmlInputElement>()
                .expect("cast input ref")
                .set_value(&Utc::now().naive_utc().date().to_string()),
            "mat" | "aln" | "fsm" | "lac" | "eal" => n_ref
                .cast::<HtmlInputElement>()
                .expect("cast input ref")
                .set_checked(false),
            "active" => n_ref
                .cast::<HtmlInputElement>()
                .expect("cast input ref")
                .set_checked(true),
            _ => (),
        }
    }
}

fn create_pupil(callback: Callback<()>, refs: HashMap<&str, NodeRef>) -> Result<()> {
    let refs: HashMap<&str, HtmlInputElement> = refs
        .into_iter()
        .map(|(field, n_ref)| {
            (
                field,
                n_ref
                    .cast::<HtmlInputElement>()
                    .expect("cast input ref in create_pupil"),
            )
        })
        .collect();
    let name = refs["name"].value();
    let name = name.split(" ").collect::<Vec<&str>>();
    if name.len() < 2 {
        return Err(ValueError!("must have first name and last name"));
    }
    let (last_name, first_names) = name.split_last().expect("returns if name not 2 parts");
    let year = refs["year"].value().parse::<i32>()?;
    let start_date = refs["start_date"].value().parse::<NaiveDate>()?;
    let leave_date = refs["leave_date"].value().parse::<NaiveDate>()?;
    let pupil = Pupil::new(
        first_names.join(" "),
        last_name.to_string(),
        year,
        start_date,
        leave_date,
        refs["active"].checked(),
        refs["mat"].checked(),
        refs["lac"].checked(),
        refs["aln"].checked(),
        refs["fsm"].checked(),
        refs["eal"].checked(),
        refs["gender"].value(),
    );
    let token = utils::get_current_token();
    spawn_local(async move {
        match Request::put(constant::PUPILS_PATH)
            .json(&pupil)
            .expect("TODO this should be able to convert into our error")
            .header("Authorization", &format!("Bearer {}", token))
            .send()
            .await
        {
            Ok(_res) => callback.emit(()),
            Err(err) => error!("failed to add pupil", err.to_string()),
        }
    });
    Ok(())
}
