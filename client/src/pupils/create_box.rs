use chrono::Utc;
use web_sys::HtmlInputElement;
use yew::prelude::*;

macro_rules! clone_batch {
    ($($to_clone:ident),+) => {
        $(let $to_clone = $to_clone.clone();)*
    }
}

#[function_component(PupilCreateBox)]
pub fn pupil_create_box() -> Html {
    let new_name = use_node_ref();
    let new_year = use_node_ref();
    let new_gender = use_node_ref();
    let new_start_date = use_node_ref();
    let new_leave_date = use_node_ref();
    let new_active = use_node_ref();
    let new_mat = use_node_ref();
    let new_lac = use_node_ref();
    let new_aln = use_node_ref();
    let new_fsm = use_node_ref();
    let new_eal = use_node_ref();

    

    // TODO can this management of noderefs be neater?
    let clear_callback = {
        clone_batch!(new_name,new_year,new_gender,new_start_date,new_leave_date,new_active,new_mat,new_lac,new_aln,new_fsm,new_eal);
        Callback::from(move |_| {
            new_name.cast::<HtmlInputElement>().unwrap().set_value("");
            new_year.cast::<HtmlInputElement>().unwrap().set_value("");
            new_gender.cast::<HtmlInputElement>().unwrap().set_value("");
            new_start_date.cast::<HtmlInputElement>().unwrap().set_value(&Utc::now().naive_utc().date().to_string());
            new_leave_date.cast::<HtmlInputElement>().unwrap().set_value(&Utc::now().naive_utc().date().to_string());
            new_active.cast::<HtmlInputElement>().unwrap().set_checked(true);
            new_mat.cast::<HtmlInputElement>().unwrap().set_checked(false);
            new_lac.cast::<HtmlInputElement>().unwrap().set_checked(false);
            new_aln.cast::<HtmlInputElement>().unwrap().set_checked(false);
            new_fsm.cast::<HtmlInputElement>().unwrap().set_checked(false);
            new_eal.cast::<HtmlInputElement>().unwrap().set_checked(false);
        })
    };

    // TODO defaults for start date (today) and leave date (use years to calculate)
    {
        clone_batch!(new_name,new_year,new_gender,new_start_date,new_leave_date,new_active,new_mat,new_lac,new_aln,new_fsm,new_eal);
        use_effect_with_deps(move |_| {
            new_name.cast::<HtmlInputElement>().unwrap().set_value("");
            new_year.cast::<HtmlInputElement>().unwrap().set_value("");
            new_gender.cast::<HtmlInputElement>().unwrap().set_value("");
            new_start_date.cast::<HtmlInputElement>().unwrap().set_value(&Utc::now().naive_utc().date().to_string());
            new_leave_date.cast::<HtmlInputElement>().unwrap().set_value(&Utc::now().naive_utc().date().to_string());
            new_active.cast::<HtmlInputElement>().unwrap().set_checked(true);
            new_mat.cast::<HtmlInputElement>().unwrap().set_checked(false);
            new_lac.cast::<HtmlInputElement>().unwrap().set_checked(false);
            new_aln.cast::<HtmlInputElement>().unwrap().set_checked(false);
            new_fsm.cast::<HtmlInputElement>().unwrap().set_checked(false);
            new_eal.cast::<HtmlInputElement>().unwrap().set_checked(false);
        }, ());
    }

    // TODO add learner button func


    html! {
        <div class={classes!("p-5", "w-96", "h-fit", "flex", "justify-start", "flex-col", "space-y-4", "bg-slate-100", "rounded-md")}>
            <span class={classes!("text-3xl")}>{"Add a learner"}</span>
            <input class={classes!("hover:bg-slate-100", "focus:outline-none", "input")} type={"text"} placeholder={"Names"}  ref={new_name}/>
            <div class={classes!("flex", "justify-between")}>
                <input class={classes!("hover:bg-slate-100", "focus:outline-none", "w-24", "my-2")} type={"text"} placeholder={"Gender"}  ref={new_gender}/>
                <input class={classes!("hover:bg-slate-100", "focus:outline-none", "w-16", "my-2")} type={"number"} placeholder={"Year"}  ref={new_year}/>
            </div>
            <div class={classes!("flex", "justify-between", "items-center")}>
                <label><span>{"Start date"}</span></label>
                <input class={classes!("hover:bg-slate-100", "focus:outline-none", "w-36", "my-2")} type={"date"} placeholder={"Start date"}  ref={new_start_date}/>
            </div>

            <div class={classes!("flex", "justify-between", "items-center")}>
                <label><span>{"Leave date"}</span></label>
                <input class={classes!("hover:bg-slate-100", "focus:outline-none", "w-36", "my-2")} type={"date"} placeholder={"Leave date"}  ref={new_leave_date}/>
            </div>

            <div class={classes!("flex", "flex-col", "space-y-4", "my-4")}>
                <div class={classes!("flex", "justify-between", "items-center")}>
                    <label for={"active"}><span>{"Active"}</span></label>
                    <input id={"active"}type={"checkbox"} ref={new_active} checked={true} />
                </div>
                <div class={classes!("flex", "justify-between", "items-center")}>
                    <label for={"mat"}><span>{"More able and talented"}</span></label>
                    <input id={"mat"}type={"checkbox"} ref={new_mat}/>
                </div>
                <div class={classes!("flex", "justify-between", "items-center")}>
                    <label for={"lac"}><span>{"Looked after child"}</span></label>
                    <input id={"lac"}type={"checkbox"} ref={new_lac}/>
                </div>
                <div class={classes!("flex", "justify-between", "items-center")}>
                    <label for={"eal"}><span>{"English as additional language"}</span></label>
                    <input id={"eal"}type={"checkbox"} ref={new_eal}/>
                </div>
                <div class={classes!("flex", "justify-between", "items-center")}>
                    <label for={"aln"}><span>{"Additional learning needs"}</span></label>
                    <input id={"aln"}type={"checkbox"} ref={new_aln}/>
                </div>
                <div class={classes!("flex", "justify-between", "items-center")}>
                    <label for={"fsm"}><span>{"Free school meals"}</span></label>
                    <input id={"fsm"}type={"checkbox"} ref={new_fsm}/>
                </div>

            </div>
            
            <div class={classes!("flex")}>
                <button class={classes!("bg-red-100", "grow")} onclick={clear_callback}>{"Clear"}</button>
                <button class={classes!("bg-green-100", "grow")}>{"Add learner"}</button>
            </div>

        </div>
    }
}
