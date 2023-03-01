use crate::users::User;

#[derive(Clone, Debug, PartialEq)]
pub struct AppContext {
    pub current_user: User,
    pub auth_token: String
}