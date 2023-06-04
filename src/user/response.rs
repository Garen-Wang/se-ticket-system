use serde::Serialize;

use super::model::User;

#[derive(Debug, Clone, Serialize)]
pub struct UserResponse {
    pub user: AuthUser,
    pub token: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuthUser {
    pub id: i32,
    pub username: String,
    pub account_type: i16,
}

impl From<(User, String)> for UserResponse {
    fn from((user, token): (User, String)) -> Self {
        Self {
            user: AuthUser {
                id: user.id,
                username: user.account_name,
                account_type: user.account_type,
            },
            token: Some(token),
        }
    }
}
