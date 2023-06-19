use std::vec;

use actix_web::{web, HttpRequest, HttpResponse};
use chrono::Utc;

use crate::{
    api::{
        request::ticket::{
            CreateAssistTicketRequest, CreateTicketRequest, FinishTicketRequest,
            GetTicketByIDRequest, MGetTicketByPageRequest, TakeTicketRequest,
        },
        response::ticket::{
            AvailableTicketsResponse, CurrentTicketResponse, HistoryTicketsResponse,
            MGetOverviewByPageResponse, PCTicketResponse, TicketOverviewResponse,
        },
    },
    error::{new_ok_error, AppError},
    models::{
        assist::{Assist, AssistWithDepartments, AssistWithEmployees, InsertAssist},
        department::{Department, EmployeeWithDepartments},
        employee::Employee,
        ticket::{Fund, InsertFund, InsertTicket, Ticket, TicketWithDepartments},
    },
    utils::{
        auth::{get_current_employee, get_current_system, is_super_admin},
        constant::{
            EMPLOYEE_STATUS_AVAILABLE, EMPLOYEE_STATUS_UNAVAILABLE, TICKET_STATE_ASSIGNED,
            TICKET_STATE_CLOSED, TICKET_STATE_OPEN,
        },
        response::{new_ok_response, CommonResponse},
    },
    AppState,
};

pub async fn get_tickets_by_page(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Query<MGetTicketByPageRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let system = get_current_system(&req, &mut conn)?;
    let employee = get_current_employee(&req, &mut conn)?;
    if let Some(approval_id) = employee.approval_id {
        let count = Ticket::get_approving_count(&mut conn, system.id, approval_id)?;
        let tickets = Ticket::mget_approving_by_page(
            &mut conn,
            system.id,
            approval_id,
            form.size,
            form.page,
        )?;

        let mut ts = vec![];
        for ticket in tickets.into_iter() {
            let employee = Employee::get_by_id(&mut conn, ticket.creator_id)?;
            let funds = Fund::mget_by_ticket_id(&mut conn, ticket.id)?;
            ts.push(TicketOverviewResponse::from((ticket, employee, funds)));
        }
        Ok(
            HttpResponse::Ok().json(CommonResponse::from(MGetOverviewByPageResponse {
                total: count,
                tickets: ts,
            })),
        )
    } else {
        Err(new_ok_error("你不是审批人"))
    }
}

pub async fn get_history_tickets_by_page(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Query<MGetTicketByPageRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let employee = get_current_employee(&req, &mut conn)?;
    if let Some(approval_id) = employee.approval_id {
        let count = Ticket::get_history_count(&mut conn, approval_id, employee.id)?;
        // let approval = Approval::get_by_id(&mut conn, approval_id)?;
        let tickets = Ticket::mget_history_by_approver(
            &mut conn,
            approval_id,
            employee.id,
            form.size,
            form.page,
        )?;

        let mut ts = vec![];
        for ticket in tickets.into_iter() {
            let employee = Employee::get_by_id(&mut conn, ticket.creator_id)?;
            let funds = Fund::mget_by_ticket_id(&mut conn, ticket.id)?;
            ts.push(TicketOverviewResponse::from((ticket, employee, funds)));
        }
        Ok(
            HttpResponse::Ok().json(CommonResponse::from(MGetOverviewByPageResponse {
                total: count,
                tickets: ts,
            })),
        )
    } else {
        Err(new_ok_error("你不是审批人"))
    }
}

pub async fn create_ticket(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<CreateTicketRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    if is_super_admin(&req, &mut conn)? {
        return Err(new_ok_error("超级管理员不能创建工单"));
    }

    let employee = get_current_employee(&req, &mut conn)?;
    let system = get_current_system(&req, &mut conn)?;

    let insert_ticket = InsertTicket {
        creator_id: employee.id,
        title: &form.title,
        amount: 0,
        reason: &form.reason,
        address: &form.address,
        image: form.image_url.as_ref().map(|x| x.as_str()),
        system_id: system.id,
        created_time: Utc::now().naive_utc(),
        updated_time: Utc::now().naive_utc(),
    };
    let ticket = Ticket::create(&mut conn, insert_ticket)?;
    let mut funds = vec![];
    let mut sum = 0;
    for f in form.funds.iter() {
        let fund = Fund::create(
            &mut conn,
            InsertFund {
                ticket_id: ticket.id,
                reason: &f.reason,
                amount: f.amount,
            },
        )?;
        funds.push(fund);
        sum += f.amount;
    }
    for dep in form.departments.iter() {
        let department = Department::get_by_name(&mut conn, dep, system.id)?;
        let _ = TicketWithDepartments::create(&mut conn, ticket.id, department.id)?;
    }
    Ticket::update_amount(&mut conn, ticket.id, sum)?;
    Ticket::update_next_current_approval_id(&mut conn, ticket.id, employee.company_name)?;
    let resp = CurrentTicketResponse::from((&mut conn, ticket));
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn create_assist(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<CreateAssistTicketRequest>,
) -> Result<HttpResponse, AppError> {
    // 必须是一个接了主工单的人
    let mut conn = app_state.conn()?;
    let system = get_current_system(&req, &mut conn)?;
    let employee = get_current_employee(&req, &mut conn)?;
    let ticket = Ticket::get_by_id(&mut conn, form.ticket_id)?;
    if let Some(receiver_id) = ticket.receiver_id {
        if receiver_id == employee.id {
            let assist = Assist::create(
                &mut conn,
                InsertAssist {
                    ticket_id: ticket.id,
                    submitter_id: employee.id,
                },
            )?;
            for r in form.requirements.iter() {
                let department = Department::get_by_name(&mut conn, &r.department_name, system.id)?;
                AssistWithDepartments::create(&mut conn, assist.id, department.id, r.total_num)?;
            }
            let resp = new_ok_response("提交协助工单成功");
            Ok(HttpResponse::Ok().json(resp))
        } else {
            Err(new_ok_error("你不是这个工单的接受人"))
        }
    } else {
        Err(new_ok_error("这个工单还没有接受人"))
    }
}

pub async fn get_available_tickets(
    app_state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let employee = get_current_employee(&req, &mut conn)?;
    let department_ids =
        EmployeeWithDepartments::mget_department_id_by_employee_id(&mut conn, employee.id)?;
    let tickets = Ticket::mget_available_by_department_ids(&mut conn, department_ids)?;
    let resp = AvailableTicketsResponse::from((&mut conn, tickets));
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn get_current_ticket(
    app_state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let employee = get_current_employee(&req, &mut conn)?;
    let ticket = Ticket::get_current_by_receiver(&mut conn, employee.id)?;
    if let Some(ticket) = ticket {
        let resp = CurrentTicketResponse::from((&mut conn, ticket));
        Ok(HttpResponse::Ok().json(resp))
    } else {
        let assist_ids = AssistWithEmployees::mget_assist_id_by_involver(&mut conn, employee.id)?;
        if assist_ids.len() > 1 {
            return Err(new_ok_error("你接了多于1个协助工单"));
        }
        if assist_ids.len() == 0 {
            return Err(new_ok_error("你没有接任何主工单或协助工单"));
        }
        let assist_id = assist_ids[0];
        let assist = Assist::get_by_id(&mut conn, assist_id)?;
        let ticket = Ticket::get_by_id(&mut conn, assist.ticket_id)?;
        let resp = CurrentTicketResponse::from((&mut conn, ticket, assist));
        Ok(HttpResponse::Ok().json(resp))
    }
}

pub async fn get_history_tickets(
    app_state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let employee = get_current_employee(&req, &mut conn)?;
    // 我发布的
    let mut ans1 = Ticket::get_by_creator(&mut conn, employee.id)?;
    // 我领的
    let mut ans2 = Ticket::mget_history_by_receiver(&mut conn, employee.id)?;
    // 我领的协助工单
    let mut ans3 = Assist::mget_history_by_receiver(&mut conn, employee.id)?;
    // 我参与的协助工单
    let ans4 = AssistWithEmployees::mget_assist_id_by_involver(&mut conn, employee.id)?;
    ans1.append(&mut ans2);
    for assist_id in ans4.into_iter() {
        let assist = Assist::get_by_id(&mut conn, assist_id)?;
        ans3.push(assist);
    }
    for assist in ans3.iter() {
        let t = Ticket::get_by_id(&mut conn, assist.ticket_id)?;
        ans2.push(t);
    }

    let resp = HistoryTicketsResponse::from((&mut conn, ans1, ans2, ans3));
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn take_ticket(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<TakeTicketRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let employee = get_current_employee(&req, &mut conn)?;
    // let system = get_current_system(&req, &mut conn)?;
    match form.is_assist {
        Some(true) => {
            let assist = Assist::get_by_id(&mut conn, form.tid)?;
            let ticket = Ticket::get_by_id(&mut conn, assist.ticket_id)?;

            if ticket.state == TICKET_STATE_ASSIGNED {
                let ids = EmployeeWithDepartments::mget_department_id_by_employee_id(
                    &mut conn,
                    employee.id,
                )?;
                for id in ids.into_iter() {
                    let department = Department::get_by_id(&mut conn, id)?;
                    AssistWithDepartments::add_person(&mut conn, assist.id, department.id)?;
                }
                AssistWithEmployees::create(&mut conn, assist.id, employee.id)?;
                Employee::update_state(&mut conn, employee.id, EMPLOYEE_STATUS_UNAVAILABLE)?;
                let resp = new_ok_response("接取协助工单成功");
                Ok(HttpResponse::Ok().json(resp))
            } else {
                Err(new_ok_error("主工单还没有接受人或已经关闭"))
            }
        }
        _ => {
            let resp = new_ok_response("接取工单成功");
            let ticket = Ticket::get_by_id(&mut conn, form.tid)?;
            if ticket.state == TICKET_STATE_OPEN {
                Ticket::set_receiver(&mut conn, form.tid, employee.id)?;
                Ticket::update_state(&mut conn, ticket.id, TICKET_STATE_ASSIGNED)?;
                Employee::update_state(&mut conn, employee.id, EMPLOYEE_STATUS_UNAVAILABLE)?;
                Ok(HttpResponse::Ok().json(resp))
            } else {
                Err(new_ok_error("工单没有经过完全审批或已经被其他人接受"))
            }
        }
    }
}

// 只针对主工单
pub async fn finish_ticket(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<FinishTicketRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let employee = get_current_employee(&req, &mut conn)?;
    let ticket = Ticket::get_by_id(&mut conn, form.ticket_id)?;
    if ticket.state == TICKET_STATE_CLOSED {
        Err(new_ok_error("工单已经完成"))
    } else {
        Ticket::update_state(&mut conn, ticket.id, TICKET_STATE_CLOSED)?;
        Employee::update_state(&mut conn, employee.id, EMPLOYEE_STATUS_AVAILABLE)?;
        let resp = new_ok_response("完成工单");
        Ok(HttpResponse::Ok().json(resp))
    }
}

pub async fn get_ticket_by_id(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Query<GetTicketByIDRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let employee = get_current_employee(&req, &mut conn)?;
    let ticket = Ticket::get_by_id(&mut conn, form.ticket_id)?;
    if employee.system_id == ticket.system_id {
        let resp = PCTicketResponse::try_from((&mut conn, ticket))?;
        Ok(HttpResponse::Ok().json(CommonResponse::from(resp)))
    } else {
        Err(new_ok_error("系统ID不匹配"))
    }
}
