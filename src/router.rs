use actix_web::{web, HttpResponse};

use crate::user;

async fn healthcheck() -> HttpResponse {
    HttpResponse::Ok().body("ok")
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/healthcheck").route("", web::get().to(healthcheck)));

    cfg.service(
        web::scope("/auth")
            .route("login", web::post().to(user::handler::login))
            .route("register", web::post().to(user::handler::register)),
    );
}
