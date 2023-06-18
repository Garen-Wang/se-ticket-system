use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct MGetApprovalLevelByCompanyRequest {
    pub company: String,
}
