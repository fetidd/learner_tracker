use crate::error::{Error, PTResult};
use chrono::NaiveDate;
use entity::pupil::{ActiveModel, Entity, Model};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, PartialEq, Deserialize)]
pub struct Pupil {
    #[serde(default = "uuid::Uuid::new_v4")]
    id: Uuid,
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
}

impl Pupil {
    pub async fn one_from_db<Id>(id: Id, db: &DatabaseConnection) -> PTResult<Self> // TEST
    where
        Id: Into<Uuid>,
    {
        let id: Uuid = id.into();
        match Entity::find_by_id(id).one(db).await? {
            Some(pupil) => Ok(pupil.into()),
            None => Err(Error::PupilDoesNotExist),
        }
    }

    pub async fn all_from_db(db: &DatabaseConnection) -> PTResult<Vec<Self>> { // TEST
        Ok(Entity::find()
            .all(db)
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    pub async fn save(&self, db: &DatabaseConnection) -> PTResult<Self> { // TEST
        Ok(ActiveModel {
            id: Set(self.id.clone()),
            first_names: Set(self.first_names.clone()),
            last_name: Set(self.last_name.clone()),
            year: Set(self.year),
            start_date: Set(self.start_date.clone()),
            end_date: Set(self.end_date.clone()),
            active: Set(self.active),
            more_able_and_talented: Set(self.more_able_and_talented),
            english_as_additional_language: Set(self.english_as_additional_language),
            free_school_meals: Set(self.free_school_meals),
            additional_learning_needs: Set(self.additional_learning_needs),
            looked_after_child: Set(self.looked_after_child),
            gender: Set(self.gender.clone()),
        }
        .insert(db)
        .await?
        .into())
    }
}

impl From<Model> for Pupil { // TEST
    fn from(value: Model) -> Self {
        Self {
            id: value.id,
            first_names: value.first_names,
            last_name: value.last_name,
            year: value.year,
            start_date: value.start_date,
            end_date: value.end_date,
            active: value.active,
            more_able_and_talented: value.more_able_and_talented,
            english_as_additional_language: value.english_as_additional_language,
            free_school_meals: value.free_school_meals,
            additional_learning_needs: value.additional_learning_needs,
            looked_after_child: value.looked_after_child,
            gender: value.gender,
        }
    }
}
