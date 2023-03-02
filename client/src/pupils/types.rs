use super::pupil::Pupil;
use serde::Deserialize;
use yew::{Properties, Callback};

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct AllPupilsResponse {
    pub pupils: Option<Vec<Pupil>>,
    pub error: Option<String>,
}

#[derive(Properties, PartialEq)]
pub struct PupilTableProps {}




