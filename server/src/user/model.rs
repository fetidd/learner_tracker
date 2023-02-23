use crate::{
    core::error::{Error, ErrorKind, Result},
    utils::functions::generate_secret,
};
use chrono::{NaiveDateTime, Utc};
use entity::user::{ActiveModel, Entity, Model};
use sea_orm::{ActiveModelTrait, EntityTrait};
use sea_orm::{DatabaseConnection, Set};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, PartialOrd)]
pub struct User {
    pub(crate) first_names: String,
    pub(crate) last_name: String,
    pub(crate) email_address: String,
    #[serde(skip_serializing)]
    pub(crate) hashed_password: String,
    pub(crate) years: Vec<u32>,
    pub(crate) secret: Vec<u8>,
    pub(crate) last_refresh: NaiveDateTime,
}

impl User {
    pub fn new(first_names: &str, last_name: &str, email_address: &str, hashed_password: &str, years: Vec<u32>) -> Self {
        Self {
            first_names: first_names.to_owned(),
            last_name: last_name.to_owned(),
            email_address: email_address.to_owned(),
            hashed_password: hashed_password.to_owned(),
            years,
            secret: generate_secret().to_vec(),
            last_refresh: Utc::now().naive_utc(),
        }
    }

    pub async fn save(&self, db: &DatabaseConnection) -> Result<Self> {
        Ok(ActiveModel {
            first_names: Set(self.first_names.clone()),
            last_name: Set(self.last_name.clone()),
            email_address: Set(self.email_address.clone()),
            hashed_password: Set(self.hashed_password.clone()),
            years: Set(self.years.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",")),
            secret: Set(self.secret.clone()),
            last_refresh: Set(self.last_refresh.clone()),
        }
        .insert(db)
        .await?
        .into())
    }

    pub async fn one_from_db(email: &str, db: &DatabaseConnection) -> Result<Self> {
        match Entity::find_by_id(email.to_owned()).one(db).await? {
            Some(user) => Ok(user.into()),
            None => Err(UserDoesNotExist!(format!("user with email {email} does not exist"))),
        }
    }

    pub async fn all_from_db(db: &DatabaseConnection) -> Result<Vec<Self>> {
        Ok(Entity::find().all(db).await?.into_iter().map(Into::into).collect())
    }

    pub async fn refresh_secret(&self, db: &DatabaseConnection) -> Result<User> {
        let new_secret = generate_secret();
        let mut active: ActiveModel = <User as Into<Model>>::into(self.clone()).into();
        active.secret = Set(new_secret.to_vec());
        active.last_refresh = Set(Utc::now().naive_local());
        Ok(active.update(db).await?.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use migration::{Migrator, MigratorTrait};
    use rstest::*;
    use sea_orm::{Database, DatabaseBackend, MockDatabase, Transaction};

    #[rstest]
    async fn test_one_from_db() {
        let results = vec![Model {
            first_names: "test".into(),
            last_name: "user".into(),
            email_address: "test@test.com".into(),
            hashed_password: "hashedpassword".into(),
            years: "2,3".into(),
            secret: vec![127; 64],
            last_refresh: Utc::now().naive_utc(),
        }];
        let db = MockDatabase::new(DatabaseBackend::Postgres).append_query_results(vec![results.clone()]).into_connection();
        let query_res = User::one_from_db(&results[0].email_address, &db).await;
        assert!(query_res.is_ok());
        let user = query_res.unwrap();
        assert_eq!(user, results[0].clone().into());
        let t_log = db.into_transaction_log();
        let exp_query = Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"SELECT "user"."first_names", "user"."last_name", "user"."email_address", "user"."hashed_password", "user"."years", "user"."secret", "user"."last_refresh" FROM "user" WHERE "user"."email_address" = $1 LIMIT $2"#,
            [results[0].email_address.clone().into(), 1u64.into()],
        );
        assert_eq!(t_log[0], exp_query);
    }

    #[rstest]
    async fn test_all_from_db() {
        let results = vec![
            Model {
                first_names: "test".into(),
                last_name: "user".into(),
                email_address: "test@test.com".into(),
                hashed_password: "hashedpassword".into(),
                years: "2,3".into(),
                secret: vec![127; 64],
                last_refresh: Utc::now().naive_utc(),
            },
            Model {
                first_names: "test2".into(),
                last_name: "user".into(),
                email_address: "test2@test.com".into(),
                hashed_password: "hashedpassword".into(),
                years: "1,2,3,4,5,6".into(),
                secret: vec![127; 64],
                last_refresh: Utc::now().naive_utc(),
            },
        ];
        let db = MockDatabase::new(DatabaseBackend::Postgres).append_query_results(vec![results.clone()]).into_connection();
        let query_res = User::all_from_db(&db).await;
        assert!(query_res.is_ok());
        let user = query_res.unwrap();
        assert_eq!(user, results.into_iter().map(User::from).collect::<Vec<User>>());
        let t_log = db.into_transaction_log();
        let exp_query = Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"SELECT "user"."first_names", "user"."last_name", "user"."email_address", "user"."hashed_password", "user"."years", "user"."secret", "user"."last_refresh" FROM "user""#,
            [],
        );
        assert_eq!(t_log[0], exp_query);
    }

    #[rstest]
    async fn test_save() {
        let secret = [129; 64];
        let refresh_dt: NaiveDateTime = NaiveDateTime::from_timestamp_millis(1662921288).unwrap();
        let mut user = User::new("test", "user", "test@test.com", "hashedpass", vec![1, 2, 3]);
        user.last_refresh = refresh_dt;
        user.secret = secret.into();
        let model = Model {
            first_names: "test".into(),
            last_name: "user".into(),
            email_address: "test@test.com".into(),
            hashed_password: "hashedpass".into(),
            years: "1,2,3".into(),
            secret: secret.to_vec(),
            last_refresh: refresh_dt.clone(),
        };
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![model.clone()]])
            .into_connection();
        let result = user.save(&db).await;
        assert!(result.is_ok());
        let t_log = db.into_transaction_log();
        let exp_query = Transaction::from_sql_and_values(
            DatabaseBackend::Postgres,
            r#"INSERT INTO "user" ("first_names", "last_name", "email_address", "hashed_password", "years", "secret", "last_refresh") VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING "first_names", "last_name", "email_address", "hashed_password", "years", "secret", "last_refresh""#,
            [
                "test".into(),
                "user".into(),
                "test@test.com".into(),
                "hashedpass".into(),
                "1,2,3".into(),
                secret.to_vec().into(),
                refresh_dt.into(),
            ],
        );
        assert_eq!(t_log[0], exp_query);
    }

    #[rstest]
    async fn test_refresh_secret() {
        let secret = [129; 64];
        let refresh_dt: NaiveDateTime = NaiveDateTime::from_timestamp_millis(1662921288).unwrap();
        let mut user = User::new("test", "user", "test@test.com", "hashedpass", vec![1, 2, 3]);
        user.last_refresh = refresh_dt;
        user.secret = secret.into();
        let db = Database::connect("sqlite::memory:").await.unwrap();
        Migrator::up(&db, None).await.unwrap();
        let result = user.save(&db).await;
        assert!(result.is_ok());
        let refresh_result = user.refresh_secret(&db).await;
        assert!(refresh_result.is_ok());
        let refreshed = refresh_result.unwrap();
        assert_ne!(refreshed.secret, secret.to_vec());
        assert!(refreshed.last_refresh > refresh_dt);
    }
}

impl From<Model> for User {
    fn from(value: Model) -> Self {
        Self {
            first_names: value.first_names,
            last_name: value.last_name,
            email_address: value.email_address,
            hashed_password: value.hashed_password,
            years: value.years.split(",").map(|x| x.parse::<u32>().expect("should be comma-sep'd list of ints")).collect(),
            secret: value.secret.into(),
            last_refresh: value.last_refresh,
        }
    }
}

impl From<User> for Model {
    fn from(value: User) -> Self {
        Self {
            first_names: value.first_names,
            last_name: value.last_name,
            email_address: value.email_address,
            hashed_password: value.hashed_password,
            years: value.years.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","),
            secret: value.secret,
            last_refresh: value.last_refresh,
        }
    }
}

#[cfg(test)]
mod trait_tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("1")]
    #[case("1,2")]
    #[case("1,2,3")]
    fn test_user_from_model(#[case] years_string: String) {
        let model = Model {
            first_names: "test".into(),
            last_name: "user".into(),
            email_address: "test@test.com".into(),
            hashed_password: "hashedpassword".into(),
            years: years_string,
            secret: vec![127; 64],
            last_refresh: Utc::now().naive_utc(),
        };
        let _user_attempt = User::from(model);
    }

    #[rstest]
    #[should_panic]
    #[case("hello")]
    #[should_panic]
    #[case("hello,derp")]
    #[should_panic]
    #[case("1, 2, 3")]
    fn test_user_from_model_bad_string(#[case] years_string: String) {
        let model = Model {
            first_names: "test".into(),
            last_name: "user".into(),
            email_address: "test@test.com".into(),
            hashed_password: "hashedpassword".into(),
            years: years_string,
            secret: vec![127; 64],
            last_refresh: Utc::now().naive_utc(),
        };
        let _user_attempt = User::from(model);
    }

    #[rstest]
    fn test_model_from_user() {}
}
