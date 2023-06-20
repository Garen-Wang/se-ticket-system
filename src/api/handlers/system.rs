use actix_web::{web, HttpRequest, HttpResponse};

use crate::{
    api::{
        request::system::{CreateSystemRequest, RegisterRequest},
        response::system::{CreateEmployeeResponse, CreateSystemResponse},
    },
    error::{new_ok_error, AppError},
    models::{
        account::Account,
        approval::{Approval, InsertApproval},
        department::{Department, EmployeeWithDepartments, InsertDepartment},
        employee::{Employee, InsertEmployee},
        system::System,
    },
    utils::{
        auth::{get_current_system, is_system_admin},
        constant::{SEX_FEMALE, SEX_MALE},
        response::CommonResponse,
    },
    AppState,
};

pub async fn initialize_system(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<CreateSystemRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    if is_system_admin(&req, &mut conn)? {
        let system = get_current_system(&req, &mut conn)?;
        if system.initialized != 0 {
            return Err(new_ok_error("系统已经被初始化"));
        }
        if form.levels.len() <= 0 {
            return Err(new_ok_error("至少要有一个审批层级"));
        }
        let system = System::set_name(&mut conn, system.id, form.name.clone())?;
        let mut departments = vec![];
        for dep_item in form.departments.iter() {
            let department = Department::create(
                &mut conn,
                InsertDepartment {
                    department_name: &dep_item.name,
                    system_id: system.id,
                },
            )?;
            departments.push(department);
        }
        for level in form.levels.iter() {
            Approval::create(
                &mut conn,
                InsertApproval {
                    approval_name: &level.name,
                    amount: level.money_limit.parse::<i32>().unwrap(),
                    company: None,
                    system_id: system.id,
                },
            )?;
        }
        for special_level in form.special_levels.iter() {
            for level in special_level.special_level.iter() {
                Approval::create(
                    &mut conn,
                    InsertApproval {
                        approval_name: &level.name,
                        amount: level.money_limit.parse::<i32>().unwrap(),
                        company: Some(&special_level.name),
                        system_id: system.id,
                    },
                )?;
            }
        }
        System::set_initialized(&mut conn, system.id, 1)?;
        let resp = CreateSystemResponse::from((system, departments));
        Ok(HttpResponse::Ok().json(CommonResponse::from(resp)))
    } else {
        Err(new_ok_error("不是管理员，没有权限创建系统"))
    }
}

// 如果是超级管理员，只要 system id 合法，随便创建
// 如果是系统管理员，只能在自己的 system id 下创建
// 否则，没有权限
pub async fn create_employee(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<RegisterRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let app_error = new_ok_error("没有权限创建新帐号");
    if !is_system_admin(&req, &mut conn)? {
        return Err(app_error);
    }
    let system = get_current_system(&req, &mut conn)?;
    let approval_id = if form.approval_name.len() > 0 {
        Approval::get_by_name(&mut conn, system.id, &form.approval_name)?.map(|x| x.id)
    } else {
        None
    };
    let sex = match form.sex.as_str() {
        "0" | "female" | "woman" | "女" => SEX_FEMALE,
        "1" | "male" | "man" | "男" => SEX_MALE,
        _ => {
            return Err(app_error);
        }
    };
    let employee = Employee::create(
        &mut conn,
        InsertEmployee {
            name: &form.name,
            age: form.age.parse().unwrap(),
            position: if form.position.len() > 0 {
                Some(&form.position)
            } else {
                None
            },
            phone: &form.phone_number.trim(),
            approval_id,
            system_id: system.id,
            sex,
            company_name: if form.company.len() > 0 {
                Some(form.company.as_str())
            } else {
                None
            },
        },
    )?;
    let (account, _) = Account::register(
        &mut conn,
        employee.id,
        &form.account,
        &form.password,
        form.account_type,
    )?;
    for dep in form.departments.iter() {
        let department = Department::get_by_name(&mut conn, dep, system.id)?;
        log::info!(
            "create, employee_id: {}, department_id: {}",
            employee.id,
            department.id
        );
        EmployeeWithDepartments::create(&mut conn, employee.id, department.id)?;
    }
    let resp = CreateEmployeeResponse::from((employee, account));
    Ok(HttpResponse::Ok().json(CommonResponse::from(resp)))
}
