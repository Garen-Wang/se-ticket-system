use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub employee_id: i32,
    pub account_type: i16,
    pub system_id: i32,
}
