use actix_web::{web, HttpRequest, HttpResponse};

use crate::{
    auth::auth::get_current_user, common::response::CommonResponse, error::AppError, AppState,
};

use super::{
    model::User,
    request::{LoginRequest, RegisterRequest},
    response::UserResponse,
};

pub async fn login(
    app_state: web::Data<AppState>,
    form: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let (user, token) = User::login(&mut conn, &form.username, &form.password)?;
    let resp = UserResponse::from((user, token));
    Ok(HttpResponse::Ok().json(CommonResponse::from(resp)))
}

pub async fn register(
    app_state: web::Data<AppState>,
    form: web::Json<RegisterRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let (user, token) = User::register(
        &mut conn,
        form.employee_id,
        &form.username,
        &form.username,
        form.account_type,
        form.system_id,
    )?;
    let resp = UserResponse::from((user, token));
    Ok(HttpResponse::Ok().json(CommonResponse::from(resp)))
}

pub async fn get_myself(req: HttpRequest) -> Result<HttpResponse, AppError> {
    let user = get_current_user(&req)?;
    let token = user.generate_token()?;
    let resp = UserResponse::from((user, token));
    Ok(HttpResponse::Ok().json(resp))
}
