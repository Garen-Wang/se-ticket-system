use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    error::AppError,
    schema::{approval_info, approved_info},
};

use super::ticket::Ticket;

#[derive(Debug, Clone, Serialize, Deserialize, Selectable, Identifiable, Queryable)]
#[diesel(table_name = approval_info)]
pub struct Approval {
    pub approval_name: String, // 这个审批级别的名字
    pub amount: i32,           // 小于这个数的，我能批
    pub company: Option<String>,
    pub system_id: i32,
    pub id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = approval_info)]
pub struct InsertApproval<'a> {
    pub approval_name: &'a str,
    pub amount: i32,
    pub company: Option<&'a str>,
    pub system_id: i32,
}

impl Approval {
    pub fn create(
        conn: &mut PgConnection,
        insert_appoval: InsertApproval,
    ) -> Result<Approval, AppError> {
        let approval = diesel::insert_into(approval_info::table)
            .values(insert_appoval)
            .get_result(conn)?;
        Ok(approval)
    }

    pub fn get_by_id(conn: &mut PgConnection, id: i32) -> Result<Approval, AppError> {
        let approval = approval_info::table.find(id).get_result(conn)?;
        Ok(approval)
    }

    pub fn get_highest_by_amount(
        conn: &mut PgConnection,
        system_id: i32,
        amount: i32,
    ) -> Result<Self, AppError> {
        let approval = approval_info::table
            .filter(approval_info::system_id.eq(system_id))
            .filter(approval_info::amount.ge(amount))
            .order(approval_info::amount.desc())
            .limit(1)
            .get_result::<Approval>(conn)?;
        Ok(approval)
    }

    pub fn get_by_name(
        conn: &mut PgConnection,
        system_id: i32,
        approval_name: &str,
    ) -> Result<Option<Approval>, AppError> {
        let approval = approval_info::table
            .filter(approval_info::system_id.eq(system_id))
            .filter(approval_info::approval_name.eq(approval_name))
            .limit(1)
            .get_result::<Approval>(conn)
            .optional()?;
        Ok(approval)
    }

    pub fn mget_by_company(
        conn: &mut PgConnection,
        system_id: i32,
        company_name: &str,
    ) -> Result<Vec<Approval>, AppError> {
        let approvals = approval_info::table
            .filter(approval_info::system_id.eq(system_id))
            .filter(approval_info::company.eq(company_name))
            .get_results::<Approval>(conn)?;
        Ok(approvals)
    }

    pub fn mget_default(
        conn: &mut PgConnection,
        system_id: i32,
    ) -> Result<Vec<Approval>, AppError> {
        let approvals = approval_info::table
            .filter(approval_info::system_id.eq(system_id))
            .filter(approval_info::company.is_null())
            .get_results::<Approval>(conn)?;
        Ok(approvals)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Selectable, Identifiable, Queryable)]
#[diesel(table_name = approved_info)]
pub struct ApprovalWithTicket {
    pub ticket_id: i32,
    pub approval_id: i32,
    pub id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = approved_info)]
pub struct InsertApprovalWithTicket {
    pub ticket_id: i32,
    pub approval_id: i32,
}

impl ApprovalWithTicket {
    pub fn create(
        conn: &mut PgConnection,
        ticket_id: i32,
        approval_id: i32,
    ) -> Result<Self, AppError> {
        let a = diesel::insert_into(approved_info::table)
            .values(InsertApprovalWithTicket {
                ticket_id,
                approval_id,
            })
            .get_result(conn)?;
        Ok(a)
    }
}
