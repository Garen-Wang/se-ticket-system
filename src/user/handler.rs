use actix_web::{web, HttpRequest, HttpResponse};

use crate::{auth::auth::get_current_user, error::AppError, AppState};

use super::{
    model::{UpdateUser, User},
    request::{LoginRequest, RegisterRequest, UpdateRequest},
    response::UserResponse,
};

pub async fn login(
    app_state: web::Data<AppState>,
    form: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let (user, token) = User::login(&mut conn, &form.user.username, &form.user.password)?;
    let resp = UserResponse::from((user, token));
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn register(
    app_state: web::Data<AppState>,
    form: web::Json<RegisterRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let (user, token) = User::register(
        &mut conn,
        form.user.employee_id,
        &form.user.username,
        &form.user.username,
        form.user.account_type,
        form.user.system_id,
    )?;
    let resp = UserResponse::from((user, token));
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn update(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<UpdateRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let current_user = get_current_user(&req)?;
    let password = form.user.password.clone();

    let updated_user = User::update(
        &mut conn,
        current_user.id,
        UpdateUser {
            password_hash: password.map(|p| bcrypt::hash(p, bcrypt::DEFAULT_COST).unwrap()),
            account_type: form.user.account_type,
        },
    )?;

    let token = updated_user.generate_token()?;
    let resp = UserResponse::from((updated_user, token));
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn get_myself(req: HttpRequest) -> Result<HttpResponse, AppError> {
    let user = get_current_user(&req)?;
    let token = user.generate_token()?;
    let resp = UserResponse::from((user, token));
    Ok(HttpResponse::Ok().json(resp))
}
