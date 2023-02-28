use crate::users::User;

#[derive(Clone, Debug, PartialEq)]
pub struct AppContext {
    pub current_user: Option<User>,
    pub auth_token: String
}