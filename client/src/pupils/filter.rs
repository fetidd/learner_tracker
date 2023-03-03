use super::Pupil;
use yew::prelude::*;

#[function_component(TableFilter)]
pub fn table_filter(props: &TableFilterProps) -> Html {
    let state = use_state(|| TableFilterState::default());
    let onchange = {
        clone!(state);
        Callback::from(move |ev| {})
    };
    
    html! {
        <ul class="flex flex-col gap-3">
            <li class="flex justify-between">
                <label for="active">{"Active"}</label>
                <input type="checkbox" id="active" {onchange} checked={(*state).active} />
            </li>
        </ul>
    }
}

#[derive(PartialEq, Properties)]
pub struct TableFilterProps {
    pub refresh_callback: Callback<()>,
    pub close_callback: Callback<MouseEvent>,

}

#[derive(Default)]
struct TableFilterState {
    active: bool
}

#[derive(Clone)]
pub enum Filter {
    Active,
    Inactive,
    Mat,
    Aln,
    Fsm,
    Lac,
    Eal,
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