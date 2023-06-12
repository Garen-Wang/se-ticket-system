use actix_web::{web, HttpRequest, HttpResponse};

use crate::{
    api::{
        request::ticket::{CreateTicketRequest, MGetTicketByPageRequest},
        response::ticket::{MGetOverviewByPageResponse, TicketOverviewResponse},
    },
    error::AppError,
    models::ticket::{InsertTicket, Ticket},
    utils::{
        auth::{get_current_employee, get_current_system},
        response::CommonResponse,
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
    Ok(
        HttpResponse::Ok().json(CommonResponse::from(MGetOverviewByPageResponse {
            total: count,
            tickets: tickets
                .into_iter()
                .map(|t| TicketOverviewResponse::from(t))
                .collect(),
        })),
    )
}

pub async fn create_ticket(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Json<CreateTicketRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;

    let employee = get_current_employee(&req, &mut conn)?;
    let system = get_current_system(&req, &mut conn)?;

    let insert_ticket = InsertTicket {
        creator_id: employee.id,
        title: &form.title,
        amount: 0,
        reason: &form.reason,
        address: &form.address,
        image: None,
    };
    let ticket = Ticket::create(&mut conn, insert_ticket)?;
    for fund in form.funds.iter() {}
    unimplemented!()
}
