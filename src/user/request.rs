use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct LoginRequest {
    pub user: LoginUser,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RegisterRequest {
    pub user: RegisterUser,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RegisterUser {
    pub username: String,
    pub password: String,
    pub employee_id: i32,
    pub account_type: i16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateRequest {
    pub user: UpdateUser,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateUser {
    pub password: Option<String>,
    pub account_type: Option<i16>,
}
