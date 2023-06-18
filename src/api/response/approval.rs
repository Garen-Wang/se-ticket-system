use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct MGetApprovalLevelByCompanyResponse {
    pub approval_names: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MGetDepartmentBySystemResponse {
    pub departments: Vec<String>,
}
