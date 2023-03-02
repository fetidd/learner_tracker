use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use crate::{elements::{Tag, Button, IconButton}, context::AppContext};
use super::pupil::Pupil;

#[function_component(PupilDetails)]
pub fn pupil_details(props: &PupilDetailsProps) -> Html {
    let _ctx = use_context::<Rc<AppContext>>().expect("NO CTX IN PUPIL DETAILS");
    html!{
        <div class="w-[600px] h-[240px] flex flex-col">
            if let Some(pupil) = &props.pupil {
                <div class="flex justify-between mb-3">
                    <span class="text-2xl">{format!("{} {}", pupil.first_names, pupil.last_name)}</span>
                    <IconButton onclick={&props.close_callback} icon="" />
                </div>
                <div class="flex justify-between h-full">
                    <div class="flex flex-col grow-1">
                        <table>
                            <tr>
                                <td class="text-bold w-[120px]">{"Year"}</td>
                                <td>{format!("{}", pupil.year)}</td>
                            </tr>
                            <tr>
                                <td class="text-bold w-[120px]">{"Gender"}</td>
                                <td>{format!("{}", pupil.gender)}</td>
                            </tr>
                            <tr>
                                <td class="text-bold w-[120px]">{"Start date"}</td>
                                <td>{format!("{}", pupil.start_date)}</td>
                            </tr>
                            <tr>
                                <td class="text-bold w-[120px]">{"Leave date"}</td>
                                <td>{format!("{}", pupil.end_date)}</td>
                            </tr>
                            <tr>
                                <td class="text-bold w-[120px]">{"Tags"}</td>
                                <td class="flex space-x-1">
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
                                </td>
                            </tr>
                        </table>
                    </div>
                    <div class="flex flex-col justify-center mb-3">
                        <Button color="yellow" text="Edit" onclick={
                            clone!(pupil);
                            Callback::from(move |_ev| {
                                clone!(pupil);
                                spawn_local(async move {
                                    update_pupil(pupil).await;
                                })
                        })} />
                        <Button color="red" text="Delete" onclick={
                            clone!(pupil);
                            Callback::from(move |_ev| {
                                spawn_local(async move {
                                    delete_pupil(&pupil.id.unwrap().to_string()).await;
                                })
                        })} />
                    </div>
                </div>
            } else {
                <div class="flex justify-end">
                    <IconButton onclick={&props.close_callback} icon="" />
                </div>
                <p>{"failed to get learner details"}</p>
            }
            
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct PupilDetailsProps {
    pub pupil: Option<Pupil>,
    pub close_callback: Callback<MouseEvent>,
    pub refresh_callback: Callback<()>,
}

async fn delete_pupil(id: &str) {
    debug!("deleting", id);
}

async fn update_pupil(pupil: Pupil) {
    debug!("updating", pupil.id.unwrap().to_string());
}