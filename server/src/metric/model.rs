use crate::{core::error::Result};

use entity::metric::{ActiveModel, Entity, Model};

use sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, Set, Unchanged,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metric {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) score_1: String,
    pub(crate) score_2: String,
    pub(crate) score_3: String,
    pub(crate) score_4: String,
}

impl Metric {
    async fn insert(&self, db: &DatabaseConnection) -> Result<Self> {
        tracing::debug!("inserting metric {:?}", self);
        Ok(ActiveModel {
            id: Set(self.id),
            name: Set(self.name.clone()),
            description: Set(self.description.clone()),
            score_1: Set(self.score_1.clone()),
            score_2: Set(self.score_2.clone()),
            score_3: Set(self.score_3.clone()),
            score_4: Set(self.score_4.clone()),
        }
        .insert(db)
        .await?
        .into())
    }

    async fn by_id(id: Uuid, db: &DatabaseConnection) -> Result<Self> {
        match Entity::find_by_id(id).one(db).await? {
            Some(metric) => Ok(metric.into()),
            None => Err(MetricDoesNotExist!()),
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

    async fn update(&self, update: Metric, db: &DatabaseConnection) -> Result<Self> {
        Ok(ActiveModel {
            id: Unchanged(self.id),
            name: Set(update.name.clone()),
            description: Set(update.description.clone()),
            score_1: Set(update.score_1.clone()),
            score_2: Set(update.score_2.clone()),
            score_3: Set(update.score_3.clone()),
            score_4: Set(update.score_4.clone()),
        }
        .update(db)
        .await?
        .into())
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use sea_orm::{DatabaseBackend, MockDatabase, Transaction, MockExecResult};

    #[fixture]
    fn test_metric() -> Metric {
        Metric {
            id: Uuid::new_v4(),
            name: "Test Metric".into(),
            description: "This is a test metric".into(),
            score_1: "Poor".into(),
            score_2: "Ok".into(),
            score_3: "Good".into(),
            score_4: "Excellent".into(),
        }
    }

    #[rstest]
    async fn test_insert(test_metric: Metric) {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![<Metric as Into<Model>>::into(test_metric.clone())]])
            .into_connection();
        let result = test_metric.insert(&db).await;
        assert!(result.is_ok());
        let t_log = db.into_transaction_log();
        let exp_query = Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"INSERT INTO "metric" ("id", "name", "description", "score_1", "score_2", "score_3", "score_4") VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING "id", "name", "description", "score_1", "score_2", "score_3", "score_4""#,
            [
                test_metric.id.into(),
                test_metric.name.into(),
                test_metric.description.into(),
                test_metric.score_1.into(),
                test_metric.score_2.into(),
                test_metric.score_3.into(),
                test_metric.score_4.into(),
            ],
        );
        assert_eq!(t_log[0], exp_query);
    }

    #[rstest]
    async fn test_by_id(test_metric: Metric) {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![<Metric as Into<Model>>::into(test_metric.clone())]])
            .into_connection();
        let result = Metric::by_id(test_metric.id, &db).await;
        assert!(result.is_ok());
        let t_log = db.into_transaction_log();
        let exp_query = Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"SELECT "metric"."id", "metric"."name", "metric"."description", "metric"."score_1", "metric"."score_2", "metric"."score_3", "metric"."score_4" FROM "metric" WHERE "metric"."id" = $1 LIMIT $2"#,
            [
                test_metric.id.into(),
                1u64.into()
            ],
        );
        assert_eq!(t_log[0], exp_query);
    }

    #[rstest]
    async fn test_all(test_metric: Metric) {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![<Metric as Into<Model>>::into(test_metric.clone())]])
            .into_connection();
        let result = Metric::all(&db).await;
        assert!(result.is_ok());
        let t_log = db.into_transaction_log();
        let exp_query = Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"SELECT "metric"."id", "metric"."name", "metric"."description", "metric"."score_1", "metric"."score_2", "metric"."score_3", "metric"."score_4" FROM "metric""#,
            [],
        );
        assert_eq!(t_log[0], exp_query);
    }

    #[rstest]
    async fn test_update(test_metric: Metric) {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![<Metric as Into<Model>>::into(test_metric.clone())]])
            .into_connection();
        let mut update = test_metric.clone();
        update.name = "Updated test metric name".into();
        update.score_1 = "Really bad".into();
        let result = test_metric.update(update, &db).await;
        assert!(result.is_ok());
        let t_log = db.into_transaction_log();
        let exp_query = Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"UPDATE "metric" SET "name" = $1, "description" = $2, "score_1" = $3, "score_2" = $4, "score_3" = $5, "score_4" = $6 WHERE "metric"."id" = $7 RETURNING "id", "name", "description", "score_1", "score_2", "score_3", "score_4""#,
            [   
                "Updated test metric name".into(),
                test_metric.description.into(),
                "Really bad".into(),
                test_metric.score_2.into(),
                test_metric.score_3.into(),
                test_metric.score_4.into(),
                test_metric.id.into(),
            ],
        );
        assert_eq!(t_log[0], exp_query);
    }

    #[rstest]
    async fn test_delete(test_metric: Metric) {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_exec_results(vec![MockExecResult {rows_affected: 1, last_insert_id: 0}])
            .into_connection();
        let result = test_metric.delete(&db).await;
        assert!(result.is_ok());
        let t_log = db.into_transaction_log();
        let exp_query = Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"DELETE FROM "metric" WHERE "metric"."id" = $1"#,
            [   
                test_metric.id.into(),

            ],
        );
        assert_eq!(t_log[0], exp_query);
    }
}

impl From<Model> for Metric {
    fn from(value: Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
            score_1: value.score_1,
            score_2: value.score_2,
            score_3: value.score_3,
            score_4: value.score_4,
        }
    }
}

impl From<Metric> for Model {
    fn from(value: Metric) -> Model {
        Model {
            id: value.id,
            name: value.name,
            description: value.description,
            score_1: value.score_1,
            score_2: value.score_2,
            score_3: value.score_3,
            score_4: value.score_4,
        }
    }
}