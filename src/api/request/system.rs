use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CreateSystemRequest {
    pub system_name: String,
    pub account_name: String,
    pub password: String,
    pub phone: String,
    pub departments: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RegisterRequest {
    pub system_id: i32,

    pub name: String,
    pub age: i32,
    pub position: Option<String>,
    pub phone: String,
    pub state: i16,
    pub approval_id: Option<i32>,

    pub account: String,
    pub password: String,
    pub account_type: i16,
}
