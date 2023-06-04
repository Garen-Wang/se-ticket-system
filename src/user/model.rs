use crate::{
    auth::token,
    error::AppError,
    schema::{account_info, employee_info},
};
use chrono::Utc;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Selectable, Identifiable, Queryable)]
#[diesel(table_name = employee_info)]
pub struct Employee {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub position: Option<String>,
    pub phone: String,
    pub state: i16,
    pub approval_level: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Selectable)]
#[diesel(table_name = account_info)]
pub struct User {
    pub id: i32,
    pub employee_id: i32,
    pub account_name: String,
    pub password_hash: String,
    pub account_type: i16,
}

#[derive(Insertable)]
#[diesel(table_name = account_info)]
pub struct InsertUser<'a> {
    pub employee_id: i32,
    pub account_name: &'a str,
    pub password_hash: &'a str,
    pub account_type: i16,
}

#[derive(AsChangeset)]
#[diesel(table_name = account_info)]
pub struct UpdateUser {
    pub password_hash: Option<String>,
    pub account_type: Option<i16>,
}

impl User {
    pub fn generate_token(&self) -> Result<String, AppError> {
        let now = Utc::now().timestamp();
        let token = token::generate_token(self.id, now)?;
        Ok(token)
    }
}

// static methods
impl User {
    pub fn register(
        conn: &mut PgConnection,
        account_name: &str,
        naive_password: &str,
        employee_id: i32,
        account_type: i16,
    ) -> Result<(User, String), AppError> {
        let encrypted_password = bcrypt::hash(naive_password, bcrypt::DEFAULT_COST)?;

        let insert_account = InsertUser {
            employee_id,
            account_name,
            password_hash: &encrypted_password,
            account_type,
        };

        let user: User = diesel::insert_into(account_info::table)
            .values(insert_account)
            .get_result(conn)?;

        let token = user.generate_token()?;
        Ok((user, token))
    }

    pub fn login(
        conn: &mut PgConnection,
        account_name: &str,
        naive_password: &str,
    ) -> Result<(User, String), AppError> {
        let account: User = account_info::table
            .filter(account_info::account_name.eq(account_name))
            .limit(1)
            .first(conn)?;
        let a = bcrypt::verify(naive_password, &account.password_hash)?;
        if a {
            let token = account.generate_token()?;
            Ok((account, token))
        } else {
            Err(AppError::InternalServerError)
        }
    }

    pub fn update(
        conn: &mut PgConnection,
        id: i32,
        changeset: UpdateUser,
    ) -> Result<User, AppError> {
        let target = account_info::table.filter(account_info::id.eq(id));
        let account = diesel::update(target).set(changeset).get_result(conn)?;
        Ok(account)
    }

    pub fn delete(conn: &mut PgConnection, id: i32) -> Result<usize, AppError> {
        let item = diesel::delete(account_info::table)
            .filter(account_info::id.eq(id))
            .execute(conn)?;
        Ok(item)
    }

    pub fn find(conn: &mut PgConnection, id: i32) -> Result<User, AppError> {
        let user = account_info::table.find(id).first(conn)?;
        Ok(user)
    }

    pub fn find_by_name(conn: &mut PgConnection, username: &str) -> Result<User, AppError> {
        use crate::schema::account_info::dsl::*;
        let user: User = account_info
            .filter(account_name.eq(username))
            .limit(1)
            .first(conn)?;
        Ok(user)
    }

    pub fn get_all(conn: &mut PgConnection) -> Result<Vec<User>, AppError> {
        let users = account_info::table.get_results(conn)?;
        Ok(users)
    }
}
