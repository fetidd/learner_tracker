use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Pupil {
    pub id: Uuid,
    pub first_names: String,
    pub last_name: String,
    pub year: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub active: bool,
    pub more_able_and_talented: bool,
    pub english_as_additional_language: bool,
    pub free_school_meals: bool,
    pub additional_learning_needs: bool,
    pub looked_after_child: bool,
    pub gender: String,
}
