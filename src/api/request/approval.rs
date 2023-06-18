use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct MGetApprovalLevelByCompanyRequest {
    pub company: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApproveRejectTicketRequest {
    pub ticket_id: i32,
}
