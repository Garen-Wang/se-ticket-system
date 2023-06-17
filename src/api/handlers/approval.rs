use actix_web::{web, HttpRequest, HttpResponse};

use crate::{error::AppError, utils::response::new_ok_response, AppState};

// 列出所有要审批的工单
pub async fn list_approving_tickets(
    app_state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(new_ok_response("哈哈")))
    // Ticket::mget_approving_ticket_by_employee_id
}

// 审批一个工单
pub async fn approve_ticket() {}

// 拒绝一个工单
pub async fn reject_ticket() {}
