use crate::{
    error::new_ok_error,
    models::department::Department,
    schema::apply_dev_info,
    utils::constant::{
        TICKET_STATE_APPROVING, TICKET_STATE_CLOSED, TICKET_STATE_OPEN, TICKET_STATE_UNAPPROVED,
    },
};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::query_dsl::methods::FilterDsl;
use serde::{Deserialize, Serialize};

use crate::{
    error::AppError,
    schema::{fund_list, ticket_info},
};

use super::{
    assist::{AssistWithDepartments, AssistWithEmployees},
    employee::Employee,
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
    pub receiver_id: Option<i32>,
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

    pub fn is_receiver(
        conn: &mut PgConnection,
        ticket_id: i32,
        employee_id: i32,
    ) -> Result<bool, AppError> {
        let ticket = Self::get_by_id(conn, ticket_id)?;
        if let Some(receiver_id) = ticket.receiver_id {
            Ok(employee_id == receiver_id)
        } else {
            Ok(false)
        }
    }

    pub fn set_receiver(
        conn: &mut PgConnection,
        ticket_id: i32,
        receiver_id: i32,
    ) -> Result<Ticket, AppError> {
        let ticket = Self::get_by_id(conn, ticket_id)?;
        if ticket.receiver_id.is_some() {
            return Err(new_ok_error("该主工单已有接受人"));
        }
        let updated_ticket: Ticket = diesel::update(ticket_info::table.find(ticket_id))
            .set(ticket_info::receiver_id.eq(receiver_id))
            .get_result(conn)?;
        Ok(updated_ticket)
    }

    // handler 判断 error 状态
    pub fn get_current_by_receiver(
        conn: &mut PgConnection,
        receiver_id: i32,
    ) -> Result<Option<Self>, AppError> {
        // open 状态的工单，且 receiver 是他
        let ticket = FilterDsl::filter(
            ticket_info::table,
            ticket_info::state
                .eq(TICKET_STATE_OPEN)
                .and(ticket_info::receiver_id.eq(receiver_id)),
        )
        .limit(1)
        .get_result::<Ticket>(conn)
        .optional()?;
        Ok(ticket)
    }

    pub fn mget_history_by_receiver(
        conn: &mut PgConnection,
        receiver_id: i32,
    ) -> Result<Vec<Self>, AppError> {
        // 拿到所有的，不管是历史的还是现在在做的
        let tickets = FilterDsl::filter(
            ticket_info::table,
            ticket_info::receiver_id
                .eq(receiver_id)
                .and(ticket_info::state.eq(TICKET_STATE_CLOSED)),
        )
        .get_results::<Ticket>(conn)?;
        Ok(tickets)
    }

    pub fn update_state(
        conn: &mut PgConnection,
        ticket_id: i32,
        new_state: i16,
    ) -> Result<Ticket, AppError> {
        let ticket = diesel::update(ticket_info::table.find(ticket_id))
            .set(ticket_info::state.eq(new_state))
            .get_result(conn)?;
        Ok(ticket)
    }

    pub fn update_amount(
        conn: &mut PgConnection,
        ticket_id: i32,
        amount: i32,
    ) -> Result<Ticket, AppError> {
        let ticket = diesel::update(ticket_info::table.find(ticket_id))
            .set(ticket_info::amount.eq(amount))
            .get_result(conn)?;
        Ok(ticket)
    }

    pub fn mget_participant(
        conn: &mut PgConnection,
        ticket_id: i32,
        with_assist: bool,
    ) -> Result<Vec<String>, AppError> {
        let mut ret = vec![];
        if with_assist {
            ret = AssistWithEmployees::mget_participant_by_assist_id(conn, ticket_id)?;
        }
        let ticket = Self::get_by_id(conn, ticket_id)?;
        if let Some(receiver_id) = ticket.receiver_id {
            let receiver = Employee::get_by_id(conn, receiver_id)?;
            ret.push(receiver.name);
        }
        Ok(ret)
    }

    pub fn mget_approving_ticket_by_system_id(
        conn: &mut PgConnection,
        system_id: i32,
    ) -> Result<Vec<Ticket>, AppError> {
        let tickets = FilterDsl::filter(
            ticket_info::table,
            ticket_info::system_id.eq(system_id).and(
                ticket_info::state
                    .ge(TICKET_STATE_UNAPPROVED)
                    .and(ticket_info::state.le(TICKET_STATE_APPROVING)),
            ),
        )
        .get_results(conn)?;
        Ok(tickets)
    }

    pub fn mget_available_by_department_ids(
        conn: &mut PgConnection,
        department_ids: Vec<i32>,
    ) -> Result<Vec<Ticket>, AppError> {
        let ticket_ids: Vec<i32> = FilterDsl::filter(
            apply_dev_info::table,
            apply_dev_info::department_id.eq_any(department_ids),
        )
        .select(apply_dev_info::ticket_id)
        .get_results(conn)?;

        let assists = FilterDsl::filter(
            ticket_info::table,
            ticket_info::id
                .eq_any(ticket_ids)
                .and(ticket_info::state.eq(TICKET_STATE_OPEN)),
        )
        .get_results(conn)?;
        Ok(assists)
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

    pub fn mget_by_ticket_id(
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

    pub fn mget_department_by_ticket_id(
        conn: &mut PgConnection,
        ticket_id: i32,
    ) -> Result<Vec<String>, AppError> {
        let departments = FilterDsl::filter(
            apply_dev_info::table,
            apply_dev_info::ticket_id.eq(ticket_id),
        )
        .get_results::<TicketWithDepartments>(conn)?;
        let mut names = vec![];
        for department in departments.iter() {
            let d = Department::get_by_id(conn, department.department_id)?;
            names.push(d.department_name);
        }
        Ok(names)
    }
}
