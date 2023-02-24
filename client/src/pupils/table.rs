use super::types::PupilTableProps;
use crate::{
    constant, error,
    pupils::pupil::Pupil,
    pupils::{create_box::PupilCreateBox, row::PupilRow, types::AllPupilsResponse},
    routes::Route, utils::get_current_token,
    error::*,
};
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::Redirect;

#[function_component(PupilTable)]
pub fn pupil_table(p: &PupilTableProps) -> Html {
    let pupils: UseStateHandle<Vec<Pupil>> = use_state(|| vec![]);
    let _show_inactive = use_state(|| false);
    {
        let pupils = pupils.clone();
        use_effect_with_deps(
            move |_| {
                let pupils = pupils.clone();
                spawn_local(async move {
                    let token = get_current_token();
                    if let Err(error) = fetch_pupils(&token, pupils).await {
                        //  TODO handle error - show an alert on screen?
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
                let token = get_current_token();
                if let Err(error) = fetch_pupils(&token, pupils).await {
                    //  TODO handle error - show an alert on screen?
                }
            })
        })
           
    };

    if p.current_user.is_some() {
        html! {
            <div class={classes!("flex", "space-x-10")}>
                <PupilCreateBox {refresh_callback} />
                <div class={classes!("overflow-y-auto", "pupil-table", "px-5", "grow")}>
                    <table class={classes!{"w-full"}}>
                        <thead class={classes!("sticky", "top-0", "bg-white", "h-12")}>
                            <th class={classes!("text-left")}>{"Name"}</th>
                            <th>{"Start date"}</th>
                            <th>{"Tags"}</th>
                        </thead>
                        <tbody>
                            {pupils.iter().map(|pupil| {
                                html!{<PupilRow pupil={pupil.clone()} />}
                            }).collect::<Html>()}
                        </tbody>
                    </table>
                </div>
            </div>
        }
    } else {
        html! { <Redirect<Route> to={Route::Login} /> }
    }
}

async fn fetch_pupils(token: &str, pupils: UseStateHandle<Vec<Pupil>>) -> Result<()> {
    match Request::get(constant::PUPILS_PATH).header("Authorization", &format!("Bearer {token}")).send().await {
        Ok(response) => {
            match response.json::<AllPupilsResponse>().await {
                Ok(pupil_response) => {
                    if let Some(err) = pupil_response.error {
                        Err(Unauthorized!(err))
                    } else {
                        let mut fetched = pupil_response.pupils.ok_or_else(|| JsonError!())?;
                        fetched.sort_by(|a, b| a.last_name.partial_cmp(&b.last_name).expect("sort pupils by last name"));
                        pupils.set(fetched);
                        Ok(())
                    }
                }
                Err(err) => {
                    Err(ResponseParseError!(err.to_string()))
                }
            }
        }
        Err(err) => Err(ResponseParseError!(err.to_string())),
    }
}
