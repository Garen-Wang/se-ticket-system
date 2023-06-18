use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{error::AppError, schema::employee_info};

#[derive(Debug, Clone, Serialize, Deserialize, Selectable, Identifiable, Queryable)]
#[diesel(table_name = employee_info)]
pub struct Employee {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub position: Option<String>,
    pub phone: String,
    pub state: i16,
    pub approval_id: Option<i32>,
    pub system_id: i32,
    pub sex: i16,
}

#[derive(Insertable)]
#[diesel(table_name = employee_info)]
pub struct InsertEmployee<'a> {
    pub name: &'a str,
    pub age: i32,
    pub position: Option<&'a str>,
    pub phone: &'a str,
    pub approval_id: Option<i32>,
    pub system_id: i32,
    pub sex: i16,
}

impl Employee {
    pub fn create(
        conn: &mut PgConnection,
        insert_employee: InsertEmployee,
    ) -> Result<Employee, AppError> {
        let employee: Employee = diesel::insert_into(employee_info::table)
            .values(insert_employee)
            .get_result(conn)?;
        Ok(employee)
    }

    pub fn get_by_id(conn: &mut PgConnection, id: i32) -> Result<Employee, AppError> {
        let employee: Employee = employee_info::table.find(id).first(conn)?;
        Ok(employee)
    }

    pub fn update_state(
        conn: &mut PgConnection,
        id: i32,
        state: i16,
    ) -> Result<Employee, AppError> {
        let employee = diesel::update(employee_info::table.filter(employee_info::id.eq(id)))
            .set(employee_info::state.eq(state))
            .get_result(conn)?;
        Ok(employee)
    }
}
