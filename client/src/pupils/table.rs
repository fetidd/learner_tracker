use std::rc::Rc;

use super::types::PupilTableProps;
use crate::{
    constant,
    context::AppContext,
    error,
    error::*,
    pupils::{create_box::PupilCreateBox, row::PupilRow},
    pupils::{pupil::Pupil, PupilDetails},
    utils::get_current_token, elements::Button,
};
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlElement;
use yew::prelude::*;

#[function_component(PupilTable)]
pub fn pupil_table(_props: &PupilTableProps) -> Html {
    let ctx = use_context::<Rc<AppContext>>().expect("NO CTX IN PUPIL TABLE");
    let pupils: UseStateHandle<Vec<Pupil>> = use_state(|| vec![]);
    // get pupils
    {
        let pupils = pupils.clone();
        use_effect_with_deps(
            move |_| {
                let pupils = pupils.clone();
                spawn_local(async move {
                    match get_current_token() {
                        Ok(token) => {
                            if let Err(error) = fetch_pupils(&token, pupils).await {
                                //  TODO handle error - show an alert on screen?
                            }
                        }
                        Err(error) => {}
                    }
                });
                || ()
            },
            (),
        );
    }
    let refresh_callback = {
        clone!(ctx, pupils);
        Callback::from(move |_| {
            clone!(ctx, pupils);
            spawn_local(async move {
                if let Err(error) = fetch_pupils(&ctx.auth_token, pupils).await {
                    error!(
                        "failed to refresh pupils in pupil table:",
                        error.to_string()
                    );
                }
            })
        })
    };
    let pupil_in_dialog = use_state(|| Option::<Pupil>::None);
    let modal_dialog_pupilcreatebox = use_node_ref();
    let modal_dialog_pupildetails = use_node_ref();
    let modal_backdrop = use_node_ref();
    let is_displayed = use_state(|| false);

    let open_create_box = {
        clone!(modal_dialog_pupilcreatebox, modal_backdrop, is_displayed);
        Callback::from(move |ev: MouseEvent| {
            let pos = (ev.x(), ev.y());
            clone!(modal_dialog_pupilcreatebox, modal_backdrop, is_displayed);
            open_modal(modal_dialog_pupilcreatebox, modal_backdrop, is_displayed, pos);
        })
    };

    let open_pupil_details = {
        clone!(pupils, pupil_in_dialog, modal_dialog_pupildetails, modal_backdrop, is_displayed);
        Callback::from(move |(id, pos)| {
            let pupil = (*pupils).clone()
                .into_iter()
                .filter(|p| p.id.expect("pupil should have id here").to_string() == id)
                .take(1)
                .collect::<Vec<Pupil>>().remove(0);
            pupil_in_dialog.set(Some(pupil));
            clone!(modal_dialog_pupildetails, modal_backdrop, is_displayed);
            open_modal(modal_dialog_pupildetails, modal_backdrop, is_displayed, pos);
        })
    };

    html! {
        <div class="flex flex-col m-3">
            <div class="overflow-y-auto [max-height:calc(90vh-60px)] px-5 pt-5 scrollbar shadow-lg rounded-md bg-white">
                <ul class="sm:columns-2 lg:columns-3 snap-y">
                    {pupils.iter().map(|pupil| {
                        html!{<PupilRow pupil={pupil.clone()} open_pupil_details_callback={&open_pupil_details}/>}
                    }).collect::<Html>()}
                </ul>
            </div>
            <div class="flex p-3 gap-2">
                <Button text="+ Add learner" color="green" onclick={open_create_box.clone()} />
                <Button text="Filter" color="purple" onclick={Callback::from(|_ev| {})} />
            </div>


            // MODALS
            <div ref={modal_backdrop.clone()} class="hidden modal-backdrop">
                <div ref={modal_dialog_pupildetails.clone()} class="hidden modal-dialog flex w-fit h-[240px] justify-around rounded-md shadow-xl mx-auto my-[calc(50vh-120px)]">
                    <PupilDetails pupil={(*pupil_in_dialog).clone()} refresh_callback={&refresh_callback} close_callback={
                        clone!(modal_dialog_pupildetails, modal_backdrop, is_displayed);
                        Callback::from(move |_| {
                            clone!(modal_dialog_pupildetails, modal_backdrop, is_displayed);
                            close_modal(modal_dialog_pupildetails, modal_backdrop, is_displayed);
                        })
                    }  />
                </div>
                <div ref={modal_dialog_pupilcreatebox.clone()} class="hidden modal-dialog w-96 h-[600px] justify-start flex-col space-y-4 rounded-md shadow-xl mx-auto my-[calc(50vh-300px)]">
                    <PupilCreateBox refresh_callback={&refresh_callback} close_callback={
                        clone!(modal_dialog_pupilcreatebox, modal_backdrop, is_displayed);
                        Callback::from(move |_| {
                            clone!(modal_dialog_pupilcreatebox, modal_backdrop, is_displayed);
                            close_modal(modal_dialog_pupilcreatebox, modal_backdrop, is_displayed);
                        })
                    } />
                </div>
            </div>
        </div>
    }
}

async fn fetch_pupils(token: &str, pupils: UseStateHandle<Vec<Pupil>>) -> Result<()> {
    match Request::get(constant::PUPILS_PATH)
        .header("Authorization", &format!("Bearer {token}"))
        .send()
        .await
    {
        Ok(response) => {
            let fetched = response.json::<Vec<Pupil>>().await?;
            pupils.set(fetched);
            Ok(())
        }
        Err(err) => Err(ResponseParseError!(err.to_string())),
    }
}

fn open_modal(
    modal_dialog: NodeRef,
    modal_backdrop: NodeRef,
    is_displayed: UseStateHandle<bool>,
    pos: (i32, i32)
) {
    let modal_dialog: HtmlElement = modal_dialog.cast().expect("cast modal to htmlelement");
    let mut box_classes = Classes::from(modal_dialog.class_name());
    let backdrop = modal_backdrop
        .cast::<HtmlElement>()
        .expect("cast modal_backdrop to htmlelement");
    let mut backdrop_classes = Classes::from(backdrop.class_name());
    if !*is_displayed {
        box_classes = box_classes
            .into_iter()
            .filter(|cl| cl != "hidden")
            .collect();
        backdrop_classes = backdrop_classes
            .into_iter()
            .filter(|cl| cl != "hidden")
            .collect();
        is_displayed.set(true);
    }
    modal_dialog.set_class_name(&box_classes.to_string());
    backdrop.set_class_name(&backdrop_classes.to_string());
}

fn close_modal(
    modal_dialog: NodeRef,
    modal_backdrop: NodeRef,
    is_displayed: UseStateHandle<bool>,
) {
    let modal_dialog: HtmlElement = modal_dialog.cast().expect("cast modal to htmlelement");
    let mut box_classes = Classes::from(modal_dialog.class_name());
    let backdrop = modal_backdrop
        .cast::<HtmlElement>()
        .expect("cast modal_backdrop to htmlelement");
    let mut backdrop_classes = Classes::from(backdrop.class_name());
    box_classes.push("hidden");
    backdrop_classes.push("hidden");
    is_displayed.set(false);
    modal_dialog.set_class_name(&box_classes.to_string());
    backdrop.set_class_name(&backdrop_classes.to_string());
}
