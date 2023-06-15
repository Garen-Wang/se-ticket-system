use serde::Serialize;

use crate::models::account::Account;

#[derive(Debug, Clone, Serialize)]
pub struct AccountResponse {
    pub id: i32,
    pub username: String,
    pub account_type: i16,
    pub token: Option<String>,
    pub system_name: String,
}

impl From<(Account, String, String)> for AccountResponse {
    fn from((user, token, system_name): (Account, String, String)) -> Self {
        Self {
            id: user.id,
            username: user.account_name,
            account_type: user.account_type,
            token: Some(token),
            system_name,
        }
    }
}
