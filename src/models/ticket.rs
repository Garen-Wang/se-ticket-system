use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    error::AppError,
    schema::{fund_list, ticket_info},
};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Selectable)]
#[diesel(table_name = ticket_info)]
pub struct Ticket {
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

#[derive(Insertable)]
#[diesel(table_name = ticket_info)]
pub struct InsertTicket<'a> {
    pub creator_id: i32,
    pub title: &'a str,
    pub amount: i32,
    pub reason: &'a str,
    pub image: Option<&'a str>,
    pub address: &'a str,
}

#[derive(AsChangeset)]
#[diesel(table_name = ticket_info)]
pub struct UpdateTicket {
    pub last_approver_id: Option<i32>,
    pub title: Option<String>,
    pub amount: Option<i32>,
    pub reason: Option<String>,
    pub state: Option<i16>,
    pub image: Option<String>,
    pub address: Option<String>,
    pub expired_type: Option<i16>,
}

// static methods
impl Ticket {
    pub fn create(
        conn: &mut PgConnection,
        insert_ticket: InsertTicket,
    ) -> Result<Ticket, AppError> {
        let ticket: Ticket = diesel::insert_into(ticket_info::table)
            .values(insert_ticket)
            .get_result(conn)?;
        Ok(ticket)
    }

    pub fn update(
        conn: &mut PgConnection,
        id: i32,
        changeset: UpdateTicket,
    ) -> Result<Ticket, AppError> {
        let target = ticket_info::table.filter(ticket_info::id.eq(id));
        let ticket = diesel::update(target).set(changeset).get_result(conn)?;
        Ok(ticket)
    }

    pub fn get_count(conn: &mut PgConnection, system_id: i32) -> Result<i64, AppError> {
        let target = ticket_info::table
            .filter(ticket_info::system_id.eq(system_id))
            .count()
            .get_result(conn)?;
        Ok(target)
    }

    pub fn mget_by_page(
        conn: &mut PgConnection,
        system_id: i32,
        size: i32,
        page: i32,
    ) -> Result<Vec<Ticket>, AppError> {
        let tickets = ticket_info::table
            .filter(ticket_info::system_id.eq(system_id))
            .limit(size as i64)
            .offset(((page - 1) * size) as i64)
            .get_results(conn)?;
        Ok(tickets)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Selectable)]
#[diesel(table_name = fund_list)]
pub struct Fund {
    pub id: i32,
    pub ticket_id: i32,
    pub reason: String,
    pub amount: i32,
}

#[derive(Insertable)]
#[diesel(table_name = fund_list)]
pub struct InsertFund<'a> {
    pub ticket_id: i32,
    pub reason: &'a str,
    pub amount: i32,
}

impl Fund {
    pub fn create(conn: &mut PgConnection, insert_fund: InsertFund) -> Result<Fund, AppError> {
        let fund = diesel::insert_into(fund_list::table)
            .values(insert_fund)
            .get_result(conn)?;
        Ok(fund)
    }
}

// #[derive(Debug, Clone, Identifiable, Selectable, Queryable, Associations)]
// #[diesel(belongs_to(Ticket))]
// #[diesel(belongs_to(Employee))]
// #[diesel(table_name = assist_info)]
// #[diesel(primary_key(ticket_id, employee_id))]
// pub struct Assist {
//     pub ticket_id: i32,
//     pub employee_id: i32,
// }
