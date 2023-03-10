use crate::{pupil::seed_pupils, sea_orm::DatabaseConnection, user::seed_users};
use chrono::{Datelike, NaiveDate};
use entity::pupil::ActiveModel as Pupil;
use rand::prelude::*;
use sea_orm_migration::sea_orm::Set;
use uuid::Uuid;

#[allow(dead_code)] // actually used by the server
pub async fn seed_database(db: &DatabaseConnection) {
    seed_pupils(db).await.expect("seeding pupils");
    seed_users(db).await.expect("seeding users");
}

pub fn generate_pupils(n: i32) -> Vec<Pupil> {
    let mut rng = thread_rng();
    (0..n).into_iter().map(|_| generate_pupil(&mut rng)).collect()
}

pub fn generate_pupil(rng: &mut ThreadRng) -> Pupil {
    let year = [0i32, 1, 2, 3, 4, 5, 6].choose(rng).unwrap().to_owned();
    let active = [(true, 10), (false, 1)].choose_weighted(rng, |ch| ch.1).unwrap().0;
    let (start_date, end_date) = get_dates_from_year(year, active);
    let mat = [(true, 1), (false, 10)].choose_weighted(rng, |ch| ch.1).unwrap().0;
    let aln = if mat {
        false
    } else {
        [(true, 1), (false, 5)].choose_weighted(rng, |ch| ch.1).unwrap().0
    };

    Pupil {
        id: Set(Uuid::new_v4()),
        first_names: Set(get_first_name(rng)),
        last_name: Set(get_last_name(rng)),
        year: Set(year),
        start_date: Set(start_date),
        end_date: Set(end_date),
        active: Set(active),
        more_able_and_talented: Set(mat),
        english_as_additional_language: Set([(true, 1), (false, 7)].choose_weighted(rng, |ch| ch.1).unwrap().0),
        free_school_meals: Set([(true, 1), (false, 5)].choose_weighted(rng, |ch| ch.1).unwrap().0),
        additional_learning_needs: Set(aln),
        looked_after_child: Set([(true, 1), (false, 8)].choose_weighted(rng, |ch| ch.1).unwrap().0),
        gender: Set(["male", "female"].choose(rng).unwrap().to_string()),
    }
}

fn get_first_name(rng: &mut ThreadRng) -> String {
    let first_names: Vec<&str> = vec!["Ben", "Gemma Victoria", "Daisy Enfys", "Aaron", "Kevin Huw", "Helen", "Belle", "Tyrion"];
    first_names.choose(rng).expect("no first_names").to_string()
}

fn get_last_name(rng: &mut ThreadRng) -> String {
    let first_names: Vec<&str> = vec!["Jones", "Mercer-Forbes", "Smith", "Davies", "Williams", "Forbes-Jones"];
    first_names.choose(rng).expect("no last_names").to_string()
}

fn get_dates_from_year(year: i32, is_active: bool) -> (NaiveDate, Option<NaiveDate>) {
    let curr_year = chrono::Utc::now().year();
    (
        NaiveDate::from_ymd_opt(curr_year - year, 9, 1).unwrap(),
        if is_active {
            None
        } else {
            Some(NaiveDate::from_ymd_opt(curr_year - year, 7, 21).unwrap())
        }
    )
}
