use crate::models::employee::Employee;
use diesel::{prelude::*, query_dsl::methods::FilterDsl};
use serde::{Deserialize, Serialize};

use crate::{
    error::AppError,
    schema::{employee_operation_info, operation_info},
};

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

    pub fn get_by_id(conn: &mut PgConnection, id: i32) -> Result<Department, AppError> {
        let department = operation_info::table
            .find(id)
            .get_result::<Department>(conn)?;
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

    pub fn mget_by_system(
        conn: &mut PgConnection,
        system_id: i32,
    ) -> Result<Vec<Department>, AppError> {
        let a = FilterDsl::filter(
            operation_info::table,
            operation_info::system_id.eq(system_id),
        )
        .get_results(conn)?;
        Ok(a)
    }
}

#[derive(Debug, Clone, Identifiable, Selectable, Queryable, Associations)]
#[diesel(belongs_to(Employee))]
#[diesel(belongs_to(Department))]
#[diesel(table_name = employee_operation_info)]
#[diesel(primary_key(employee_id, department_id))]
pub struct EmployeeWithDepartments {
    pub employee_id: i32,
    pub department_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = employee_operation_info)]
pub struct InsertEmployeeWithDepartments {
    pub employee_id: i32,
    pub department_id: i32,
}

impl EmployeeWithDepartments {
    pub fn create(
        conn: &mut PgConnection,
        employee_id: i32,
        department_id: i32,
    ) -> Result<Self, AppError> {
        let a = diesel::insert_into(employee_operation_info::table)
            .values(InsertEmployeeWithDepartments {
                employee_id,
                department_id,
            })
            .get_result(conn)?;
        Ok(a)
    }

    pub fn mget_department_id_by_employee_id(
        conn: &mut PgConnection,
        employee_id: i32,
    ) -> Result<Vec<i32>, AppError> {
        let a = FilterDsl::filter(
            employee_operation_info::table,
            employee_operation_info::employee_id.eq(employee_id),
        )
        .select(employee_operation_info::department_id)
        .get_results::<i32>(conn)?;
        Ok(a)
    }
}
