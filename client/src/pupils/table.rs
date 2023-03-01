use super::types::PupilTableProps;
use crate::{
    constant, error,
    error::*,
    pupils::pupil::Pupil,
    pupils::{create_box::PupilCreateBox, row::PupilRow},
    utils::get_current_token, 
};
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(PupilTable)]
pub fn pupil_table(_props: &PupilTableProps) -> Html {
    let pupils: UseStateHandle<Vec<Pupil>> = use_state(|| vec![]);
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
                        },
                        Err(error) => {}
                    }
                    
                });
                || ()
            },
            (),
        );
    }

    let refresh_callback = {
        let pupils = pupils.clone();
        Callback::from(move |_| {
            let pupils = pupils.clone();
            spawn_local(async move {
                match get_current_token() {
                    Ok(token) => {
                        if let Err(error) = fetch_pupils(&token, pupils).await {
                            //  TODO handle error - show an alert on screen?
                        }
                    },
                    Err(error) => {}
                }
            })
        })
    };

    html! {
        <div class="flex flex-col m-2">
            <div class="overflow-y-auto [max-height:calc(90vh-60px)] px-5 pt-5">
                <ul class="sm:columns-2 snap-y">
                    {pupils.iter().map(|pupil| {
                        html!{<PupilRow pupil={pupil.clone()} />}
                    }).collect::<Html>()}
                </ul>
            </div>
            <PupilCreateBox {refresh_callback} />
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
            let mut fetched = response.json::<Vec<Pupil>>().await?;
            fetched.sort_by(|a, b| {
                a.last_name
                    .partial_cmp(&b.last_name)
                    .expect("sort pupils by last name")
            });
            pupils.set(fetched);
            Ok(())
        }
        Err(err) => Err(ResponseParseError!(err.to_string())),
    }
}
