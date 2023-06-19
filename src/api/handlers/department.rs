use actix_web::{web, HttpRequest, HttpResponse};

use crate::{
    api::response::approval::MGetDepartmentBySystemResponse,
    error::AppError,
    models::department::Department,
    utils::{auth::get_current_system, response::CommonResponse},
    AppState,
};

pub async fn list_departments(
    app_state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let system = get_current_system(&req, &mut conn)?;
    let departments = Department::mget_by_system(&mut conn, system.id)?;
    let resp = MGetDepartmentBySystemResponse {
        departments: departments.into_iter().map(|x| x.department_name).collect(),
    };
    Ok(HttpResponse::Ok().json(CommonResponse::from(resp)))
}
