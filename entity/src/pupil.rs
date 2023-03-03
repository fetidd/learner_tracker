use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Default, Deserialize, Serialize)]
#[sea_orm(table_name = "pupil")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub first_names: String,
    pub last_name: String,
    pub year: i32,
    pub start_date: Date,
    pub end_date: Option<Date>,
    pub active: bool,
    pub more_able_and_talented: bool,
    pub english_as_additional_language: bool,
    pub free_school_meals: bool,
    pub additional_learning_needs: bool,
    pub looked_after_child: bool,
    pub gender: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
