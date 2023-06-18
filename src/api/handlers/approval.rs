use actix_web::{web, HttpRequest, HttpResponse};

use crate::{
    api::{
        request::approval::MGetApprovalLevelByCompanyRequest,
        response::approval::{MGetApprovalLevelByCompanyResponse, MGetDepartmentBySystemResponse},
    },
    error::{new_ok_error, AppError},
    models::{approval::Approval, department::Department},
    utils::{
        auth::get_current_system,
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
pub async fn approve_ticket() {}

// 拒绝一个工单
pub async fn reject_ticket() {}

pub async fn get_approval_levels_by_company(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Query<MGetApprovalLevelByCompanyRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let system = get_current_system(&req, &mut conn)?;
    let company_name = &form.company;
    let approvals = Approval::mget_by_company(&mut conn, system.id, company_name)?;
    if approvals.len() > 0 {
        let resp = MGetApprovalLevelByCompanyResponse {
            approval_names: approvals.into_iter().map(|x| x.approval_name).collect(),
        };
        Ok(HttpResponse::Ok().json(CommonResponse::from(resp)))
    } else {
        Err(new_ok_error("这个公司没有特殊审批配置"))
    }
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
