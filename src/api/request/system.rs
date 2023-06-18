use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CreateSystemRequest {
    pub name: String, // 系统名字
    pub levels: Vec<LevelItem>,
    pub departments: Vec<DepItem>,
    pub special_levels: Vec<SpecialLevelItem>,
    // pub departments: Vec<String>,
    // pub approvals: Vec<ApprovalRequest>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LevelItem {
    pub key: i64,
    pub name: String,
    pub money_limit: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DepItem {
    pub key: i64,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SpecialLevelItem {
    pub key: i64,
    pub name: String, // 特殊配置的公司名字
    pub special_level: Vec<LevelItem>,
}

#[derive(Debug, Clone, Deserialize)]
struct ApprovalRequest {
    pub approval_level: i32,
    pub approval_name: String,
    pub company_name: Option<String>,
    pub amount: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RegisterRequest {
    pub system_id: i32,

    pub name: String,
    pub age: i32,
    pub position: Option<String>,
    pub phone: String,
    pub state: i16,
    pub approval_name: Option<String>,

    pub account: String,
    pub password: String,
    pub account_type: i16,
}
