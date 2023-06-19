use chrono::Utc;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    error::{new_ok_error, AppError},
    schema::account_info,
    utils,
};

use super::employee::Employee;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Selectable)]
#[diesel(table_name = account_info)]
pub struct Account {
    pub id: i32,
    pub employee_id: i32,
    pub account_name: String,
    pub password_hash: String,
    pub account_type: i16,
}

#[derive(Insertable)]
#[diesel(table_name = account_info)]
pub struct InsertAccount<'a> {
    pub employee_id: i32,
    pub account_name: &'a str,
    pub password_hash: &'a str,
    pub account_type: i16,
}

impl Account {
    pub fn generate_token(&self) -> Result<String, AppError> {
        let now = Utc::now().timestamp();
        let token = utils::token::generate_token(self.id, now)?;
        Ok(token)
    }
}

// static methods
impl Account {
    pub fn register(
        conn: &mut PgConnection,
        employee_id: i32,
        account_name: &str,
        naive_password: &str,
        account_type: i16,
    ) -> Result<(Account, String), AppError> {
        // 审批职位 Option
        // 所属部门 Option
        // 职位 required
        // gender
        // real name
        // account
        // password
        // type: (管理员)，审批人，运维，报表查看者，申请人, 0, 1, 2, 3, 4
        // 能审批不一定能查看报表，能查看爆表一定能审批
        let encrypted_password = bcrypt::hash(naive_password, bcrypt::DEFAULT_COST)?;

        let insert_account = InsertAccount {
            employee_id,
            account_name,
            password_hash: &encrypted_password,
            account_type,
        };

        let _employee = Employee::get_by_id(conn, employee_id)?;

        let account: Account = diesel::insert_into(account_info::table)
            .values(insert_account)
            .get_result(conn)?;

        let token = account.generate_token()?;
        Ok((account, token))
    }

    pub fn login(
        conn: &mut PgConnection,
        account_name: &str,
        naive_password: &str,
    ) -> Result<(Account, String), AppError> {
        let account: Account = account_info::table
            .filter(account_info::account_name.eq(account_name))
            .limit(1)
            .first(conn)?;
        let a = bcrypt::verify(naive_password, &account.password_hash)?;
        if a {
            let token = account.generate_token()?;
            Ok((account, token))
        } else {
            Err(new_ok_error("登录失败"))
        }
    }

    pub fn find(conn: &mut PgConnection, id: i32) -> Result<Account, AppError> {
        let account = account_info::table.find(id).first(conn)?;
        Ok(account)
    }

    pub fn find_by_name(conn: &mut PgConnection, account_name: &str) -> Result<Account, AppError> {
        let account: Account = account_info::table
            .filter(account_info::account_name.eq(account_name))
            .limit(1)
            .first(conn)?;
        Ok(account)
    }

    pub fn get_all(conn: &mut PgConnection) -> Result<Vec<Account>, AppError> {
        let accounts = account_info::table.get_results(conn)?;
        Ok(accounts)
    }
}
