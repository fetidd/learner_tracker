use crate::{core::error::Result};
use chrono::NaiveDate;
use entity::pupil::{ActiveModel, Entity, Model};

use sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, Set, Unchanged,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, PartialEq, Deserialize)]
pub struct Pupil {
    #[serde(default = "uuid::Uuid::new_v4")]
    pub(crate) id: Uuid,
    pub(crate) first_names: String,
    pub(crate) last_name: String,
    pub(crate) year: i32,
    pub(crate) start_date: NaiveDate,
    #[serde(skip_serializing_if="Option::is_none")]
    pub(crate) end_date: Option<NaiveDate>,
    pub(crate) active: bool,
    pub(crate) more_able_and_talented: bool,
    pub(crate) english_as_additional_language: bool,
    pub(crate) free_school_meals: bool,
    pub(crate) additional_learning_needs: bool,
    pub(crate) looked_after_child: bool,
    pub(crate) gender: String,
}

impl Pupil {
    pub async fn by_id(id: Uuid, db: &DatabaseConnection) -> Result<Self> {
        match Entity::find_by_id(id).one(db).await? {
            Some(pupil) => Ok(pupil.into()),
            None => Err(PupilDoesNotExist!()),
        }
    }

    pub async fn all(db: &DatabaseConnection) -> Result<Vec<Self>> {
        Ok(Entity::find()
            .all(db)
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    pub async fn insert(&self, db: &DatabaseConnection) -> Result<Self> {
        tracing::debug!("inserting pupil {:?}", self);
        Ok(ActiveModel {
            id: Set(self.id),
            first_names: Set(self.first_names.clone()),
            last_name: Set(self.last_name.clone()),
            year: Set(self.year),
            start_date: Set(self.start_date),
            end_date: Set(self.end_date),
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

    pub async fn update(&self, db: &DatabaseConnection) -> Result<Self> {
        Ok(ActiveModel {
            id: Unchanged(self.id),
            first_names: Set(self.first_names.clone()),
            last_name: Set(self.last_name.clone()),
            year: Set(self.year),
            start_date: Set(self.start_date),
            end_date: Set(self.end_date),
            active: Set(self.active),
            more_able_and_talented: Set(self.more_able_and_talented),
            english_as_additional_language: Set(self.english_as_additional_language),
            free_school_meals: Set(self.free_school_meals),
            additional_learning_needs: Set(self.additional_learning_needs),
            looked_after_child: Set(self.looked_after_child),
            gender: Set(self.gender.clone()),
        }
        .update(db)
        .await?
        .into())
    }

    pub fn set_from_update(&mut self, update: PupilUpdate) {
        // TODO is there a nicer way of doing this?
        if update.first_names.is_some() {
            self.first_names = update.first_names.unwrap();
        }
        if update.last_name.is_some() {
            self.last_name = update.last_name.unwrap();
        }
        if update.year.is_some() {
            self.year = update.year.unwrap();
        }
        if update.start_date.is_some() {
            self.start_date = update.start_date.unwrap();
        }
        if update.end_date.is_some() {
            self.end_date = Some(update.end_date.unwrap());
        }
        if update.active.is_some() {
            self.active = update.active.unwrap();
        }
        if update.more_able_and_talented.is_some() {
            self.more_able_and_talented = update.more_able_and_talented.unwrap();
        }
        if update.english_as_additional_language.is_some() {
            self.english_as_additional_language = update.english_as_additional_language.unwrap();
        }
        if update.free_school_meals.is_some() {
            self.free_school_meals = update.free_school_meals.unwrap();
        }
        if update.additional_learning_needs.is_some() {
            self.additional_learning_needs = update.additional_learning_needs.unwrap();
        }
        if update.looked_after_child.is_some() {
            self.looked_after_child = update.looked_after_child.unwrap();
        }
        if update.gender.is_some() {
            self.gender = update.gender.unwrap();
        }
    }

    pub async fn delete(&self, db: &DatabaseConnection) -> Result<()> {
        Entity::delete_by_id(self.id).exec(db).await?;
        Ok(()) // ignore the delete result
    }
}

#[derive(Clone, PartialEq, Debug, Deserialize, Default)]
pub struct PupilUpdate {
    pub(crate) first_names: Option<String>,
    pub(crate) last_name: Option<String>,
    pub(crate) year: Option<i32>,
    pub(crate) start_date: Option<NaiveDate>,
    pub(crate) end_date: Option<NaiveDate>,
    pub(crate) active: Option<bool>,
    pub(crate) more_able_and_talented: Option<bool>,
    pub(crate) english_as_additional_language: Option<bool>,
    pub(crate) free_school_meals: Option<bool>,
    pub(crate) additional_learning_needs: Option<bool>,
    pub(crate) looked_after_child: Option<bool>,
    pub(crate) gender: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult, Transaction};

    #[fixture]
    fn test_pupil() -> Pupil {
        Pupil {
            id: "1164ce28-8915-4126-924d-fa580f1e9f01".parse().unwrap(),
            first_names: "test".into(),
            last_name: "pupil".into(),
            year: 6,
            start_date: "2021-01-01".parse().unwrap(),
            end_date: None,
            active: true,
            more_able_and_talented: false,
            english_as_additional_language: false,
            free_school_meals: false,
            additional_learning_needs: false,
            looked_after_child: false,
            gender: "gender".into(),
        }
    }

    #[rstest]
    async fn test_by_id() {
        let results = vec![Model {
            id: Uuid::new_v4(),
            first_names: "test".into(),
            last_name: "pupil".into(),
            year: 1,
            start_date: "2022-01-01".parse().unwrap(),
            active: true,
            gender: "female".into(),
            ..Default::default()
        }];
        let user = User::new("test", "user", "test@test.com", "pass", vec![1]);
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![results.clone()])
            .into_connection();
        let query_res = Pupil::by_id(results[0].id, &db).await; // TEST user restrictions
        assert!(query_res.is_ok());
        let pupil = query_res.unwrap();
        assert_eq!(pupil, results[0].clone().into());
        let t_log = db.into_transaction_log();
        let exp_query = Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"SELECT "pupil"."id", "pupil"."first_names", "pupil"."last_name", "pupil"."year", "pupil"."start_date", "pupil"."end_date", "pupil"."active", "pupil"."more_able_and_talented", "pupil"."english_as_additional_language", "pupil"."free_school_meals", "pupil"."additional_learning_needs", "pupil"."looked_after_child", "pupil"."gender" FROM "pupil" WHERE "pupil"."id" = $1 LIMIT $2"#,
            [results[0].id.into(), 1u64.into()],
        );
        assert_eq!(t_log[0], exp_query);
    }

    #[rstest]
    async fn test_all() {
        let results = vec![
            Model {
                id: Uuid::new_v4(),
                first_names: "test".into(),
                last_name: "pupil".into(),
                year: 1,
                start_date: "2022-01-01".parse().unwrap(),
                active: true,
                gender: "female".into(),
                ..Default::default()
            },
            Model {
                id: Uuid::new_v4(),
                first_names: "test".into(),
                last_name: "pupil".into(),
                year: 1,
                start_date: "2022-01-01".parse().unwrap(),
                active: true,
                gender: "female".into(),
                ..Default::default()
            },
        ];
        let user = User::new("test", "user", "test@test.com", "pass", vec![1]); // TEST user restrictions
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![results.clone()])
            .into_connection();
        let query_res = Pupil::all(&db).await;
        assert!(query_res.is_ok());
        let pupils = query_res.unwrap();
        assert_eq!(
            pupils,
            results
                .clone()
                .into_iter()
                .map(Pupil::from)
                .collect::<Vec<Pupil>>()
        );
        let t_log = db.into_transaction_log();
        let exp_query = Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"SELECT "pupil"."id", "pupil"."first_names", "pupil"."last_name", "pupil"."year", "pupil"."start_date", "pupil"."end_date", "pupil"."active", "pupil"."more_able_and_talented", "pupil"."english_as_additional_language", "pupil"."free_school_meals", "pupil"."additional_learning_needs", "pupil"."looked_after_child", "pupil"."gender" FROM "pupil""#,
            [],
        );
        assert_eq!(t_log[0], exp_query);
    }

    #[rstest]
    async fn test_save(test_pupil: Pupil) {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![<Pupil as Into<Model>>::into(test_pupil.clone())]])
            .into_connection();
        let result = test_pupil.insert(&db).await;
        assert!(result.is_ok());
        let t_log = db.into_transaction_log();
        let exp_query = Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"INSERT INTO "pupil" ("id", "first_names", "last_name", "year", "start_date", "end_date", "active", "more_able_and_talented", "english_as_additional_language", "free_school_meals", "additional_learning_needs", "looked_after_child", "gender") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13) RETURNING "id", "first_names", "last_name", "year", "start_date", "end_date", "active", "more_able_and_talented", "english_as_additional_language", "free_school_meals", "additional_learning_needs", "looked_after_child", "gender""#,
            [
                test_pupil.id.into(),
                "test".into(),
                "pupil".into(),
                6.into(),
                test_pupil.start_date.into(),
                test_pupil.end_date.into(),
                true.into(),
                false.into(),
                false.into(),
                false.into(),
                false.into(),
                false.into(),
                "gender".into(),
            ],
        );
        assert_eq!(t_log[0], exp_query);
    }

    #[rstest]
    async fn test_update(mut test_pupil: Pupil) {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![<Pupil as Into<Model>>::into(test_pupil.clone())]])
            .into_connection();
        let update = PupilUpdate {
            last_name: Some("newname".into()),
            free_school_meals: Some(true),
            ..Default::default()
        };
        test_pupil.set_from_update(update);
        let result = test_pupil.update(&db).await;
        assert!(result.is_ok());
        let t_log = db.into_transaction_log();
        let exp_query = Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"UPDATE "pupil" SET "first_names" = $1, "last_name" = $2, "year" = $3, "start_date" = $4, "end_date" = $5, "active" = $6, "more_able_and_talented" = $7, "english_as_additional_language" = $8, "free_school_meals" = $9, "additional_learning_needs" = $10, "looked_after_child" = $11, "gender" = $12 WHERE "pupil"."id" = $13 RETURNING "id", "first_names", "last_name", "year", "start_date", "end_date", "active", "more_able_and_talented", "english_as_additional_language", "free_school_meals", "additional_learning_needs", "looked_after_child", "gender""#,
            [
                "test".into(),
                "newname".into(),
                6.into(),
                test_pupil.start_date.into(),
                test_pupil.end_date.into(),
                true.into(),
                false.into(),
                false.into(),
                true.into(),
                false.into(),
                false.into(),
                "gender".into(),
                "1164ce28-8915-4126-924d-fa580f1e9f01"
                    .parse::<Uuid>()
                    .unwrap()
                    .into(),
            ],
        );
        assert_eq!(t_log[0], exp_query);
    }

    #[rstest] // TODO add more cases
    #[case(PupilUpdate{last_name: Some("newname".into()), ..Default::default()}, Pupil {
        id: "1164ce28-8915-4126-924d-fa580f1e9f01".parse().unwrap(),
        first_names: "test".into(),
        last_name: "newname".into(),
        year: 6,
        start_date: "2021-01-01".parse().unwrap(),
        end_date: None,
        active: true,
        more_able_and_talented: false,
        english_as_additional_language: false,
        free_school_meals: false,
        additional_learning_needs: false,
        looked_after_child: false,
        gender: "gender".into(),
    })]
    async fn test_set_from_update(
        mut test_pupil: Pupil,
        #[case] update: PupilUpdate,
        #[case] expected: Pupil,
    ) {
        test_pupil.set_from_update(update);
        assert_eq!(expected, test_pupil);
    }

    #[rstest]
    async fn test_delete(test_pupil: Pupil) {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_exec_results(vec![MockExecResult {
                rows_affected: 1,
                last_insert_id: 0,
            }])
            .into_connection();
        let result = test_pupil.delete(&db).await;
        assert!(result.is_ok());
        let t_log = db.into_transaction_log();
        let exp_query = Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"DELETE FROM "pupil" WHERE "pupil"."id" = $1"#,
            ["1164ce28-8915-4126-924d-fa580f1e9f01"
                .parse::<Uuid>()
                .unwrap()
                .into()],
        );
        assert_eq!(t_log[0], exp_query);
    }
}

impl From<Model> for Pupil {
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

impl From<Pupil> for Model {
    fn from(val: Pupil) -> Self {
        Model {
            id: val.id,
            first_names: val.first_names,
            last_name: val.last_name,
            year: val.year,
            start_date: val.start_date,
            end_date: val.end_date,
            active: val.active,
            more_able_and_talented: val.more_able_and_talented,
            english_as_additional_language: val.english_as_additional_language,
            free_school_meals: val.free_school_meals,
            additional_learning_needs: val.additional_learning_needs,
            looked_after_child: val.looked_after_child,
            gender: val.gender,
        }
    }
}
