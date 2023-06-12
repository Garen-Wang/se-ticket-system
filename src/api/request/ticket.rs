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
}

#[derive(Debug, Clone, Deserialize)]
pub struct TicketFundRequest {
    pub reason: String,
    pub amount: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateAssistTicketRequest {
    pub ticket_id: i32,
    pub department_name: String,
    pub amount: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TakeTicketRequest {
    pub ticket_id: i32,
    pub department_name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FinishTicketRequest {
    pub ticket_id: i32,
}
