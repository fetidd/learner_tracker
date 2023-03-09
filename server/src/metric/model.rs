use crate::{core::error::Result, record::model::*};
use chrono::NaiveDate;
use entity::metric::{ActiveModel, Column, Entity, Model};
use migration::Condition;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, Unchanged,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metric {
    pub name: String,
    pub description: String,
    pub score_1: String,
    pub score_2: String,
    pub score_3: String,
    pub score_4: String,
}

impl Metric {
    async fn insert(&self, db: &DatabaseConnection) -> Result<Self> {
        tracing::debug!("inserting metric {:?}", self);
        Ok(ActiveModel {
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
}


#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult, Transaction};

    #[fixture]
    fn test_metric() -> Metric {
        Metric {
            name: "Test Metric".into(),
            description: "This is a test metric".into(),
            score_1: "Poor".into(),
            score_2: "Ok".into(),
            score_3: "Good".into(),
            score_4: "Excellent".into(),
        }
    }

    #[rstest]
    async fn test_add_metric(test_metric: Metric) {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![<Metric as Into<Model>>::into(test_metric.clone())]])
            .into_connection();
        let result = test_metric.insert(&db).await;
        assert!(result.is_ok());
        let t_log = db.into_transaction_log();
        let exp_query = Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"INSERT INTO "metric" ("name", "description", "score_1", "score_2", "score_3", "score_4") VALUES ($1, $2, $3, $4, $5, $6) RETURNING "name", "description", "score_1", "score_2", "score_3", "score_4""#,
            [
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

}

impl From<Model> for Metric {
    fn from(value: Model) -> Self {
        Self {
            name: value.name,
            description: value.description,
            score_1: value.score_1,
            score_2: value.score_2,
            score_3: value.score_3,
            score_4: value.score_4,
        }
    }
}

impl Into<Model> for Metric {
    fn into(self) -> Model {
        Model {
            name: self.name,
            description: self.description,
            score_1: self.score_1,
            score_2: self.score_2,
            score_3: self.score_3,
            score_4: self.score_4,
        }
    }
}