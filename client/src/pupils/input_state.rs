use chrono::{NaiveDate, Utc};
use super::Pupil;

#[derive(Clone, PartialEq)]
pub struct InputState {
    pub name: String,
    pub gender: String,
    pub start_date: NaiveDate,
    pub leave_date: NaiveDate,
    pub active: bool,
    pub mat: bool,
    pub lac: bool,
    pub fsm: bool,
    pub eal: bool,
    pub aln: bool,
    pub year: i32,
}

impl Default for InputState {
    fn default() -> Self {
        let today = Utc::now().date_naive();
        Self {
            name: Default::default(),
            gender: Default::default(),
            start_date: today,
            leave_date: today,
            active: true,
            mat: Default::default(),
            lac: Default::default(),
            fsm: Default::default(),
            eal: Default::default(),
            aln: Default::default(),
            year: Default::default(),
        }
    }
}

impl From<&Pupil> for InputState {
    fn from(value: &Pupil) -> Self {
        Self {
            name: value.first_names.to_owned() + " " + &value.last_name,
            gender: value.gender.to_owned(),
            start_date: value.start_date,
            leave_date: value.end_date,
            active: value.active,
            mat: value.more_able_and_talented,
            lac: value.looked_after_child,
            fsm: value.free_school_meals,
            eal: value.english_as_additional_language,
            aln: value.additional_learning_needs,
            year: value.year,
        }
    }
}