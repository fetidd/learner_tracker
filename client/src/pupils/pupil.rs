use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Eq, PartialOrd)]
pub struct Pupil {
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<Uuid>,
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

impl Pupil {
    pub fn new(
        first_names: String,
        last_name: String,
        year: i32,
        start_date: NaiveDate,
        end_date: NaiveDate,
        active: bool,
        more_able_and_talented: bool,
        english_as_additional_language: bool,
        free_school_meals: bool,
        additional_learning_needs: bool,
        looked_after_child: bool,
        gender: String,
    ) -> Pupil {
        Pupil {
            id: None,
            first_names,
            last_name,
            year,
            start_date,
            end_date,
            active,
            more_able_and_talented,
            english_as_additional_language,
            free_school_meals,
            additional_learning_needs,
            looked_after_child,
            gender,
        }
    }
}

impl Ord for Pupil {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.last_name < other.last_name {
            std::cmp::Ordering::Less
        } else if self.last_name > other.last_name {
            std::cmp::Ordering::Greater
        } else {
            if self.first_names < other.first_names {
                std::cmp::Ordering::Less
            } else if self.first_names > other.first_names {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
