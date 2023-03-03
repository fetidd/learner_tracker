use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Eq, Default, Ord)]
pub struct Pupil {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub first_names: String,
    pub last_name: String,
    pub year: i32,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
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
        end_date: Option<NaiveDate>,
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

impl PartialOrd for Pupil {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.last_name < other.last_name {
            Some(std::cmp::Ordering::Less)
        } else if self.last_name > other.last_name {
            Some(std::cmp::Ordering::Greater)
        } else {
            if self.first_names < other.first_names {
                Some(std::cmp::Ordering::Less)
            } else if self.first_names > other.first_names {
                Some(std::cmp::Ordering::Greater)
            } else {
                Some(std::cmp::Ordering::Equal)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_sort_pupils() {
        let mut pupils = vec![
            Pupil {
                id: Some(Uuid::from_str("a3aff180-4209-46ca-a409-15b53a96ec24").unwrap()),
                first_names: "E".into(),
                last_name: "E".into(),
                year: 1,
                start_date: "2021-01-01".parse().unwrap(),
                end_date: Some("2021-01-01".parse().unwrap()),
                gender: "male".into(),
                ..Default::default()
            },
            Pupil {
                id: Some(Uuid::from_str("e3aff180-4209-46ca-a409-15b53a96ec24").unwrap()),
                first_names: "C".into(),
                last_name: "C".into(),
                year: 2,
                start_date: "2025-01-01".parse().unwrap(),
                end_date: Some("2021-01-01".parse().unwrap()),
                gender: "male".into(),
                active: true,
                ..Default::default()
            },
            Pupil {
                id: Some(Uuid::from_str("f3aff180-4209-46ca-a409-15b53a96ec24").unwrap()),
                first_names: "B".into(),
                last_name: "B".into(),
                year: 1,
                start_date: "2021-01-01".parse().unwrap(),
                end_date: Some("2021-01-01".parse().unwrap()),
                gender: "male".into(),
                ..Default::default()
            },
            Pupil {
                id: Some(Uuid::from_str("b3aff180-4209-46ca-a409-15b53a96ec24").unwrap()),
                first_names: "A".into(),
                last_name: "A".into(),
                year: 5,
                start_date: "2020-01-01".parse().unwrap(),
                end_date: Some("2021-01-01".parse().unwrap()),
                gender: "male".into(),
                ..Default::default()
            },
            Pupil {
                id: Some(Uuid::from_str("c3aff180-4209-46ca-a409-15b53a96ec24").unwrap()),
                first_names: "D".into(),
                last_name: "D".into(),
                year: 2,
                start_date: "2021-01-01".parse().unwrap(),
                end_date: Some("2021-01-01".parse().unwrap()),
                gender: "male".into(),
                ..Default::default()
            },
        ];
        let expected = vec![
            Pupil {
                id: Some(Uuid::from_str("b3aff180-4209-46ca-a409-15b53a96ec24").unwrap()),
                first_names: "A".into(),
                last_name: "A".into(),
                year: 5,
                start_date: "2020-01-01".parse().unwrap(),
                end_date: Some("2021-01-01".parse().unwrap()),
                gender: "male".into(),
                ..Default::default()
            },
            Pupil {
                id: Some(Uuid::from_str("f3aff180-4209-46ca-a409-15b53a96ec24").unwrap()),
                first_names: "B".into(),
                last_name: "B".into(),
                year: 1,
                start_date: "2021-01-01".parse().unwrap(),
                end_date: Some("2021-01-01".parse().unwrap()),
                gender: "male".into(),
                ..Default::default()
            },
            Pupil {
                id: Some(Uuid::from_str("e3aff180-4209-46ca-a409-15b53a96ec24").unwrap()),
                first_names: "C".into(),
                last_name: "C".into(),
                year: 2,
                start_date: "2025-01-01".parse().unwrap(),
                end_date: Some("2021-01-01".parse().unwrap()),
                gender: "male".into(),
                active: true,
                ..Default::default()
            },
            Pupil {
                id: Some(Uuid::from_str("c3aff180-4209-46ca-a409-15b53a96ec24").unwrap()),
                first_names: "D".into(),
                last_name: "D".into(),
                year: 2,
                start_date: "2021-01-01".parse().unwrap(),
                end_date: Some("2021-01-01".parse().unwrap()),
                gender: "male".into(),
                ..Default::default()
            },
            Pupil {
                id: Some(Uuid::from_str("a3aff180-4209-46ca-a409-15b53a96ec24").unwrap()),
                first_names: "E".into(),
                last_name: "E".into(),
                year: 1,
                start_date: "2021-01-01".parse().unwrap(),
                end_date: Some("2021-01-01".parse().unwrap()),
                gender: "male".into(),
                ..Default::default()
            },
        ];
        pupils.sort();
        assert_eq!(pupils, expected);
    }
}
