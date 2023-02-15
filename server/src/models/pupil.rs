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
    pub async fn one_from_db<Id>(id: Id, db: &DatabaseConnection) -> PTResult<Self>
    where
        Id: Into<Uuid>,
    {
        let id: Uuid = id.into();
        match Entity::find_by_id(id).one(db).await? {
            Some(pupil) => Ok(pupil.into()),
            None => Err(Error::PupilDoesNotExist),
        }
    }

    pub async fn all_from_db(db: &DatabaseConnection) -> PTResult<Vec<Self>> {
        Ok(Entity::find()
            .all(db)
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    pub async fn save(&self, db: &DatabaseConnection) -> PTResult<Self> {
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

impl Into<Model> for Pupil {
    fn into(self) -> Model {
        Model {
            id: self.id,
            first_names: self.first_names,
            last_name: self.last_name,
            year: self.year,
            start_date: self.start_date,
            end_date: self.end_date,
            active: self.active,
            more_able_and_talented: self.more_able_and_talented,
            english_as_additional_language: self.english_as_additional_language,
            free_school_meals: self.free_school_meals,
            additional_learning_needs: self.additional_learning_needs,
            looked_after_child: self.looked_after_child,
            gender: self.gender,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use sea_orm::{DatabaseBackend, MockDatabase, Transaction, MockExecResult};

    #[rstest]
    async fn test_one_from_db() {
        let results = vec![Model {
            id: Uuid::new_v4(),
            first_names: "test".into(),
            last_name: "pupil".into(),
            year: 1,
            start_date: "2022-01-01".parse().unwrap(),
            end_date: "2028-01-01".parse().unwrap(),
            active: true,
            gender: "female".into(),
            ..Default::default()
        }];
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![results.clone()])
            .into_connection();
        let query_res = Pupil::one_from_db(results[0].id, &db).await;
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
    async fn test_all_from_db() {
        let results = vec![
            Model {
                id: Uuid::new_v4(),
                first_names: "test".into(),
                last_name: "pupil".into(),
                year: 1,
                start_date: "2022-01-01".parse().unwrap(),
                end_date: "2028-01-01".parse().unwrap(),
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
                end_date: "2028-01-01".parse().unwrap(),
                active: true,
                gender: "female".into(),
                ..Default::default()
            },
        ];
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![results.clone()])
            .into_connection();
        let query_res = Pupil::all_from_db(&db).await;
        assert!(query_res.is_ok());
        let pupils = query_res.unwrap();
        assert_eq!(pupils, results.clone().into_iter().map(Pupil::from).collect::<Vec<Pupil>>());
        let t_log = db.into_transaction_log();
        let exp_query = Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"SELECT "pupil"."id", "pupil"."first_names", "pupil"."last_name", "pupil"."year", "pupil"."start_date", "pupil"."end_date", "pupil"."active", "pupil"."more_able_and_talented", "pupil"."english_as_additional_language", "pupil"."free_school_meals", "pupil"."additional_learning_needs", "pupil"."looked_after_child", "pupil"."gender" FROM "pupil""#,
            [],
        );
        assert_eq!(t_log[0], exp_query);
    }

    #[rstest]
    async fn test_save() {
        let pupil = Pupil {
            id: Uuid::new_v4(),
            first_names: "test".into(),
            last_name: "student".into(),
            year: 2,
            start_date: "2022-01-01".parse().unwrap(),
            end_date: "2028-01-01".parse().unwrap(),
            active: true,
            more_able_and_talented: false,
            english_as_additional_language: false,
            free_school_meals: false,
            additional_learning_needs: false,
            looked_after_child: false,
            gender: "male".into(),
        };
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![<Pupil as Into<Model>>::into(pupil.clone())]])
            .into_connection();
        let result = pupil.save(&db).await;
        assert!(result.is_ok());
        let t_log = db.into_transaction_log();
        let exp_query = Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"INSERT INTO "pupil" ("id", "first_names", "last_name", "year", "start_date", "end_date", "active", "more_able_and_talented", "english_as_additional_language", "free_school_meals", "additional_learning_needs", "looked_after_child", "gender") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13) RETURNING "id", "first_names", "last_name", "year", "start_date", "end_date", "active", "more_able_and_talented", "english_as_additional_language", "free_school_meals", "additional_learning_needs", "looked_after_child", "gender""#,
            [pupil.id.into(), "test".into(), "student".into(), 2.into(), pupil.start_date.into(), pupil.end_date.into(), true.into(), false.into(), false.into(), false.into(), false.into(), false.into(), "male".into()],
        );
        assert_eq!(t_log[0], exp_query);
    }
}
