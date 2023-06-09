use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct LoginRequest {
    pub account: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RegisterAdminRequest {
    pub account: String,
    pub password: String,
}
