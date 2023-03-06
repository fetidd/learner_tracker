use std::collections::HashMap;

use super::Pupil;
use crate::elements::{Button, IconButton};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(TableFilter)]
pub fn table_filter(props: &TableFilterProps) -> Html {
    let state: UseStateHandle<TableFilterState> = use_state_eq(|| props.currently_applied.clone().into());

    let onchange = {
        clone!(state);
        Callback::from(move |ev: Event| {
            let target: HtmlInputElement = ev.target_unchecked_into();
            let mut filter_state = (*state).clone();
            filter_state.update(target);
            state.set(filter_state);
        })
    };
    let update_selected_filters = props.update_selected_filters.clone();
    let apply_filters = {
        clone!(state);
        Callback::from(move |_ev| {
            let mut filters = vec![];
            for (filter, should_apply) in &(*state).flags {
                if *should_apply {
                    let to_apply = match filter.as_str() {
                        "active" => Filter::Active,
                        "inactive" => Filter::Inactive,
                        "mat" => Filter::Mat,
                        "lac" => Filter::Lac,
                        "fsm" => Filter::Fsm,
                        "aln" => Filter::Aln,
                        "eal" => Filter::Eal,
                        unknown => panic!("{unknown} is not a valid filter"),
                    };
                    filters.push(to_apply);
                }
            }
            if !(*state).name.is_empty() {
                let name_filter = Filter::Name((*state).name.to_owned());
                filters.push(name_filter);
            }
            if let Some(year) = (*state).year {
                filters.push(Filter::Year(year));
            }
            update_selected_filters.emit(filters);
        })
    };

    html! {
        <div class="flex flex-col w-[200px] h-[600px]">
            <div class="flex justify-between">
                <span class="text-2xl">{"Filters"}</span>
                <IconButton icon="close" onclick={&props.close_callback} />
            </div>
            <ul class="flex flex-col gap-3">
                <li class="flex justify-between">
                    <label for="active">{"Active"}</label>
                    <input type="checkbox" id="active" onchange={&onchange} checked={(*state).flags["active"]} />
                </li>
                <li class="flex justify-between">
                    <label for="inactive">{"Inactive"}</label>
                    <input type="checkbox" id="inactive" onchange={&onchange} checked={(*state).flags["inactive"]} />
                </li>
                <li class="flex justify-between">
                    <label for="mat">{"More able and talented"}</label>
                    <input type="checkbox" id="mat" onchange={&onchange} checked={(*state).flags["mat"]} />
                </li>
                <li class="flex justify-between">
                    <label for="lac">{"Looked after"}</label>
                    <input type="checkbox" id="lac" onchange={&onchange} checked={(*state).flags["lac"]} />
                </li>
                <li class="flex justify-between">
                    <label for="fsm">{"Free school meals"}</label>
                    <input type="checkbox" id="fsm" onchange={&onchange} checked={(*state).flags["fsm"]} />
                </li>
                <li class="flex justify-between">
                    <label for="eal">{"English as additional language"}</label>
                    <input type="checkbox" id="eal" onchange={&onchange} checked={(*state).flags["eal"]} />
                </li>
                <li class="flex justify-between">
                    <label for="aln">{"Additional learning needs"}</label>
                    <input type="checkbox" id="aln" onchange={&onchange} checked={(*state).flags["aln"]} />
                </li>
                <li class="flex justify-between">
                    <label for="name">{"Name"}</label>
                    <input type="text" id="name" onchange={&onchange} value={(*state).name.to_owned()} />
                </li>
                <li class="flex justify-between">
                    <label for="year">{"Year"}</label>
                    <input type="number" min="0" id="year" onchange={&onchange} value={
                        if let Some(year) = (*state).year {
                            year.to_string()
                        } else {
                            String::new()
                        }
                    } />
                </li>
            </ul>
            <Button color="green" text="Apply" onclick={&apply_filters} />
            <Button color="red" text="Clear" onclick={
                clone!(state);
                let refresh = props.refresh_callback.clone();
                let update = props.update_selected_filters.clone();
                Callback::from(move |_ev| {
                    state.set(TableFilterState::default());
                    update.emit(vec![]);
                    refresh.emit(());
                })
            } />
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct TableFilterProps {
    pub currently_applied: Vec<Filter>,
    pub refresh_callback: Callback<()>,
    pub close_callback: Callback<MouseEvent>,
    pub update_selected_filters: Callback<Vec<Filter>>,
}

#[derive(PartialEq, Clone, Debug)]
struct TableFilterState {
    flags: HashMap<String, bool>,
    name: String,
    year: Option<i32>,
}

impl TableFilterState {
    fn update(&mut self, target: HtmlInputElement) {
        match target.id().as_str() {
            "active" | "inactive" | "mat" | "fsm" | "eal" | "lac" | "aln" => {
                if let Some(filter_ref) = self.flags.get_mut(&target.id()) {
                    *filter_ref = target.checked();
                }
            }
            "name" => self.name = target.value(),
            "year" => {
                self.year = Some(
                    target
                        .value()
                        .parse::<i32>()
                        .expect("invalid input, not an integer"),
                )
            }
            unknown => panic!("{} is not a valid filter", unknown),
        }
    }
}

impl Default for TableFilterState {
    fn default() -> TableFilterState {
        let kvs = ["active", "inactive", "mat", "aln", "fsm", "eal", "lac"]
            .iter()
            .map(|v| (v.to_string(), false))
            .collect::<HashMap<String, bool>>();
        TableFilterState {
            flags: kvs,
            name: String::new(),
            year: None,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Filter {
    Active,
    Inactive,
    Mat,
    Aln,
    Fsm,
    Lac,
    Eal,
    Name(String),
    Year(i32),
}

impl Filter {
    pub fn apply(&self, pupil: &Pupil) -> bool {
        match self {
            Filter::Active => pupil.active,
            Filter::Inactive => !pupil.active,
            Filter::Mat => pupil.more_able_and_talented,
            Filter::Aln => pupil.additional_learning_needs,
            Filter::Fsm => pupil.free_school_meals,
            Filter::Lac => pupil.looked_after_child,
            Filter::Eal => pupil.english_as_additional_language,
            Filter::Year(filter_year) => pupil.year == *filter_year,
            Filter::Name(filter_name) => {
                // concat pupil name
                let concat_name = pupil.first_names.to_owned() + " " + &pupil.last_name;
                concat_name.contains(filter_name)
            }
        }
    }
}

pub fn filter(pupil: &Pupil, filters: Vec<Filter>) -> bool {
    for filter in filters {
        if !filter.apply(pupil) {
            return false;
        }
    }
    true
}

impl From<Vec<Filter>> for TableFilterState {
    fn from(filters: Vec<Filter>) -> Self {
        let mut tf = TableFilterState::default();
        for filter in filters {
            match filter {
                Filter::Active => *tf.flags.get_mut("active").unwrap() = true,
                Filter::Inactive => *tf.flags.get_mut("inactive").unwrap() = true,
                Filter::Mat => *tf.flags.get_mut("mat").unwrap() = true,
                Filter::Aln => *tf.flags.get_mut("aln").unwrap() = true,
                Filter::Fsm => *tf.flags.get_mut("fsm").unwrap() = true,
                Filter::Lac => *tf.flags.get_mut("lac").unwrap() = true,
                Filter::Eal => *tf.flags.get_mut("eal").unwrap() = true,
                Filter::Name(name) => tf.name = name,
                Filter::Year(year) => tf.year = Some(year),
            }
        }
        tf
    }
}
