use yew::prelude::*;

#[function_component(PupilCreateBox)]
pub fn pupil_create_box() -> Html {
    let new_first_names = use_node_ref();
    let new_last_name = use_node_ref();
    let new_year = use_node_ref();
    let new_gender = use_node_ref();
    let new_start_date = use_node_ref();
    html! {
        <div class={classes!()}>
            <input class={classes!("input")} type={"text"} placeholder={"First names"}  ref={new_first_names}/>
            <input class={classes!("input")} type={"text"} placeholder={"Surname"}  ref={new_last_name}/>
            <input class={classes!("input")} type={"text"} placeholder={"Gender"}  ref={new_gender}/>
            <input class={classes!("input")} type={"text"} placeholder={"Year"}  ref={new_year}/>
            <input class={classes!("input")} type={"text"} placeholder={"Start date"}  ref={new_start_date}/>

            <button class={classes!("bg-red-100")}>{"Clear"}</button>
            <button class={classes!("bg-green-100")}>{"Add learner"}</button>
        </div>
    }
}
