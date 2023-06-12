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
    pub amount: String,
}
