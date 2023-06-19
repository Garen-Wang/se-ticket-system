use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct MGetTicketByPageRequest {
    pub size: i32, // # of items per page
    pub page: i32, // # of current page
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateTicketRequest {
    pub title: String,
    pub address: String,
    pub reason: String,
    pub funds: Vec<TicketFundRequest>,
    pub departments: Vec<String>,
    pub image_url: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TicketFundRequest {
    pub reason: String,
    pub amount: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateAssistTicketRequest {
    pub ticket_id: i32,
    pub requirements: Vec<AssistRequirementRequest>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AssistRequirementRequest {
    pub department_name: String,
    pub total_num: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TakeTicketRequest {
    pub tid: i32,
    pub is_assist: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FinishTicketRequest {
    pub ticket_id: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetTicketByIDRequest {
    pub ticket_id: i32,
}
