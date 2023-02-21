use super::model::Pupil;
use crate::models::User;
use serde::Deserialize;
use yew::Properties;

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct AllPupilsResponse {
    pub pupils: Option<Vec<Pupil>>,
    pub error: Option<String>,
}

#[derive(Properties, PartialEq)]
pub struct PupilTableProps {
    pub current_user: Option<User>,
}

#[derive(Properties, PartialEq)]
pub struct PupilRowProps {
    pub pupil: Pupil,
}
