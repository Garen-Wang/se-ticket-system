use crate::api::request::auth::RegisterAdminRequest;
use crate::api::response::auth::RegisterAdminResponse;
use crate::error::new_ok_error;
use crate::models::employee::{Employee, InsertEmployee};
use crate::models::system::System;
use crate::utils::auth::{get_current_system, get_current_user};
use crate::utils::constant::{ACCOUNT_TYPE_ADMIN, SEX_MALE};
use crate::utils::response::CommonResponse;
use crate::{
    api::{request::auth::LoginRequest, response::auth::AccountResponse},
    error::AppError,
    models::account::Account,
    AppState,
};
use actix_web::{web, HttpRequest, HttpResponse};

// 前端接口需求 1
// DONE
pub async fn login(
    app_state: web::Data<AppState>,
    form: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    if let Ok((account, token)) = Account::login(&mut conn, &form.account, &form.password) {
        let employee = Employee::get_by_id(&mut conn, account.employee_id)?;
        let system = System::get_by_id(&mut conn, employee.system_id)?;
        let system_name = if system.initialized == 0 {
            None
        } else {
            Some(system.name)
        };
        let resp = AccountResponse::from((account, token, system_name));
        Ok(HttpResponse::Ok().json(CommonResponse::from(resp)))
    } else {
        Err(new_ok_error("登录失败"))
    }
}

pub async fn get_myself(
    app_state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let account = get_current_user(&req)?;
    let token = account.generate_token()?;
    let system = get_current_system(&req, &mut conn)?;
    let system_name = if system.initialized == 0 {
        None
    } else {
        Some(system.name)
    };
    let resp = AccountResponse::from((account, token, system_name));
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn register_admin(
    app_state: web::Data<AppState>,
    form: web::Json<RegisterAdminRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let system = System::create(&mut conn, &format!("{}", chrono::Utc::now().timestamp()))?;
    let employee = Employee::create(
        &mut conn,
        InsertEmployee {
            name: &format!("系统管理员_{}", system.id),
            age: 0,
            position: Some("管理员"),
            phone: "11111111111",
            approval_id: None,
            system_id: system.id,
            sex: SEX_MALE,
            company_name: None,
        },
    )?;
    let (account, token) = Account::register(
        &mut conn,
        employee.id,
        &form.account,
        &form.password,
        ACCOUNT_TYPE_ADMIN,
    )?;

    System::set_admin_account_id(&mut conn, system.id, account.id)?;

    let resp = RegisterAdminResponse {
        system_id: system.id,
        token,
    };
    Ok(HttpResponse::Ok().json(CommonResponse::from(resp)))
}
