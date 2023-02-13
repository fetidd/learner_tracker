use crate::{models::{Pupil, User}, routes::Route, constant, error};
use gloo_net::http::Request;
use gloo_storage::{SessionStorage, Storage};
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::Redirect;

#[function_component(PupilTable)]
pub fn pupil_table(p: &PupilTableProps) -> Html {
    let pupils: UseStateHandle<Vec<Pupil>> = use_state(|| vec![]);
    let _show_inactive = use_state(|| false);

    let new_first_names = use_node_ref();
    let new_last_name = use_node_ref();
    {
        let pupils = pupils.clone();
        use_effect_with_deps(
            move |_| {
                let pupils = pupils.clone();
                spawn_local(async move {
                    if let Ok(sid) = SessionStorage::get::<Uuid>(constant::SESSION_ID_SESSIONSTORAGE_KEY) {
                        let response = Request::get(constant::GET_PUPILS_PATH)
                            .header(constant::SESSION_ID_COOKIE, &sid.to_string())
                            .send()
                            .await;
                        match response {
                            Ok(fetched) => {
                                match fetched.json().await {
                                    Ok(fetched_pupils) => pupils.set(fetched_pupils),
                                    Err(err) => error!(err.to_string())
                                };
                            }
                            Err(err) => error!(err.to_string()),
                        }
                    } else {
                        error!("no session found, how are we at the pupils page?!");
                    };
                });
                || ()
            },
            (),
        );
    }
    if p.current_user.is_some() {
        html! {
            <>
            <div class={classes!("box")}>
                <div class={classes!("field", "is-horizontal")}>
                    <div class={classes!("field-body")}>
                        <div class={classes!("field")}>
                            <p class={classes!("control")}>
                                <input class={classes!("input")} type={"text"} placeholder={"First names"}  ref={new_first_names}/>
                            </p>
                        </div>
                        <div class={classes!("field")}>
                            <p class={classes!("control")}>
                                <input class={classes!("input")} type={"text"} placeholder={"Surname"}  ref={new_last_name}/>
                            </p>
                        </div>
                    </div>
                </div>
            </div>
            <div class={classes!{"table-container", "pt-scrollable-table", "pt-sticky-header", "snap-parent"}}>
                <table class={classes!{"table", "is-hoverable", "is-fullwidth"}}>
                    <thead>
                        <th>{"Name"}</th>
                        <th>{"Start date"}</th>
                        <th>{"Leave date"}</th>
                        <th>{"Gender"}</th>
                        <th>{"Tags"}</th>
                    </thead>
                    <tbody>
                        {pupils.iter().map(|pupil| {
                            html!{<PupilRow pupil={pupil.clone()} />}
                        }).collect::<Html>()}
                    </tbody>
                </table>
            </div>
            </>
        }
    } else {
        html!{ <Redirect<Route> to={Route::Login} /> }
    }
}

#[derive(Properties, PartialEq)]
pub struct PupilTableProps {
    pub current_user: Option<User>
}

#[function_component(PupilRow)]
pub fn pupil_row(p: &PupilRowProps) -> Html {
    html! { if p.pupil.active {
        <tr key={p.pupil.id.to_string()} class={classes!("snap-child")}>
        <td class={classes!("is-unselectable")}>{format!("{} {}", p.pupil.first_names, p.pupil.last_name)}</td>
        <td class={classes!("is-unselectable")}>{format!("{}", p.pupil.start_date)}</td>
        <td class={classes!("is-unselectable")}>{format!("{}", p.pupil.end_date)}</td>
        <td class={classes!("is-unselectable")}>{format!("{}", p.pupil.gender)}</td>
        <td>
            if p.pupil.more_able_and_talented {
                <span class={classes!("tag", "is-warning", "is-unselectable")}>{"MAT"}</span>
            }
            if p.pupil.english_as_additional_language {
                <span class={classes!("tag", "is-primary", "is-unselectable")}>{"EAL"}</span>
            }
            if p.pupil.additional_learning_needs {
                <span class={classes!("tag", "is-success", "is-unselectable")}>{"ALN"}</span>
            }
            if p.pupil.free_school_meals {
                <span class={classes!("tag", "is-danger", "is-unselectable")}>{"FSM"}</span>
            }
            if p.pupil.looked_after_child {
                <span class={classes!("tag", "is-link", "is-unselectable")}>{"LAC"}</span>
            }
        </td>
        </tr>
    }}
}

#[derive(Properties, PartialEq)]
pub struct PupilRowProps {
    pupil: Pupil,
}
