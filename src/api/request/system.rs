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
pub struct RegisterRequest {
    pub account: String,
    pub password: String,
    pub name: String,
    pub age: String,
    pub phone_number: String,
    pub sex: String,
    pub account_type: i16,
    pub position: String,
    pub company: String,
    pub departments: Vec<String>,
    pub approval_name: String,
}
