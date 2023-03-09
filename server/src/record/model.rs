use crate::{core::error::Result, pupil::model::*};
use chrono::NaiveDate;
use entity::record::{ActiveModel, Column, Entity, Model};
use migration::Condition;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, Unchanged,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Record {
    pub(crate) id: Uuid,
    pub(crate) pupil: Uuid,
    pub(crate) metric: Uuid,
    pub(crate) score: u32,
    pub(crate) note: String, 
    pub(crate) date: NaiveDate,
}

impl Record {
    async fn insert(&self, db: &DatabaseConnection) -> Result<Self> {
        tracing::debug!("inserting record {:?}", self);
        Ok(ActiveModel {
            id: Set(self.id),
            pupil: Set(self.pupil),
            metric: Set(self.metric),
            score: Set(self.score),
            note: Set(self.note.to_owned()), 
            date: Set(self.date),
        }
        .insert(db)
        .await?
        .into())
    }

    async fn by_id(id: Uuid, db: &DatabaseConnection) -> Result<Self> {
        match Entity::find_by_id(id).one(db).await? {
            Some(record) => Ok(record.into()),
            None => Err(RecordDoesNotExist!()),
        }
    }

    async fn all(db: &DatabaseConnection) -> Result<Vec<Self>> {
        Ok(Entity::find()
            .all(db)
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    async fn delete(&self, db: &DatabaseConnection) -> Result<()> {
        Entity::delete_by_id(self.id).exec(db).await?;
        Ok(()) // ignore the delete result
    }

    async fn update(&self, update: Record, db: &DatabaseConnection) -> Result<Self> {
        Ok(ActiveModel {
            id: Unchanged(self.id),
            pupil: Set(update.pupil),
            metric: Set(update.metric),
            score: Set(update.score.to_owned()),
            note: Set(update.note), 
            date: Set(update.date),
        }
        .update(db)
        .await?
        .into())
    }
}

#[cfg(test)]
mod tests {
    use crate::metric::model::Metric;

    use super::*;
    use rstest::*;
    use sea_orm::{DatabaseBackend, MockDatabase, Transaction, MockExecResult};

    #[fixture]
    fn test_record() -> Record {
        Record {
            id: "73fe3f69-133b-4619-9014-58dafb5bdf04".parse().unwrap(),
            pupil: "1164ce28-8915-4126-924d-fa580f1e9f01".parse().unwrap(),
            metric: "4e91e5ff-372b-41a7-9823-b91ab5ea1d29".parse().unwrap(),
            score: 2,
            note: "note about the record".into(), 
            date: "2022-01-01".parse().unwrap(),
        }
    }

    #[fixture]
    fn test_metric() -> Metric {
        Metric {
            id: "4e91e5ff-372b-41a7-9823-b91ab5ea1d29".parse().unwrap(),
            name: "Test Metric".into(),
            description: "This is a test metric".into(),
            score_1: "Poor".into(),
            score_2: "Ok".into(),
            score_3: "Good".into(),
            score_4: "Excellent".into(),
        }
    }

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
    async fn test_insert(test_record: Record) {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![<Record as Into<Model>>::into(test_record.clone())]])
            .into_connection();
        let result = test_record.insert(&db).await;
        assert!(result.is_ok());
        let t_log = db.into_transaction_log();
        let exp_query = Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"INSERT INTO "record" ("id", "pupil", "metric", "score", "note", "date") VALUES ($1, $2, $3, $4, $5, $6) RETURNING "id", "pupil", "metric", "score", "note", "date""#,
            [
                test_record.id.into(),
                test_record.pupil.into(),
                test_record.metric.into(),
                test_record.score.into(),
                test_record.note.into(),
                test_record.date.into(),
            ],
        );
        assert_eq!(t_log[0], exp_query);
    }

    #[rstest]
    async fn test_by_id(test_record: Record) {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![<Record as Into<Model>>::into(test_record.clone())]])
            .into_connection();
        let result = Record::by_id(test_record.id, &db).await;
        assert!(result.is_ok());
        let t_log = db.into_transaction_log();
        let exp_query = Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"SELECT "record"."id", "record"."pupil", "record"."metric", "record"."score", "record"."note", "record"."date" FROM "record" WHERE "record"."id" = $1 LIMIT $2"#,
            [
                test_record.id.into(),
                1u64.into()
            ],
        );
        assert_eq!(t_log[0], exp_query);
    }

    #[rstest]
    async fn test_all(test_record: Record) {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![<Record as Into<Model>>::into(test_record.clone())]])
            .into_connection();
        let result = Record::all(&db).await;
        assert!(result.is_ok());
        let t_log = db.into_transaction_log();
        let exp_query = Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"SELECT "record"."id", "record"."name", "record"."description", "record"."score_1", "record"."score_2", "record"."score_3", "record"."score_4" FROM "record""#,
            [],
        );
        assert_eq!(t_log[0], exp_query);
    }

    #[rstest]
    async fn test_update(test_record: Record) {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![<Record as Into<Model>>::into(test_record.clone())]])
            .into_connection();
        let mut update = test_record.clone();
        update.score = 1;
        let result = test_record.update(update, &db).await;
        assert!(result.is_ok());
        let t_log = db.into_transaction_log();
        let exp_query = Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"UPDATE "record" SET "pupil" = $1, "metric" = $2, "score" = $3, "note" = $4, "date" = $5 WHERE "record"."id" = $6 RETURNING "id", "pupil", "metric", "score", "note", "date""#,
            [   
                test_record.id.into(),
                test_record.pupil.into(),
                test_record.metric.into(),
                test_record.score.into(),
                test_record.note.into(),
                test_record.date.into(),
            ],
        );
        assert_eq!(t_log[0], exp_query);
    }

    #[rstest]
    async fn test_delete(test_record: Record) {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_exec_results(vec![MockExecResult {rows_affected: 1, last_insert_id: 0}])
            .into_connection();
        let mut update = test_record.clone();
        let result = test_record.delete(&db).await;
        assert!(result.is_ok());
        let t_log = db.into_transaction_log();
        let exp_query = Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"DELETE FROM "record" WHERE "record"."id" = $1"#,
            [   
                test_record.id.into(),

            ],
        );
        assert_eq!(t_log[0], exp_query);
    }
}

impl From<Model> for Record {
    fn from(value: Model) -> Self {
        Self {
            id: value.id,
            pupil: value.pupil,
            metric: value.metric,
            score: value.score,
            note: value.note,
            date: value.date,
        }
    }
}

impl From<Record> for Model {
    fn from(value: Record) -> Model {
        Model {
            id: value.id,
            pupil: value.pupil,
            metric: value.metric,
            score: value.score,
            note: value.note,
            date: value.date,
        }
    }
}