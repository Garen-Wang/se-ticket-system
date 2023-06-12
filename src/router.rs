use actix_web::{web, HttpResponse};

use crate::api::handlers::*;

async fn healthcheck() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/healthcheck").route("", web::get().to(healthcheck)));

    cfg.service(
        web::scope("/auth")
            .route("login", web::post().to(auth::login))
            .route("", web::get().to(auth::get_myself)),
    );

    cfg.service(
        web::scope("/system")
            .route("", web::post().to(system::create_system))
            .route("employee", web::post().to(system::create_employee)),
    );

    cfg.service(
        web::scope("/ticket")
            .route("page", web::get().to(ticket::get_tickets_by_page))
            .route("", web::post().to(ticket::create_ticket))
            .route("assist", web::post().to(ticket::create_assist_ticket))
            .route("", web::get().to(ticket::get_current_ticket))
            .route("take", web::post().to(ticket::take_ticket))
            .route("finish", web::post().to(ticket::finish_ticket)),
    );
}
