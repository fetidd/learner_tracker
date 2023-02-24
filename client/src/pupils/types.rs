use super::pupil::Pupil;
use crate::models::User;
use serde::Deserialize;
use yew::{Properties, Callback};

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

#[derive(PartialEq, Properties)]
pub struct PupilDetailsProps {
    pub id: String
}

#[derive(PartialEq, Properties)]
pub struct PupilCreateBoxProps {
    pub refresh_callback: Callback<()>
}
