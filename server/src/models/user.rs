use crate::error::{Error, PTResult};
use entity::user::{ActiveModel, Entity, Model};
use sea_orm::{ActiveModelTrait, EntityTrait};
use sea_orm::{DatabaseConnection, Set};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, PartialOrd)]
pub struct User {
    first_names: String,
    last_name: String,
    email_address: String,
    #[serde(skip_serializing)]
    hashed_password: String,
    years: Vec<u32>,
}

impl User {
    pub fn new( // TEST
        first_names: &str,
        last_name: &str,
        email_address: &str,
        hashed_password: &str,
        years: Option<Vec<u32>>,
    ) -> Self {
        Self {
            first_names: first_names.to_owned(),
            last_name: last_name.to_owned(),
            email_address: email_address.to_owned(),
            hashed_password: hashed_password.to_owned(),
            years: {
                match years {
                    Some(years) => years.clone(),
                    None => vec![],
                }
            },
        }
    }

    pub async fn save(&self, db: &DatabaseConnection) -> PTResult<Self> { // TEST
        Ok(ActiveModel {
            first_names: Set(self.first_names.clone()),
            last_name: Set(self.last_name.clone()),
            email_address: Set(self.email_address.clone()),
            hashed_password: Set(self.hashed_password.clone()),
            years: Set(self
                .years
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(",")),
        }
        .insert(db)
        .await?
        .into())
    }

    pub async fn one_from_db(email: &str, db: &DatabaseConnection) -> PTResult<Self> { // TEST
        match Entity::find_by_id(email.to_owned()).one(db).await? {
            Some(user) => Ok(user.into()),
            None => Err(Error::UserDoesNotExist),
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

    pub fn hashed_password(&self) -> String {
        self.hashed_password.clone()
    }

    pub fn email_address(&self) -> String {
        self.email_address.clone()
    }
}
 
impl From<Model> for User { // TEST
    fn from(value: Model) -> Self {
        Self {
            first_names: value.first_names,
            last_name: value.last_name,
            email_address: value.email_address,
            hashed_password: value.hashed_password,
            years: value
                .years
                .split(",")
                .map(|x| {
                    x.parse::<u32>()
                        .expect("should be comma-sep'd list of ints")
                })
                .collect(),
        }
    }
}

impl From<User> for Model { // TEST
    fn from(value: User) -> Self {
        Self {
            first_names: value.first_names,
            last_name: value.last_name,
            email_address: value.email_address,
            hashed_password: value.hashed_password,
            years: value
                .years
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(","),
        }
    }
}
