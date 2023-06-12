use chrono::NaiveDateTime;
use serde::Serialize;

use crate::models::{
    employee::Employee,
    ticket::{Fund, Ticket},
};

#[derive(Debug, Clone, Serialize)]
pub struct MGetOverviewByPageResponse {
    pub total: i64, // count all
    pub tickets: Vec<TicketOverviewResponse>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TicketOverviewResponse {
    pub tid: i32,
    pub title: String,
    pub name: String,
    pub phone: String,
    pub money: i32,
    pub submitted_time: NaiveDateTime,
    pub reason: String,
    pub address: String,
    pub funds: Vec<Fund>,
}

impl From<(Ticket, Employee, Vec<Fund>)> for TicketOverviewResponse {
    fn from((ticket, employee, funds): (Ticket, Employee, Vec<Fund>)) -> Self {
        Self {
            tid: ticket.id,
            title: ticket.title,
            phone: employee.phone,
            name: employee.name,
            money: ticket.amount,
            submitted_time: ticket.created_time,
            reason: ticket.reason,
            address: ticket.address,
            funds,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct TicketDetailResponse {
    pub id: i32,
    pub creator_id: i32,
    pub approval_id: Option<i32>,
    pub last_approver_id: Option<i32>,
    pub title: String,
    pub amount: i32,
    pub reason: String,
    pub state: i16,
    pub image: Option<String>,
    pub address: String,
    pub created_time: NaiveDateTime,
    pub updated_time: NaiveDateTime,
    pub expired_type: i16,
    pub system_id: i32,
}

impl From<Ticket> for TicketDetailResponse {
    fn from(value: Ticket) -> Self {
        Self {
            id: value.id,
            creator_id: value.creator_id,
            approval_id: value.approval_id,
            last_approver_id: value.last_approver_id,
            title: value.title,
            amount: value.amount,
            reason: value.reason,
            state: value.state,
            image: value.image,
            address: value.address,
            created_time: value.created_time,
            updated_time: value.updated_time,
            expired_type: value.expired_type,
            system_id: value.system_id,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateTicketResponse {
    pub id: i32,
    pub title: String,
    pub reason: String,
    pub funds: Vec<Fund>,
}

impl From<(Ticket, Vec<Fund>)> for CreateTicketResponse {
    fn from((ticket, funds): (Ticket, Vec<Fund>)) -> Self {
        Self {
            id: ticket.id,
            title: ticket.title,
            reason: ticket.reason,
            funds,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct HistoryTicketsResponse {
    pub tickets: Vec<HistoryTicketResponse>,
}

#[derive(Debug, Clone, Serialize)]
pub struct HistoryTicketResponse {
    pub title: String,
    pub submitter_name: String,
    pub submitter_phone: String,
    pub receiver_name: String,
    pub receiver_phone: String,
    pub reason: String,
    pub department: String,
    pub submitted_time: NaiveDateTime,
    pub finished_time: NaiveDateTime,
    pub total_cost: i32,
}
