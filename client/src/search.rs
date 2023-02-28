use yew::prelude::*;

#[function_component(SearchBar)]
pub fn search_bar() -> Html {

    html! {
        <div class={classes!()}>
            <input id="search-bar" type="text" />
        </div>
    }
}
