use chrono::NaiveDateTime;
use serde::Serialize;

use crate::{
    error::AppError,
    models::{
        assist::Assist,
        employee::Employee,
        ticket::{Fund, Ticket, TicketWithDepartments},
    },
    utils::{constant::TEST_IMAGE_PATH, date_format},
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
    #[serde(with = "date_format")]
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
            phone: employee.phone.trim().to_string(),
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
pub struct AvailableTicketsResponse {
    pub tickets: Vec<CurrentTicketResponse>,
}

impl From<(&mut AppConn, Vec<Ticket>)> for AvailableTicketsResponse {
    fn from((conn, tickets): (&mut AppConn, Vec<Ticket>)) -> Self {
        let mut new_tickets = vec![];
        for ticket in tickets.into_iter() {
            let new_ticket = CurrentTicketResponse::from((&mut *conn, ticket));
            new_tickets.push(new_ticket);
        }
        Self {
            tickets: new_tickets,
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
            phone_nubmer: submitter.phone.trim().to_string(),
            submitter_ass: None,
            phone_number_ass: None,
            participants: Ticket::mget_participant(conn, ticket.id, false).unwrap(),
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
            phone_nubmer: submitter.phone.trim().to_string(),
            submitter_ass: Some(assist_submitter.name),
            phone_number_ass: Some(assist_submitter.phone.trim().to_string()),
            participants: Ticket::mget_participant(conn, ticket.id, true).unwrap(),
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
    #[serde(with = "date_format")]
    pub submitted_time: NaiveDateTime,
    pub submitter_ass: Option<String>,
    pub phone_number_ass: Option<String>,
    pub image_path: Option<String>,
    pub participants: Vec<String>,
    pub approval_info: Vec<String>,
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
                phone_number: employee.phone.trim().to_string(),
                reason: t.reason,
                departments,
                state: t.state,
                submitted_time: t.created_time,
                submitter_ass: None,
                phone_number_ass: None,
                image_path: Some(TEST_IMAGE_PATH.into()), // TODO:
                participants: Ticket::mget_participant(conn, t.id, false).unwrap(),
                approval_info: vec!["黄姥爷".into(), "黄姥爷".into(), "黄姥爷".into()],
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
                phone_number: creator.phone.trim().to_string(),
                reason: t.reason,
                departments,
                state: t.state,
                submitted_time: t.created_time,
                submitter_ass: Some(submitter.name),
                phone_number_ass: Some(submitter.phone.trim().to_string()),
                image_path: Some(TEST_IMAGE_PATH.into()), // TODO:
                participants: Ticket::mget_participant(conn, t.id, true).unwrap(),
                approval_info: vec!["黄姥爷".into(), "黄姥爷".into(), "黄姥爷".into()],
            });
        }
        Self { tickets: ret }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PCTicketResponse {
    pub title: String,
    pub check_name: String,
    pub ticket_id: i32,
    pub submitter: String,
    pub phone_number: String,
    pub money: i32,
    pub reason: String,
    pub state: i16,
    pub address: String,
    #[serde(with = "date_format")]
    pub submitted_time: NaiveDateTime,
    pub departments: String,
    pub detail_money: String,
    pub image_path: Option<String>,
}

impl TryFrom<(&mut AppConn, Ticket)> for PCTicketResponse {
    type Error = AppError;

    fn try_from((conn, t): (&mut AppConn, Ticket)) -> Result<Self, Self::Error> {
        let submitter = Employee::get_by_id(conn, t.creator_id)?;
        let departments = TicketWithDepartments::mget_department_by_ticket_id(conn, t.id)?;
        let funds = Fund::mget_by_ticket_id(conn, t.id)?;

        Ok(Self {
            title: t.title,
            check_name: "黄姥爷 -> 黄姥爷 -> 黄姥爷".into(), // FIXME:
            ticket_id: t.id,
            submitter: submitter.name,
            phone_number: submitter.phone.trim().to_string(),
            money: t.amount,
            reason: t.reason,
            state: t.state,
            address: t.address,
            submitted_time: t.created_time,
            departments: departments.join(", "),
            detail_money: funds
                .into_iter()
                .map(|x| format!("{}: {}", x.reason, x.amount))
                .collect::<Vec<String>>()
                .join(";"),
            image_path: Some(TEST_IMAGE_PATH.into()), // FIXME:
        })
    }
}
