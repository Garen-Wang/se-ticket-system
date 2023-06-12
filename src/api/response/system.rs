use serde::Serialize;

use crate::models::{account::Account, department::Department, employee::Employee, system::System};

#[derive(Debug, Clone, Serialize)]
pub struct CreateSystemResponse {
    pub id: i32,
    pub name: String,
    pub departments: Vec<DepartmentResponse>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DepartmentResponse {
    pub id: i32,
    pub department_name: String,
}

impl From<Department> for DepartmentResponse {
    fn from(value: Department) -> Self {
        Self {
            id: value.id,
            department_name: value.department_name,
        }
    }
}

impl From<(System, Vec<Department>)> for CreateSystemResponse {
    fn from((system, departments): (System, Vec<Department>)) -> Self {
        Self {
            id: system.id,
            name: system.name,
            departments: departments.into_iter().map(|d| d.into()).collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateEmployeeResponse {
    pub employee_id: i32,
    pub name: String,
    pub account: String,
    pub phone: String,
    pub approval_id: Option<i32>,
    pub account_type: i16,
}

impl From<(Employee, Account)> for CreateEmployeeResponse {
    fn from((employee, account): (Employee, Account)) -> Self {
        Self {
            employee_id: employee.id,
            name: employee.name,
            account: account.account_name,
            phone: employee.phone,
            approval_id: employee.approval_id,
            account_type: account.account_type,
        }
    }
}
