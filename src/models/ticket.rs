use crate::{
    api::response::figure::{BarChartState, GetPieChartDataResponse, GetTableResponse, TableState},
    error::new_ok_error,
    models::department::Department,
    schema::apply_dev_info,
    utils::constant::{
        TICKET_STATE_APPROVING, TICKET_STATE_ASSIGNED, TICKET_STATE_CLOSED, TICKET_STATE_OPEN,
        TICKET_STATE_REJECTED, TICKET_STATE_UNAPPROVED,
    },
};
use chrono::{Duration, NaiveDateTime};
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
    pub approved_time: Option<NaiveDateTime>,
    pub system_id: i32,
    pub receiver_id: Option<i32>,
    pub received_time: Option<NaiveDateTime>,
    pub finished_time: Option<NaiveDateTime>,
    pub rejected_time: Option<NaiveDateTime>,
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
}

#[derive(AsChangeset)]
#[diesel(table_name = ticket_info)]
pub struct UpdateTicket {
    pub last_approver_id: Option<i32>,
    pub amount: Option<i32>,
    pub state: Option<i16>,
    pub approval_id: Option<Option<i32>>,
    pub receiver_id: Option<i32>,
    pub approved_time: Option<NaiveDateTime>,
    pub received_time: Option<NaiveDateTime>,
    pub finished_time: Option<NaiveDateTime>,
    pub rejected_time: Option<NaiveDateTime>,
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

    // Deprecated: 有问题，包含了历史审批
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

    // Deprecated: 有问题，包含了历史审批
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

    pub fn get_alarm_count(conn: &mut PgConnection, system_id: i32) -> Result<i64, AppError> {
        let now = chrono::Utc::now().naive_local();
        let expired_datetime = now - Duration::days(3);
        // let warning_datetime = now - Duration::days(2);
        let a = FilterDsl::filter(
            ticket_info::table,
            ticket_info::system_id
                .eq(system_id)
                .and(ticket_info::created_time.le(expired_datetime))
                .and(ticket_info::state.le(TICKET_STATE_ASSIGNED)),
        )
        .count()
        .get_result(conn)?;
        Ok(a)
    }

    pub fn mget_alarm_by_page(
        conn: &mut PgConnection,
        system_id: i32,
        size: i32,
        page: i32,
    ) -> Result<Vec<Self>, AppError> {
        let now = chrono::Utc::now().naive_local();
        let expired_datetime = now - Duration::days(2);
        let tickets = FilterDsl::filter(
            ticket_info::table,
            ticket_info::system_id
                .eq(system_id)
                .and(ticket_info::created_time.le(expired_datetime))
                .and(ticket_info::state.le(TICKET_STATE_ASSIGNED)),
        )
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
            // .set(ticket_info::receiver_id.eq(receiver_id))
            .set(UpdateTicket {
                last_approver_id: None,
                amount: None,
                state: Some(TICKET_STATE_ASSIGNED),
                approval_id: None,
                receiver_id: Some(receiver_id),
                approved_time: None,
                received_time: Some(chrono::Utc::now().naive_local()),
                finished_time: None,
                rejected_time: None,
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

    pub fn open(conn: &mut PgConnection, ticket_id: i32) -> Result<Ticket, AppError> {
        let ticket = diesel::update(ticket_info::table.find(ticket_id))
            // .set(ticket_info::state.eq(new_state))
            .set(UpdateTicket {
                last_approver_id: None,
                amount: None,
                state: Some(TICKET_STATE_OPEN),
                approval_id: None,
                receiver_id: None,
                approved_time: None,
                received_time: None,
                finished_time: None,
                rejected_time: None,
            })
            .get_result(conn)?;
        Ok(ticket)
    }

    pub fn reject(conn: &mut PgConnection, ticket_id: i32) -> Result<Ticket, AppError> {
        let ticket = diesel::update(ticket_info::table.find(ticket_id))
            .set(UpdateTicket {
                last_approver_id: None,
                amount: None,
                state: Some(TICKET_STATE_REJECTED),
                approval_id: None,
                receiver_id: None,
                approved_time: None,
                received_time: None,
                finished_time: None,
                rejected_time: Some(chrono::Utc::now().naive_local()),
            })
            .get_result(conn)?;
        Ok(ticket)
    }

    pub fn close(conn: &mut PgConnection, ticket_id: i32) -> Result<Ticket, AppError> {
        let ticket = diesel::update(ticket_info::table.find(ticket_id))
            .set(UpdateTicket {
                last_approver_id: None,
                amount: None,
                state: Some(TICKET_STATE_CLOSED),
                approval_id: None,
                receiver_id: None,
                approved_time: None,
                received_time: None,
                finished_time: Some(chrono::Utc::now().naive_local()),
                rejected_time: None,
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
                approved_time: None,
                received_time: None,
                finished_time: None,
                rejected_time: None,
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
                approved_time: Some(chrono::Utc::now().naive_local()),
                received_time: None,
                finished_time: None,
                rejected_time: None,
            })
            .get_result(conn)?;
        Ok(a)
    }

    pub fn init_next_current_approval_id(
        conn: &mut PgConnection,
        ticket_id: i32,
        company_name: Option<String>,
    ) -> Result<(), AppError> {
        let mut new_approval = Approval::get_next_by_company(conn, company_name, 0)?;
        if new_approval.is_none() {
            new_approval = Approval::get_next_by_company(conn, None, 0)?;
        }
        diesel::update(ticket_info::table)
            .filter(ticket_info::id.eq(ticket_id))
            .set(UpdateTicket {
                last_approver_id: None,
                amount: None,
                state: None,
                approval_id: Some(new_approval.map(|x| x.id)),
                receiver_id: None,
                approved_time: Some(chrono::Utc::now().naive_local()),
                received_time: None,
                finished_time: None,
                rejected_time: None,
            })
            .execute(conn)?;
        Ok(())
    }

    pub fn update_next_current_approval_id(
        conn: &mut PgConnection,
        ticket_id: i32,
        company_name: Option<String>,
        cur_appover_id: i32,
    ) -> Result<bool, AppError> {
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
                    approved_time: Some(chrono::Utc::now().naive_local()),
                    received_time: None,
                    finished_time: None,
                    rejected_time: None,
                })
                .execute(conn)?;
            Ok(false)
        } else {
            let new_approval = Approval::get_next_by_company(conn, company_name, cur_money_limit)?;
            let ret = new_approval.is_some();
            diesel::update(ticket_info::table)
                .filter(ticket_info::id.eq(ticket_id))
                .set(UpdateTicket {
                    last_approver_id: Some(cur_appover_id),
                    amount: None,
                    state: None,
                    approval_id: Some(new_approval.map(|x| x.id)),
                    receiver_id: None,
                    approved_time: Some(chrono::Utc::now().naive_local()),
                    received_time: None,
                    finished_time: None,
                    rejected_time: None,
                })
                .execute(conn)?;
            Ok(ret)
        }
    }

    pub fn get_pie_chart_data(
        conn: &mut PgConnection,
        system_id: i32,
        t: NaiveDateTime,
    ) -> Result<GetPieChartDataResponse, AppError> {
        let mut unapproved = 0;
        let mut approving = 0;
        let mut available = 0;
        let mut received = 0;
        let mut closed = 0;
        let mut rejected = 0;

        let tickets: Vec<Ticket> =
            FilterDsl::filter(ticket_info::table, ticket_info::system_id.eq(system_id))
                .get_results(conn)?;

        for ticket in tickets.into_iter() {
            match ticket.get_state_at_moment(t)? {
                Some(TICKET_STATE_UNAPPROVED) => {
                    unapproved += 1;
                }
                Some(TICKET_STATE_APPROVING) => {
                    approving += 1;
                }
                Some(TICKET_STATE_OPEN) => {
                    available += 1;
                }
                Some(TICKET_STATE_ASSIGNED) => {
                    received += 1;
                }
                Some(TICKET_STATE_CLOSED) => {
                    closed += 1;
                }
                Some(TICKET_STATE_REJECTED) => {
                    rejected += 1;
                }
                _ => {}
            }
        }
        Ok(GetPieChartDataResponse {
            unapproved,
            approving,
            available,
            received,
            closed,
            rejected,
        })
    }

    pub fn get_bar_chart_data(
        conn: &mut PgConnection,
        system_id: i32,
        t: NaiveDateTime,
        weekday: i32,
        period: Option<String>,
    ) -> Result<BarChartState, AppError> {
        let mut open = 0;
        let mut closed = 0;
        let tickets: Vec<Ticket> =
            FilterDsl::filter(ticket_info::table, ticket_info::system_id.eq(system_id))
                .get_results(conn)?;
        for ticket in tickets.into_iter() {
            match ticket.get_state_at_moment(t)? {
                Some(TICKET_STATE_UNAPPROVED)
                | Some(TICKET_STATE_APPROVING)
                | Some(TICKET_STATE_OPEN)
                | Some(TICKET_STATE_ASSIGNED) => {
                    open += 1;
                }
                Some(TICKET_STATE_CLOSED) | Some(TICKET_STATE_REJECTED) => {
                    closed += 1;
                }
                _ => {}
            }
        }
        Ok(BarChartState {
            weekday,
            period,
            open,
            closed,
        })
    }

    pub fn get_table_by_date(
        conn: &mut PgConnection,
        system_id: i32,
        ranges: Vec<i32>, // 审批钱数
        t: NaiveDateTime, // 时间
    ) -> Result<GetTableResponse, AppError> {
        let mut resp = vec![];
        for i in 0..(ranges.len() - 1) {
            let range = format!("{}-{}", ranges[i], ranges[i + 1]);
            let tickets: Vec<Ticket> = FilterDsl::filter(
                ticket_info::table,
                ticket_info::system_id
                    .eq(system_id)
                    .and(ticket_info::amount.between(ranges[i], ranges[i + 1])),
            )
            .get_results(conn)?;
            let mut open = 0;
            let mut closed = 0;
            for ticket in tickets.into_iter() {
                match ticket.get_state_at_moment(t)? {
                    Some(TICKET_STATE_UNAPPROVED)
                    | Some(TICKET_STATE_APPROVING)
                    | Some(TICKET_STATE_OPEN)
                    | Some(TICKET_STATE_ASSIGNED) => {
                        open += 1;
                    }
                    Some(TICKET_STATE_CLOSED) | Some(TICKET_STATE_REJECTED) => {
                        closed += 1;
                    }
                    _ => {}
                }
            }
            resp.push(TableState {
                range,
                open,
                closed,
            });
        }
        Ok(resp)
    }
}

impl Ticket {
    pub fn get_state_at_moment(&self, timestamp: NaiveDateTime) -> Result<Option<i16>, AppError> {
        if let Some(rejected_time) = self.rejected_time {
            if timestamp >= rejected_time {
                return Ok(Some(TICKET_STATE_REJECTED));
            }
        }
        if timestamp < self.created_time {
            Ok(None)
        } else if self.approved_time.is_none() {
            Ok(Some(TICKET_STATE_UNAPPROVED))
        } else {
            let approved_time = self.approved_time.unwrap();
            if timestamp < approved_time {
                Ok(Some(TICKET_STATE_APPROVING))
            } else if self.received_time.is_none() {
                Ok(Some(TICKET_STATE_OPEN))
            } else {
                let received_time = self.received_time.unwrap();
                if timestamp < received_time {
                    Ok(Some(TICKET_STATE_OPEN))
                } else if self.finished_time.is_none() {
                    Ok(Some(TICKET_STATE_ASSIGNED))
                } else {
                    let finished_time = self.finished_time.unwrap();
                    if timestamp < finished_time {
                        Ok(Some(TICKET_STATE_ASSIGNED))
                    } else {
                        Ok(Some(TICKET_STATE_CLOSED))
                    }
                }
            }
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
