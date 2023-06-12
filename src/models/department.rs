use diesel::{prelude::*, query_dsl::methods::FilterDsl};
use serde::{Deserialize, Serialize};

use crate::{error::AppError, schema::operation_info};

#[derive(Debug, Clone, Serialize, Deserialize, Selectable, Identifiable, Queryable)]
#[diesel(table_name = operation_info)]
pub struct Department {
    pub id: i32,
    pub department_name: String,
    pub system_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = operation_info)]
pub struct InsertDepartment<'a> {
    pub department_name: &'a str,
    pub system_id: i32,
}

impl Department {
    pub fn create(
        conn: &mut PgConnection,
        insert_department: InsertDepartment,
    ) -> Result<Department, AppError> {
        let department = diesel::insert_into(operation_info::table)
            .values(insert_department)
            .get_result(conn)?;
        Ok(department)
    }

    pub fn get_by_name(
        conn: &mut PgConnection,
        name: &str,
        system_id: i32,
    ) -> Result<Department, AppError> {
        let target = FilterDsl::filter(
            operation_info::table,
            operation_info::system_id
                .eq(system_id)
                .and(operation_info::department_name.eq(name)),
        )
        .limit(1)
        .first(conn)?;
        Ok(target)
    }
}
