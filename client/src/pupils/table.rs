use super::types::PupilTableProps;
use crate::{
    app::AppContext,
    constant,
    elements::{Button, ModalCallbacks},
    error,
    error::*,
    pupils::{create_box::PupilCreateBox, row::PupilRow},
    pupils::{pupil::Pupil, PupilDetails},
};
use gloo_net::http::Request;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(PupilTable)]
pub fn pupil_table(_props: &PupilTableProps) -> Html {
    let ctx = use_context::<Rc<AppContext>>().expect("NO CTX IN PUPIL TABLE");
    let pupils: UseStateHandle<Vec<Pupil>> = use_state(|| vec![]);
    // get pupils
    {
        clone!(ctx, pupils);
        use_effect_with_deps(
            move |_| {
                clone!(ctx, pupils);
                spawn_local(async move {
                    if let Err(error) = fetch_pupils(&ctx.auth_token, pupils).await {
                        error!(
                            "failed to get pupils in pupil table on load:",
                            error.to_string()
                        );
                        if error.kind == ErrorKind::Unauthorized {
                            ctx.logout_callback.emit(());
                        }
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

    // MODAL SETUP ========================================================================
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
    // ====================================================================================

    html! {
        <div class="flex flex-col m-3">
            <div class="overflow-y-auto [max-height:calc(90vh-60px)] px-5 pt-5 scrollbar shadow-lg rounded-md bg-white">
                <ul class="sm:columns-2 2xl:columns-3 snap-y">
                    {pupils.iter().map(|pupil| {
                        html!{<PupilRow pupil={pupil.clone()} open_pupil_details_callback={&open_pupil_details}/>}
                    }).collect::<Html>()}
                </ul>
            </div>
            <div class="flex p-3 gap-2">
                <Button text="+ Add learner" color="green" onclick={open_create_box.clone()} />
                <Button text="Filter" color="purple" onclick={Callback::from(|_ev| {})} />
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
        Ok(response) => match response.status() {
            401 => Err(Unauthorized!()),
            200 => {
                let mut fetched = response.json::<Vec<Pupil>>().await?;
                fetched.sort();
                pupils.set(fetched);
                Ok(())
            }
            unknown => Err(ServerError!(format!("unknown status code {unknown}"))),
        },
        Err(err) => Err(ResponseParseError!(err.to_string())),
    }
}
