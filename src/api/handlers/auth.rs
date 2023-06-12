use crate::utils::auth::get_current_user;
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
