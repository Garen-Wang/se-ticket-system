use actix_web::{web, HttpResponse};

async fn healthcheck() -> HttpResponse {
    HttpResponse::Ok().body("ok")
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/healthcheck").route("", web::get().to(healthcheck)));
}
