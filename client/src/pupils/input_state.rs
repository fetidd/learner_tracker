use super::Pupil;
use chrono::{NaiveDate, Utc};
use web_sys::HtmlInputElement;

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

impl InputState {
    /// Update the InputState from a HtmlInputElement
    pub fn update(&mut self, target: HtmlInputElement) {
        match target.id().as_str() {
            "name" => self.name = target.value(),
            "gender" => self.gender = target.value(),
            "year" => self.year = target.value().parse::<i32>().expect("TODO HANDLE"),
            "start_date" => {
                self.start_date = target.value().parse::<NaiveDate>().expect("TODO HANDLE")
            }
            "leave_date" => {
                self.leave_date = target.value().parse::<NaiveDate>().expect("TODO HANDLE")
            }
            "active" => self.active = target.checked(),
            "mat" => {
                let is_active = target
                    .get_attribute("active")
                    .expect("tag has no active attribute");
                self.mat = match is_active.as_str() {
                    "true" => false,
                    _ => true,
                }
            }
            "lac" => {
                let is_active = target
                    .get_attribute("active")
                    .expect("tag has no active attribute");
                self.lac = match is_active.as_str() {
                    "true" => false,
                    _ => true,
                }
            }
            "aln" => {
                let is_active = target
                    .get_attribute("active")
                    .expect("tag has no active attribute");
                self.aln = match is_active.as_str() {
                    "true" => false,
                    _ => true,
                }
            }
            "fsm" => {
                let is_active = target
                    .get_attribute("active")
                    .expect("tag has no active attribute");
                self.fsm = match is_active.as_str() {
                    "true" => false,
                    _ => true,
                }
            }
            "eal" => {
                let is_active = target
                    .get_attribute("active")
                    .expect("tag has no active attribute");
                self.eal = match is_active.as_str() {
                    "true" => false,
                    _ => true,
                }
            }
            _ => panic!("input trying to change non-existent state"),
        }
    }
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

impl From<&InputState> for Pupil {
    fn from(value: &InputState) -> Self {
        let name = value.name.split(" ").collect::<Vec<&str>>();
        let (last_name, first_names) = name.split_last().expect("returns if name not 2 parts");
        Pupil::new(
            first_names.join(" "),
            last_name.to_string(),
            value.year,
            value.start_date,
            value.leave_date,
            value.active,
            value.mat,
            value.eal,
            value.fsm,
            value.aln,
            value.lac,
            value.gender.clone(),
        )
    }
}
