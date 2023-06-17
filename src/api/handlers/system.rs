use actix_web::{web, HttpRequest, HttpResponse};
use passwords::PasswordGenerator;

use crate::{
    api::{
        request::system::{CreateSystemRequest, RegisterRequest},
        response::system::{CreateEmployeeResponse, CreateSystemResponse},
    },
    error::{new_ok_error, AppError},
    models::{
        account::Account,
        approval::{Approval, InsertApproval},
        department::{Department, InsertDepartment},
        employee::{Employee, InsertEmployee},
        system::System,
    },
    utils::{
        auth::{get_current_system, is_super_admin, is_system_admin},
        constant::{ACCOUNT_TYPE_ADMIN, APPROVAL_ID_ADMIN},
        response::CommonResponse,
    },
    AppState,
};

pub async fn create_system(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<CreateSystemRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    if is_super_admin(&req, &mut conn)? {
        if form.levels.len() <= 0 {
            return Err(new_ok_error("至少要有一个审批层级"));
        }
        let system = System::create(&mut conn, &form.name)?;
        let employee = Employee::create(
            &mut conn,
            InsertEmployee {
                name: "管理员",
                age: 0,
                position: Some("管理员"),
                phone: "12312341234",
                approval_id: Some(APPROVAL_ID_ADMIN),
                system_id: system.id,
            },
        )?;
        let pg = PasswordGenerator {
            length: 8,
            numbers: true,
            lowercase_letters: true,
            uppercase_letters: true,
            exclude_similar_characters: true,
            symbols: true,
            spaces: false,
            strict: true,
        };
        let (account, _) = Account::register(
            &mut conn,
            employee.id,
            &format!("{}_admin", form.name),
            &pg.generate_one().unwrap(),
            ACCOUNT_TYPE_ADMIN,
        )?;
        let system = System::set_admin_account_id(&mut conn, system.id, account.id)?;
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
                    amount: level.money_limit,
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
                        amount: level.money_limit,
                        company: Some(&special_level.name),
                        system_id: system.id,
                    },
                )?;
            }
        }
        let resp = CreateSystemResponse::from((system, departments));
        Ok(HttpResponse::Ok().json(CommonResponse::from(resp)))
    } else {
        Err(new_ok_error("不是超级管理员，没有权限创建系统"))
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
    let system_id = form.system_id;
    let app_error = new_ok_error("没有权限创建新帐号");
    if is_super_admin(&req, &mut conn)? {
    } else if is_system_admin(&req, &mut conn)? {
        let system = get_current_system(&req, &mut conn)?;
        if system.id != system_id {
            return Err(app_error);
        }
    } else {
        return Err(app_error);
    }
    let approval_id = match &form.approval_name {
        Some(approval_name) => {
            Approval::get_by_name(&mut conn, system_id, &approval_name)?.map(|x| x.id)
        }
        None => None,
    };
    let employee = Employee::create(
        &mut conn,
        InsertEmployee {
            name: &form.name,
            age: form.age,
            position: form.position.as_ref().map(|x| &**x),
            phone: &form.phone,
            approval_id,
            system_id: form.system_id,
        },
    )?;
    let (account, _) = Account::register(
        &mut conn,
        employee.id,
        &form.account,
        &form.password,
        form.account_type,
    )?;
    let resp = CreateEmployeeResponse::from((employee, account));
    Ok(HttpResponse::Ok().json(CommonResponse::from(resp)))
}
