use crate::error::new_ok_error;
use crate::models::employee::Employee;
use crate::models::system::System;
use crate::utils::auth::{get_current_system, get_current_user};
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
        let resp = AccountResponse::from((account, token, system.name));
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
    let resp = AccountResponse::from((account, token, system.name));
    Ok(HttpResponse::Ok().json(resp))
}
