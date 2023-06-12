use crate::{
    models::department::Department,
    schema::{apply_dev_info, assist_info},
};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::query_dsl::methods::FilterDsl;
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
    pub system_id: i32,
    pub created_time: NaiveDateTime,
    pub updated_time: NaiveDateTime,
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
        let ticket = diesel::update(ticket_info::table)
            .filter(ticket_info::id.eq(id))
            .set(changeset)
            .get_result(conn)?;
        Ok(ticket)
    }

    pub fn get_count(conn: &mut PgConnection, system_id: i32) -> Result<i64, AppError> {
        let target = FilterDsl::filter(ticket_info::table, ticket_info::system_id.eq(system_id))
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
        let tickets = FilterDsl::filter(ticket_info::table, ticket_info::system_id.eq(system_id))
            .limit(size as i64)
            .offset(((page - 1) * size) as i64)
            .get_results(conn)?;
        Ok(tickets)
    }

    pub fn get_by_id(conn: &mut PgConnection, id: i32) -> Result<Self, AppError> {
        let ticket = ticket_info::table.find(id).first(conn)?;
        Ok(ticket)
    }

    pub fn get_by_creator(conn: &mut PgConnection, creator_id: i32) -> Result<Vec<Self>, AppError> {
        let tickets = FilterDsl::filter(ticket_info::table, ticket_info::creator_id.eq(creator_id))
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

    pub fn get_total_cost(conn: &mut PgConnection, ticket_id: i32) -> Result<i32, AppError> {
        let funds: Vec<Fund> =
            FilterDsl::filter(fund_list::table, fund_list::ticket_id.eq(ticket_id))
                .get_results(conn)?;
        let a = funds.into_iter().map(|x| x.amount).sum();
        Ok(a)
    }

    pub fn get_by_ticket_id(
        conn: &mut PgConnection,
        ticket_id: i32,
    ) -> Result<Vec<Fund>, AppError> {
        let funds: Vec<Fund> =
            FilterDsl::filter(fund_list::table, fund_list::ticket_id.eq(ticket_id))
                .get_results(conn)?;
        Ok(funds)
    }
}

#[derive(Debug, Clone, Identifiable, Selectable, Queryable, Associations)]
#[diesel(belongs_to(Ticket))]
#[diesel(belongs_to(Department))]
#[diesel(table_name = apply_dev_info)]
#[diesel(primary_key(ticket_id, department_id))]
pub struct TicketWithDepartments {
    pub id: i32,
    pub ticket_id: i32,
    pub department_id: i32,
    pub receiver_id: Option<i32>,
    pub state: i16,
}

#[derive(Insertable)]
#[diesel(table_name = apply_dev_info)]
pub struct InsertTicketWithDepartments {
    pub ticket_id: i32,
    pub department_id: i32,
}

impl TicketWithDepartments {
    pub fn create(
        conn: &mut PgConnection,
        ticket_id: i32,
        department_id: i32,
    ) -> Result<Self, AppError> {
        let a = diesel::insert_into(apply_dev_info::table)
            .values(InsertTicketWithDepartments {
                ticket_id,
                department_id,
            })
            .get_result(conn)?;
        Ok(a)
    }

    pub fn is_receiver(
        conn: &mut PgConnection,
        ticket_id: i32,
        receiver_id: i32,
    ) -> Result<bool, AppError> {
        let n: i64 = FilterDsl::filter(
            apply_dev_info::table,
            apply_dev_info::ticket_id
                .eq(ticket_id)
                .and(apply_dev_info::receiver_id.eq(receiver_id)),
        )
        .count()
        .get_result(conn)?;
        Ok(n != 0)
    }

    pub fn add_receiver(
        conn: &mut PgConnection,
        ticket_id: i32,
        receiver_id: i32,
        department_id: i32,
    ) -> Result<TicketWithDepartments, AppError> {
        let ret = diesel::update(apply_dev_info::table)
            .filter(
                apply_dev_info::ticket_id
                    .eq(ticket_id)
                    .and(apply_dev_info::department_id.eq(department_id)),
            )
            .set(apply_dev_info::receiver_id.eq(receiver_id))
            .get_result(conn)?;
        Ok(ret)
    }

    pub fn get_by_receiver(
        conn: &mut PgConnection,
        ticket_id: i32,
        receiver_id: i32,
    ) -> Result<Self, AppError> {
        let target = FilterDsl::filter(
            apply_dev_info::table,
            apply_dev_info::ticket_id
                .eq(ticket_id)
                .and(apply_dev_info::receiver_id.eq(receiver_id)),
        )
        .limit(1)
        .first(conn)?;
        Ok(target)
    }

    pub fn mget_by_receiver(
        conn: &mut PgConnection,
        receiver_id: i32,
    ) -> Result<Vec<Self>, AppError> {
        let target = FilterDsl::filter(
            apply_dev_info::table,
            apply_dev_info::receiver_id.eq(receiver_id),
        )
        .get_results(conn)?;
        Ok(target)
    }

    pub fn update_state(conn: &mut PgConnection, id: i32, state: i16) -> Result<(), AppError> {
        diesel::update(apply_dev_info::table.find(id))
            .set(apply_dev_info::state.eq(state))
            .execute(conn)?;
        Ok(())
    }

    pub fn get_current_ticket_id(
        conn: &mut PgConnection,
        employee_id: i32,
    ) -> Result<i32, AppError> {
        let target: TicketWithDepartments = FilterDsl::filter(
            apply_dev_info::table,
            apply_dev_info::state
                .eq(1)
                .and(apply_dev_info::receiver_id.eq(employee_id)),
        )
        .limit(1)
        .first(conn)?;
        Ok(target.id)
    }
}

#[derive(Debug, Clone, Identifiable, Selectable, Queryable)]
// #[diesel(belongs_to(Ticket))]
#[diesel(table_name = assist_info)]
pub struct Assist {
    pub id: i32,
    pub state: i16,               // 0: 没人接，1：有人接
    pub ticket_id: i32,           // 原工单
    pub submitter_id: i32,        // 接原工单，提交协助工单的人
    pub department_id: i32,       // 部门ID
    pub amount: i32,              // 人数
    pub receiver_id: Option<i32>, // 接协助工单的人
}

#[derive(Insertable)]
#[diesel(table_name = assist_info)]
pub struct InsertAssist {
    pub ticket_id: i32,
    pub submitter_id: i32,
    pub department_id: i32,
    pub amount: i32,
}

impl Assist {
    pub fn create(
        conn: &mut PgConnection,
        insert_assist: InsertAssist,
    ) -> Result<Assist, AppError> {
        let assist = diesel::insert_into(assist_info::table)
            .values(insert_assist)
            .get_result(conn)?;
        Ok(assist)
    }

    pub fn get_by_receiver(
        conn: &mut PgConnection,
        receiver_id: i32,
    ) -> Result<Vec<Assist>, AppError> {
        let assists =
            FilterDsl::filter(assist_info::table, assist_info::receiver_id.eq(receiver_id))
                .get_results(conn)?;
        Ok(assists)
    }
}
