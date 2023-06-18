use actix_web::{web, HttpRequest, HttpResponse};

use crate::{
    api::{
        request::approval::{ApproveRejectTicketRequest, MGetApprovalLevelByCompanyRequest},
        response::approval::{MGetApprovalLevelByCompanyResponse, MGetDepartmentBySystemResponse},
    },
    error::{new_ok_error, AppError},
    models::{approval::Approval, department::Department, ticket::Ticket},
    utils::{
        auth::{get_current_employee, get_current_system},
        constant::{TICKET_STATE_CLOSED, TICKET_STATE_OPEN},
        response::{new_ok_response, CommonResponse},
    },
    AppState,
};

// 列出所有要审批的工单
pub async fn list_approving_tickets(
    app_state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(new_ok_response("哈哈")))
    // Ticket::mget_approving_ticket_by_employee_id
}

// 审批一个工单
pub async fn approve_ticket(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Query<ApproveRejectTicketRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    // let employee = get_current_employee(&req, &mut conn)?;
    // if employee.approval_id.is_none() {
    //     Err(new_ok_error("你不是审批人"))
    // } else {
    Ticket::update_state(&mut conn, form.ticket_id, TICKET_STATE_OPEN)?;
    Ok(HttpResponse::Ok().json(new_ok_response("已通过")))
    // }
}

// 拒绝一个工单
pub async fn reject_ticket(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Query<ApproveRejectTicketRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    // let employee = get_current_employee(&req, &mut conn)?;
    // if employee.approval_id.is_none() {
    //     Err(new_ok_error("你不是审批人"))
    // } else {
    Ticket::update_state(&mut conn, form.ticket_id, TICKET_STATE_CLOSED)?;
    Ok(HttpResponse::Ok().json(new_ok_response("已驳回")))
    // }
}

pub async fn get_approval_levels_by_company(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Query<MGetApprovalLevelByCompanyRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let system = get_current_system(&req, &mut conn)?;
    let company_name = &form.company;
    let mut approvals = Approval::mget_by_company(&mut conn, system.id, company_name)?;
    if approvals.len() > 0 {
    } else {
        approvals = Approval::mget_default(&mut conn, system.id)?;
    }
    let resp = MGetApprovalLevelByCompanyResponse {
        approval_names: approvals.into_iter().map(|x| x.approval_name).collect(),
    };
    Ok(HttpResponse::Ok().json(CommonResponse::from(resp)))
}

// 找不到位置，乱放了
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
