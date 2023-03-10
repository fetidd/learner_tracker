use super::types::PupilTableProps;
use super::*;
use crate::{
    app::AppContext,
    constant,
    elements::{Button, ModalCallbacks},
    error,
    error::*,
};
use gloo_net::http::Request;
use web_sys::HtmlInputElement;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(PupilTable)]
pub fn pupil_table(_props: &PupilTableProps) -> Html {
    let ctx = use_context::<Rc<AppContext>>().expect("NO CTX IN PUPIL TABLE");
    let show_inactive =  use_state_eq(|| false);

    // PUPILS ===================================================================================
    let pupils: UseStateHandle<Vec<Pupil>> = use_state_eq(|| vec![]);
    let pupils_cache: UseStateHandle<Vec<Pupil>> = use_state_eq(|| vec![]);
    {
        clone!(ctx, pupils, pupils_cache);
        use_effect_with_deps(
            move |_| {
                clone!(ctx, pupils, pupils_cache);
                spawn_local(async move {
                    if let Err(error) = fetch_pupils(&ctx.auth_token, pupils.clone(), pupils_cache).await {
                        error!(
                            "failed to get pupils in pupil table on load:",
                            error.to_string()
                        );
                        if error.kind == ErrorKind::Unauthorized {
                            ctx.logout_callback.emit(());
                        }
                    }
                });
            },
            (),
        );
    }
    let refresh_callback = {
        clone!(ctx, pupils, pupils_cache);
        Callback::from(move |use_server: bool| {
            clone!(ctx, pupils, pupils_cache);
            spawn_local(async move {
                clone!(pupils);
                if use_server {
                    if let Err(error) = fetch_pupils(&ctx.auth_token, pupils.clone(), pupils_cache).await {
                        error!(
                            "failed to refresh pupils in pupil table:",
                            error.to_string()
                        );
                    }
                } else {
                    pupils.set((*pupils_cache).clone());
                }
            })
        })
    };

    // FILTER ===================================================================================
    
    let filters = use_state(|| Vec::<PupilFilter>::new());
    pupils.set(
        (*pupils)
            .clone()
            .into_iter()
            .filter(|p| filter::filter(p, (*filters).clone()))
            .collect(),
    );
    let select_filter_callback = {
        clone!(filters);
        Callback::from(move |selected_filters: Vec<PupilFilter>| {
            filters.set(selected_filters);
        })
    };

    // MODALS ===================================================================================
    let (invoke_modal, dismiss_modal) =
        use_context::<ModalCallbacks>().expect("failed to get modal callbacks");
    let open_create_box = {
        clone!(invoke_modal, dismiss_modal, refresh_callback);
        Callback::from(move |ev: MouseEvent| {
            invoke_modal.emit((ev, html!(<PupilCreateBox refresh_callback={&refresh_callback} close_callback={&dismiss_modal}/>), classes!("shadow-lg", "rounded-md", "mx-auto", "my-[calc(50vh-300px)]")));
        })
    };
    let open_pupil_details = {
        clone!(invoke_modal, dismiss_modal, refresh_callback);
        Callback::from(move |(ev, pupil): (MouseEvent, Pupil)| {
            invoke_modal.emit((ev, html!(<PupilDetails pupil={pupil.clone()} refresh_callback={&refresh_callback} close_callback={&dismiss_modal}/>), classes!("shadow-lg", "rounded-md", "mx-auto", "my-[calc(50vh-120px)]")));
        })
    };
    let open_filter = {
        clone!(invoke_modal, dismiss_modal, refresh_callback);
        Callback::from(move |ev: MouseEvent| {
            invoke_modal.emit((ev, html!(<PupilTableFilter update_selected_filters={&select_filter_callback} refresh_callback={&refresh_callback} close_callback={&dismiss_modal} currently_applied={(*filters).clone()} />), classes!("shadow-lg", "rounded-md", "mx-auto", "my-[calc(50vh-300px)]")));
        })
    };
    // RENDER ===================================================================================

    html! {
        <div class="flex flex-col m-3 gap-3">
            <div class="flex p-3 gap-2 justify-between shadow-lg rounded-md bg-white">
                <Button icon={html!(<yew_feather::Plus />)} text="Add" color="green" onclick={&open_create_box} />
                <div class="flex items-center gap-3">
                    <label for="show_inactive"><em>{"Show inactive learners"}</em></label>
                    <input id="show_inactive" type="checkbox" checked={(*show_inactive).clone()} onchange={clone!(show_inactive);Callback::from(move |ev: Event| {
                        let t: HtmlInputElement = ev.target_unchecked_into();
                        show_inactive.set(t.checked());
                    })} />
                    <Button icon={html!(<yew_feather::Filter size="16" />)} text="Filter" color="purple" onclick={&open_filter} />
                    <Button icon={html!(<yew_feather::RefreshCcw size="16" />)} text="Refresh" color="green" onclick={
                        clone!(ctx, pupils, pupils_cache);
                        Callback::from(move |_ev| {
                            clone!(ctx, pupils, pupils_cache);
                                spawn_local(async move {
                                    clone!(pupils);
                                    if let Err(error) = fetch_pupils(&ctx.auth_token, pupils.clone(), pupils_cache).await {
                                        error!(
                                            "failed to refresh pupils in pupil table:",
                                            error.to_string()
                                        );
                                    }
                                })
                        })} />
                </div>
            </div>
            <div class="overflow-y-auto [max-height:calc(90vh-60px)] px-5 pt-5 scrollbar shadow-lg rounded-md bg-white">
                <ul class="sm:columns-2 2xl:columns-3 snap-y">
                    {pupils.iter().filter(|p| {
                        if !(*show_inactive) {
                            p.active
                        } else {
                            true
                        }
                    }).map(|pupil| {
                        html!{<PupilRow pupil={pupil.clone()} open_pupil_details_callback={&open_pupil_details}/>}
                    }).collect::<Html>()}
                </ul>
            </div>
            
        </div>
    }
}

async fn fetch_pupils(token: &str, pupils: UseStateHandle<Vec<Pupil>>, cache: UseStateHandle<Vec<Pupil>>) -> Result<()> {
    match Request::get(constant::PUPILS_PATH)
        .header("Authorization", &format!("Bearer {token}"))
        .send()
        .await
    {
        Ok(response) => match response.status() {
            200 => {
                let mut fetched = response.json::<Vec<Pupil>>().await?;
                fetched.sort();
                pupils.set(fetched.clone());
                cache.set(fetched);
                Ok(())
            }
            401 => Err(Unauthorized!()),
            unknown => Err(ServerError!(format!("unknown status code {unknown}"))),
        },
        Err(err) => Err(ResponseParseError!(err.to_string())),
    }
}
