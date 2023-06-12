use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct CreateSystemResponse {
    pub id: i32,
    pub name: String,
}
