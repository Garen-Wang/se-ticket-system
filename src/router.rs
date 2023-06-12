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
            .route("register", web::post().to(auth::register)),
    );

    cfg.service(web::scope("/ticket").route("page", web::get().to(ticket::get_tickets_by_page)));
}
