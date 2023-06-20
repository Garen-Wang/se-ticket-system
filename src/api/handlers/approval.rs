use actix_web::{web, HttpRequest, HttpResponse};

use crate::{
    api::{
        request::approval::{ApproveRejectTicketRequest, MGetApprovalLevelByCompanyRequest},
        response::approval::MGetApprovalLevelByCompanyResponse,
    },
    error::{new_ok_error, AppError},
    models::{
        approval::{Approval, ApprovalWithTicket},
        ticket::Ticket,
    },
    utils::{
        auth::{get_current_employee, get_current_system},
        constant::{
            APPROVE_RESULT_APPROVED, APPROVE_RESULT_REJECTED, TICKET_STATE_OPEN,
            TICKET_STATE_REJECTED,
        },
        response::{new_ok_response, CommonResponse},
    },
    AppState,
};

// 审批一个工单
pub async fn approve_ticket(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Query<ApproveRejectTicketRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let employee = get_current_employee(&req, &mut conn)?;
    if let Some(approval_id) = employee.approval_id {
        ApprovalWithTicket::create(
            &mut conn,
            form.ticket_id,
            approval_id,
            employee.id,
            APPROVE_RESULT_APPROVED,
        )?;
        if !Ticket::update_next_current_approval_id(
            &mut conn,
            form.ticket_id,
            employee.company_name,
            employee.id,
        )? {
            // 如果能找到下一个审批的人，就还是审批状态
            // 如果没有，就通过
            Ticket::open(&mut conn, form.ticket_id)?;
        }
        Ok(HttpResponse::Ok().json(new_ok_response("已通过")))
    } else {
        Err(new_ok_error("你不是审批人"))
    }
}

// 拒绝一个工单
pub async fn reject_ticket(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Query<ApproveRejectTicketRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let employee = get_current_employee(&req, &mut conn)?;
    if let Some(approval_id) = employee.approval_id {
        ApprovalWithTicket::create(
            &mut conn,
            form.ticket_id,
            approval_id,
            employee.id,
            APPROVE_RESULT_REJECTED,
        )?;
        Ticket::reject(&mut conn, form.ticket_id)?;
        Ok(HttpResponse::Ok().json(new_ok_response("已驳回")))
    } else {
        Err(new_ok_error("你不是审批人"))
    }
}

pub async fn get_approval_levels_by_company(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Query<MGetApprovalLevelByCompanyRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let system = get_current_system(&req, &mut conn)?;
    let company_name = if form.company.len() > 0 {
        Some(form.company.clone())
    } else {
        None
    };
    let mut approvals = Approval::mget_by_company(&mut conn, system.id, company_name)?;
    if approvals.len() == 0 {
        approvals = Approval::mget_by_company(&mut conn, system.id, None)?;
    }
    let resp = MGetApprovalLevelByCompanyResponse {
        approval_names: approvals.into_iter().map(|x| x.approval_name).collect(),
    };
    Ok(HttpResponse::Ok().json(CommonResponse::from(resp)))
}
