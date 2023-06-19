use crate::{
    error::new_ok_error,
    models::department::Department,
    schema::apply_dev_info,
    utils::constant::{
        TICKET_STATE_APPROVING, TICKET_STATE_ASSIGNED, TICKET_STATE_CLOSED, TICKET_STATE_OPEN,
        TICKET_STATE_UNAPPROVED,
    },
};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::query_dsl::methods::FilterDsl;
use serde::{Deserialize, Serialize};

use crate::{
    error::AppError,
    schema::{approved_info, fund_list, ticket_info},
};

use super::{approval::Approval, assist::AssistWithEmployees, employee::Employee};

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
    pub amount: Option<i32>,
    pub state: Option<i16>,
    pub approval_id: Option<Option<i32>>,
    pub receiver_id: Option<i32>,
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

    // TODO: 有问题，包含了历史审批
    pub fn get_count(conn: &mut PgConnection, system_id: i32) -> Result<i64, AppError> {
        let target = FilterDsl::filter(ticket_info::table, ticket_info::system_id.eq(system_id))
            .count()
            .get_result(conn)?;
        Ok(target)
    }

    pub fn get_approving_count(
        conn: &mut PgConnection,
        system_id: i32,
        approval_id: i32,
    ) -> Result<i64, AppError> {
        let target = FilterDsl::filter(
            ticket_info::table,
            ticket_info::system_id
                .eq(system_id)
                .and(ticket_info::approval_id.eq(approval_id))
                .and(ticket_info::state.lt(TICKET_STATE_OPEN)),
        )
        .count()
        .get_result(conn)?;
        Ok(target)
    }

    // TODO: 有问题，包含了历史审批
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

    pub fn mget_approving_by_page(
        conn: &mut PgConnection,
        system_id: i32,
        approval_id: i32,
        size: i32,
        page: i32,
    ) -> Result<Vec<Ticket>, AppError> {
        let tickets = FilterDsl::filter(
            ticket_info::table,
            ticket_info::system_id
                .eq(system_id)
                .and(ticket_info::approval_id.eq(approval_id))
                .and(ticket_info::state.lt(TICKET_STATE_OPEN)),
        )
        .limit(size as i64)
        .offset(((page - 1) * size) as i64)
        .get_results(conn)?;
        Ok(tickets)
    }

    pub fn get_history_count(
        conn: &mut PgConnection,
        approval_id: i32,
        employee_id: i32,
        id: Option<i32>,
        title: Option<String>,
    ) -> Result<i64, AppError> {
        let mut query = approved_info::table.into_boxed();
        query = FilterDsl::filter(
            query,
            approved_info::approval_id
                .eq(approval_id)
                .and(approved_info::employee_id.eq(employee_id)),
        );
        if let Some(id) = id {
            query = FilterDsl::filter(query, approved_info::ticket_id.eq(id));
        }
        let ticket_ids: Vec<i32> = query.select(approved_info::ticket_id).get_results(conn)?;
        if let Some(title) = title {
            let a = FilterDsl::filter(
                ticket_info::table,
                ticket_info::id
                    .eq_any(ticket_ids)
                    .and(ticket_info::title.ilike(format!("%{}%", title))),
            )
            .count()
            .get_result(conn)?;
            Ok(a)
        } else {
            Ok(ticket_ids.len() as i64)
        }
    }

    // 我审批过的工单
    pub fn mget_history_by_approver(
        conn: &mut PgConnection,
        approval_id: i32,
        employee_id: i32,
        id: Option<i32>,
        title: Option<String>,
        size: i32,
        page: i32,
    ) -> Result<Vec<Ticket>, AppError> {
        let mut query = approved_info::table.into_boxed();
        query = FilterDsl::filter(
            query,
            approved_info::approval_id
                .eq(approval_id)
                .and(approved_info::employee_id.eq(employee_id)),
        );
        if let Some(id) = id {
            query = FilterDsl::filter(query, approved_info::ticket_id.eq(id));
        }
        let ticket_ids: Vec<i32> = query
            .select(approved_info::ticket_id)
            .limit(size as i64)
            .offset(((page - 1) * size) as i64)
            .get_results(conn)?;
        let mut query = ticket_info::table.into_boxed();
        query = FilterDsl::filter(query, ticket_info::id.eq_any(ticket_ids));
        if let Some(title) = title {
            query = FilterDsl::filter(query, ticket_info::title.like(format!("%{}%", title)));
        }
        let tickets: Vec<Ticket> = query.get_results(conn)?;
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
            // .set(ticket_info::receiver_id.eq(receiver_id))
            .set(UpdateTicket {
                last_approver_id: None,
                amount: None,
                state: None,
                approval_id: None,
                receiver_id: Some(receiver_id),
            })
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
                .eq(TICKET_STATE_ASSIGNED)
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
            // .set(ticket_info::state.eq(new_state))
            .set(UpdateTicket {
                last_approver_id: None,
                amount: None,
                state: Some(new_state),
                approval_id: None,
                receiver_id: None,
            })
            .get_result(conn)?;
        Ok(ticket)
    }

    pub fn update_amount(
        conn: &mut PgConnection,
        ticket_id: i32,
        amount: i32,
    ) -> Result<Ticket, AppError> {
        let ticket = diesel::update(ticket_info::table.find(ticket_id))
            // .set(ticket_info::amount.eq(amount))
            .set(UpdateTicket {
                last_approver_id: None,
                amount: Some(amount),
                state: None,
                approval_id: None,
                receiver_id: None,
            })
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

    pub fn update_approval_id(
        conn: &mut PgConnection,
        ticket_id: i32,
        approval_id: Option<i32>,
    ) -> Result<Self, AppError> {
        let a = diesel::update(ticket_info::table)
            .filter(ticket_info::id.eq(ticket_id))
            // .set(ticket_info::approval_id.eq(approval_id))
            .set(UpdateTicket {
                last_approver_id: None,
                amount: None,
                state: None,
                approval_id: Some(approval_id),
                receiver_id: None,
            })
            .get_result(conn)?;
        Ok(a)
    }

    pub fn update_next_current_approval_id(
        conn: &mut PgConnection,
        ticket_id: i32,
        company_name: Option<String>,
        cur_appover_id: i32,
    ) -> Result<Option<Self>, AppError> {
        let ticket = Self::get_by_id(conn, ticket_id)?;
        let cur_money_limit = if let Some(approval_id) = ticket.approval_id {
            let approval = Approval::get_by_id(conn, approval_id)?;
            approval.amount
        } else {
            0
        };
        if ticket.amount <= cur_money_limit {
            // Self::update_approval_id(conn, ticket_id, None)?;
            diesel::update(ticket_info::table)
                .filter(ticket_info::id.eq(ticket_id))
                .set(UpdateTicket {
                    last_approver_id: Some(cur_appover_id),
                    amount: None,
                    state: None,
                    approval_id: None,
                    receiver_id: None,
                })
                .execute(conn)?;
            return Ok(None);
        }
        let new_approval = Approval::get_next_by_company(conn, company_name, cur_money_limit)?;
        if let Some(new_approval) = new_approval {
            let a = Self::update_approval_id(conn, ticket_id, Some(new_approval.id))?;
            Ok(Some(a))
        } else {
            Self::update_approval_id(conn, ticket_id, None)?;
            Ok(None)
        }
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
