use std::rc::Rc;
use yew::prelude::*;
use crate::{elements::{Tag, Button}, context::AppContext};
use super::pupil::Pupil;

#[function_component(PupilDetails)]
pub fn pupil_details(props: &PupilDetailsProps) -> Html {
    let ctx = use_context::<Rc<AppContext>>().expect("NO CTX IN PUPIL DETAILS");
    if let Some(pupil) = &props.pupil {
        html!{
            <div>
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
                <div class="flex flex-col space-x-1">
                    if pupil.more_able_and_talented {
                        <Tag color="purple" text="MAT" />
                    }
                    if pupil.english_as_additional_language {
                        <Tag color="yellow" text="EAL" />
                    }
                    if pupil.additional_learning_needs {
                        <Tag color="orange" text="ALN" />
                    }
                    if pupil.free_school_meals {
                        <Tag color="green" text="FSM" />
                    }
                    if pupil.looked_after_child {
                        <Tag color="blue" text="LAC" />
                    }
                </div>
                <Button onclick={&props.close_callback} color="red" text="Close" />
            </div>
        }
    } else {
        html! {
            <>
            <p>{"failed to get learner details"}</p>
            <Button onclick={&props.close_callback} color="red" text="Close" />
            </>
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct PupilDetailsProps {
    pub pupil: Option<Pupil>,
    pub close_callback: Callback<MouseEvent>,
}