use chrono::NaiveDateTime;
use serde::Serialize;

use crate::{
    models::{
        assist::Assist,
        employee::Employee,
        ticket::{Fund, Ticket, TicketWithDepartments},
    },
    AppConn,
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
pub struct CurrentTicketResponse {
    pub ticket_id: i32,
    pub title: String,
    pub submitter: String,
    pub phone_nubmer: String,
    pub submitter_ass: Option<String>,
    pub phone_number_ass: Option<String>,
    pub participants: Vec<String>,
    pub reason: String,
    pub departments: Vec<String>,
    pub state: i16,
    pub manager_id: Option<i32>,
}

impl From<(&mut AppConn, Ticket)> for CurrentTicketResponse {
    fn from((conn, ticket): (&mut AppConn, Ticket)) -> Self {
        let submitter = Employee::get_by_id(conn, ticket.creator_id).unwrap();
        let departments =
            TicketWithDepartments::mget_department_by_ticket_id(conn, ticket.id).unwrap();
        Self {
            ticket_id: ticket.id,
            title: ticket.title,
            submitter: submitter.name,
            phone_nubmer: submitter.phone,
            submitter_ass: None,
            phone_number_ass: None,
            participants: vec!["黄姥爷".into()],
            reason: ticket.reason,
            departments,
            state: ticket.state,
            manager_id: None,
        }
    }
}

impl From<(&mut AppConn, Ticket, Assist)> for CurrentTicketResponse {
    fn from((conn, ticket, assist): (&mut AppConn, Ticket, Assist)) -> Self {
        let submitter = Employee::get_by_id(conn, ticket.creator_id).unwrap();
        let assist_submitter = Employee::get_by_id(conn, assist.submitter_id).unwrap();
        let departments =
            TicketWithDepartments::mget_department_by_ticket_id(conn, ticket.id).unwrap();
        Self {
            ticket_id: ticket.id,
            title: ticket.title,
            submitter: submitter.name,
            phone_nubmer: submitter.phone,
            submitter_ass: Some(assist_submitter.name),
            phone_number_ass: Some(assist_submitter.phone),
            participants: vec!["黄姥爷".into()],
            reason: ticket.reason,
            departments,
            state: ticket.state,
            manager_id: Some(assist.submitter_id),
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
    pub ticket_id: i32,
    pub title: String,
    pub submitter: String,
    pub phone_number: String,
    pub reason: String,
    pub departments: Vec<String>,
    pub state: i16,
    pub submitted_time: NaiveDateTime,
    pub submitter_ass: Option<String>,
    pub phone_number_ass: Option<String>,
    pub image_path: Option<String>,
}

impl From<(&mut AppConn, Vec<Ticket>, Vec<Ticket>, Vec<Assist>)> for HistoryTicketsResponse {
    fn from(
        (conn, main_tickets, ass_main_tickets, ass_tickets): (
            &mut AppConn,
            Vec<Ticket>,
            Vec<Ticket>,
            Vec<Assist>,
        ),
    ) -> Self {
        let mut ret = vec![];
        for t in main_tickets.into_iter() {
            let employee = Employee::get_by_id(conn, t.creator_id).unwrap();
            let departments =
                TicketWithDepartments::mget_department_by_ticket_id(conn, t.id).unwrap();
            ret.push(HistoryTicketResponse {
                ticket_id: t.id,
                title: t.title,
                submitter: employee.name,
                phone_number: employee.phone,
                reason: t.reason,
                departments,
                state: t.state,
                submitted_time: t.created_time,
                submitter_ass: None,
                phone_number_ass: None,
                image_path: None, // TODO:
            });
        }
        for (t, ass) in ass_main_tickets.into_iter().zip(ass_tickets.into_iter()) {
            let creator = Employee::get_by_id(conn, t.creator_id).unwrap();
            let submitter = Employee::get_by_id(conn, ass.submitter_id).unwrap();
            let departments =
                TicketWithDepartments::mget_department_by_ticket_id(conn, t.id).unwrap();
            ret.push(HistoryTicketResponse {
                ticket_id: t.id,
                title: t.title,
                submitter: creator.name,
                phone_number: creator.phone,
                reason: t.reason,
                departments,
                state: t.state,
                submitted_time: t.created_time,
                submitter_ass: Some(submitter.name),
                phone_number_ass: Some(submitter.phone),
                image_path: None, // TODO:
            });
        }
        Self { tickets: ret }
    }
}
