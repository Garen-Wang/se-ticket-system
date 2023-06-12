use std::vec;

use actix_web::{web, HttpRequest, HttpResponse};
use chrono::Utc;

use crate::{
    api::{
        request::ticket::{
            CreateAssistTicketRequest, CreateTicketRequest, FinishTicketRequest,
            MGetTicketByPageRequest, TakeTicketRequest,
        },
        response::ticket::{
            MGetOverviewByPageResponse, TicketDetailResponse, TicketOverviewResponse,
        },
    },
    error::{new_ok_error, AppError},
    models::{
        department::Department,
        employee::Employee,
        ticket::{
            Assist, Fund, InsertAssist, InsertFund, InsertTicket, Ticket, TicketWithDepartments,
        },
    },
    utils::{
        auth::{get_current_employee, get_current_system, is_super_admin},
        response::{new_ok_response, CommonResponse},
    },
    AppState,
};

pub async fn get_tickets_by_page(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<MGetTicketByPageRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let system = get_current_system(&req, &mut conn)?;
    let count = Ticket::get_count(&mut conn, system.id)?;
    let tickets = Ticket::mget_by_page(&mut conn, system.id, form.size, form.page)?;

    let mut ts = vec![];
    for ticket in tickets.into_iter() {
        let employee = Employee::get_by_id(&mut conn, ticket.creator_id)?;
        let funds = Fund::get_by_ticket_id(&mut conn, ticket.id)?;
        ts.push(TicketOverviewResponse::from((ticket, employee, funds)));
    }
    Ok(
        HttpResponse::Ok().json(CommonResponse::from(MGetOverviewByPageResponse {
            total: count,
            tickets: ts,
        })),
    )
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
        image: None, // TODO: 插入图片
        system_id: system.id,
        created_time: Utc::now().naive_utc(),
        updated_time: Utc::now().naive_utc(),
    };
    // FIXME: 事务
    let ticket = Ticket::create(&mut conn, insert_ticket)?;
    let mut funds = vec![];
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
    }
    for dep in form.departments.iter() {
        let department = Department::get_by_name(&mut conn, dep, system.id)?;
        let _ = TicketWithDepartments::create(&mut conn, ticket.id, department.id)?;
    }
    let resp = TicketDetailResponse::from(ticket);
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn create_assist_ticket(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<CreateAssistTicketRequest>,
) -> Result<HttpResponse, AppError> {
    // 必须是一个接了主工单的人
    let mut conn = app_state.conn()?;
    let system = get_current_system(&req, &mut conn)?;
    let employee = get_current_employee(&req, &mut conn)?;
    if TicketWithDepartments::is_receiver(&mut conn, form.ticket_id, employee.id)? {
        let department = Department::get_by_name(&mut conn, &form.department_name, system.id)?;
        let _assist = Assist::create(
            &mut conn,
            InsertAssist {
                ticket_id: form.ticket_id,
                submitter_id: employee.id,
                department_id: department.id,
                amount: form.amount,
            },
        )?;
        let resp = new_ok_response("提交协助工单成功");
        Ok(HttpResponse::Ok().json(resp))
    } else {
        Err(new_ok_error("协助工单提交者没接主工单"))
    }
}

pub async fn get_current_ticket(
    app_state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let employee = get_current_employee(&req, &mut conn)?;
    let ticket_id = TicketWithDepartments::get_current_ticket_id(&mut conn, employee.id)?;
    let ticket = Ticket::get_by_id(&mut conn, ticket_id)?;
    let resp = TicketDetailResponse::from(ticket);
    Ok(HttpResponse::Ok().json(resp))
}

// pub async fn get_history_tickets(
//     app_state: web::Data<AppState>,
//     req: HttpRequest,
// ) -> Result<HttpResponse, AppError> {
//     let mut conn = app_state.conn()?;
//     let employee = get_current_employee(&req, &mut conn)?;
//     let ans1 = Ticket::get_by_creator(&mut conn, employee.id)?;
//     let ans2 = TicketWithDepartments::mget_by_receiver(&mut conn, employee.id)?;
//     let ans3 = Assist::get_by_receiver(&mut conn, employee.id)?;
//     let ans = vec![];

//     let resp = HistoryTicketsResponse::from((ans1, ans2, ans3));
//     Ok(HttpResponse::Ok().json(resp))
// }

pub async fn take_ticket(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<TakeTicketRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let employee = get_current_employee(&req, &mut conn)?;
    let system = get_current_system(&req, &mut conn)?;
    let department = Department::get_by_name(&mut conn, &form.department_name, system.id)?;
    TicketWithDepartments::add_receiver(&mut conn, form.ticket_id, employee.id, department.id)?;

    let resp = new_ok_response("接取工单成功");
    // TODO: 人的状态改变了，主工单的状态也可能改变
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn finish_ticket(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<FinishTicketRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let employee = get_current_employee(&req, &mut conn)?;
    let ticket = Ticket::get_by_id(&mut conn, form.ticket_id)?;
    let associations = TicketWithDepartments::get_by_receiver(&mut conn, ticket.id, employee.id)?;
    TicketWithDepartments::update_state(&mut conn, associations.id, 1)?;
    let resp = new_ok_response("完成工单");
    // TODO: 如果所有 apply_dev_info 完成，就改 ticket 状态
    Ok(HttpResponse::Ok().json(resp))
}
