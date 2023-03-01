use gloo_net::http::Request;
use serde_json::json;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use crate::constant;

#[function_component(SearchBar)]
pub fn search_bar() -> Html {
    let results: UseStateHandle<Vec<String>> = use_state(|| vec![]);

    let search_callback = {
        let res_handle = results.clone();
        Callback::from(move |ev: Event| {
            let res_handle = res_handle.clone();
            let search_input: HtmlInputElement = ev.target_unchecked_into();
            let search_string = search_input.value();
            if search_string.len() > 3 {
                spawn_local(async move {
                    search_request(&search_string, res_handle.clone()).await;
                })
            }
        })
    };
    html! {
        <div>
            <input id="search-input" class="w-96 h-[30px] rounded focus:outline-none" id="search-bar" type="text"  onchange={search_callback}/>
            // <button id="search-btn" class="bg-blue-200 hover:bg-blue-400">{"Search"}</button>

            if !results.is_empty() {
                <div id="result-dropdown">
                    {results.iter().map(|res| {
                        html!{
                            <span>{res}</span>
                        }
                    }).collect::<Html>()}
                </div>
            }
        </div>
    }
}

async fn search_request(request_string: &str, result_handle: UseStateHandle<Vec<String>>) {
    let token = "akljosdhfilshdfikjh";
    let result = Request::post(constant::SEARCH_ENDPOINT).json(&json!({
        "entities": ["pupil"],
        "term": request_string
    })).expect("bad json in search request").header("Authorization", &format!("Bearer {token}")).send().await;
} 
