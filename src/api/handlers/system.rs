use actix_web::{web, HttpResponse};

use crate::{api::request::system::CreateSystemRequest, error::AppError, AppState};

pub async fn create_system(
    app_state: web::Data<AppState>,
    form: web::Json<CreateSystemRequest>,
) -> Result<HttpResponse, AppError> {
    unimplemented!()
}
