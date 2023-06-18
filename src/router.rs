use actix_web::{web, HttpResponse};

use crate::api::handlers::{ticket::get_available_tickets, *};

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
            .route("assist", web::post().to(ticket::create_assist))
            .route("current", web::get().to(ticket::get_current_ticket))
            .route(
                "history/page",
                web::get().to(ticket::get_history_tickets_by_page),
            )
            .route("history", web::get().to(ticket::get_history_tickets))
            .route("available", web::get().to(get_available_tickets))
            .route("take", web::post().to(ticket::take_ticket))
            .route("finish", web::post().to(ticket::finish_ticket))
            .route("", web::get().to(ticket::get_ticket_by_id)),
    );

    cfg.service(web::scope("/department").route("", web::get().to(approval::list_departments)));
    cfg.service(
        web::scope("approval").route("", web::get().to(approval::get_approval_levels_by_company)),
    );

    cfg.service(
        web::scope("/figure")
            .route("pie", web::get().to(figure::get_pie_chart_data))
            .route("bar", web::get().to(figure::get_bar_chart_data)),
    );

    cfg.route("/upload", web::post().to(upload::save_file));
}
