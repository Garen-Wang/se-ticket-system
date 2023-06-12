use chrono::NaiveDateTime;
use serde::Serialize;

use crate::models::ticket::Ticket;

#[derive(Debug, Clone, Serialize)]
pub struct MGetOverviewByPageResponse {
    pub total: i64, // count all
    pub tickets: Vec<TicketOverviewResponse>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TicketOverviewResponse {
    pub tid: String,
    pub title: String,
    pub name: String,
    pub money: String,
    pub submitted_time: NaiveDateTime,
    pub address: String,
}

impl From<Ticket> for TicketOverviewResponse {
    fn from(value: Ticket) -> Self {
        Self {
            tid: value.id.to_string(),
            title: value.title,
            name: value.creator_id.to_string(),
            money: value.amount.to_string(),
            submitted_time: value.created_time,
            address: value.address,
        }
    }
}
