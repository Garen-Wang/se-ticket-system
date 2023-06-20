use serde::Serialize;

use crate::models::account::Account;

#[derive(Debug, Clone, Serialize)]
pub struct AccountResponse {
    pub id: i32,
    pub username: String,
    pub account_type: i16,
    pub token: Option<String>,
    pub system_name: Option<String>,
}

impl From<(Account, String, Option<String>)> for AccountResponse {
    fn from((user, token, system_name): (Account, String, Option<String>)) -> Self {
        Self {
            id: user.id,
            username: user.account_name,
            account_type: user.account_type,
            token: Some(token),
            system_name,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct RegisterAdminResponse {
    pub system_id: i32,
    pub token: String,
}
