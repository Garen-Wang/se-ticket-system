use crate::api::request::auth::RegisterRequest;
use crate::models::employee::{Employee, InsertEmployee};
use crate::utils::auth::{get_current_system, get_current_user, is_super_admin, is_system_admin};
use crate::utils::response::CommonResponse;
use crate::{
    api::{request::auth::LoginRequest, response::auth::AccountResponse},
    error::AppError,
    models::account::Account,
    AppState,
};
use actix_web::{web, HttpRequest, HttpResponse};

pub async fn login(
    app_state: web::Data<AppState>,
    form: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let (account, token) = Account::login(&mut conn, &form.account, &form.password)?;
    let resp = AccountResponse::from((account, token));
    Ok(HttpResponse::Ok().json(CommonResponse::from(resp)))
}

pub async fn get_myself(
    // app_state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    // let mut conn = app_state.conn()?;
    let account = get_current_user(&req)?;
    let token = account.generate_token()?;
    let resp = AccountResponse::from((account, token));
    Ok(HttpResponse::Ok().json(resp))
}

// 如果是超级管理员，只要 system id 合法，随便创建
// 如果是系统管理员，只能在自己的 system id 下创建
// 否则，没有权限
pub async fn register(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<RegisterRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let system_id = form.system_id;
    let app_error = AppError::Ok(crate::error::ErrMessage {
        error: "no permission to create account".into(),
    });
    if is_super_admin(&req, &mut conn)? {
    } else if is_system_admin(&req, &mut conn)? {
        let system = get_current_system(&req, &mut conn)?;
        if system.id != system_id {
            return Err(app_error);
        }
    } else {
        return Err(app_error);
    }
    let employee = Employee::create(
        &mut conn,
        InsertEmployee {
            name: &form.name,
            age: form.age,
            position: form.position.as_ref().map(|x| &**x),
            phone: &form.phone,
            approval_id: form.approval_id,
            system_id: form.system_id,
        },
    )?;
    let (user, token) = Account::register(
        &mut conn,
        employee.id,
        &form.account,
        &form.password,
        form.account_type,
    )?;
    let resp = AccountResponse::from((user, token));
    Ok(HttpResponse::Ok().json(CommonResponse::from(resp)))
}
