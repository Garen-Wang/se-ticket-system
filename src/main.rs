use actix_cors::Cors;
use actix_files::Files;
use actix_web::{http, middleware::Logger, web, App, HttpServer};
use diesel::{
    r2d2::{self, ConnectionManager, PooledConnection},
    PgConnection,
};
use error::AppError;
use utils::auth::Authorization;

pub mod error;
pub mod router;
pub mod schema;

pub mod api;
pub mod models;
pub mod utils;

pub type Manager = ConnectionManager<PgConnection>;
pub type Pool = r2d2::Pool<Manager>;
pub type AppConn = PooledConnection<Manager>;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool,
}

impl AppState {
    pub fn conn(&self) -> Result<AppConn, AppError> {
        let conn = self.pool.get()?;
        Ok(conn)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    env_logger::init();
    let pool = r2d2::Pool::builder()
        .build(Manager::new(database_url))
        .expect("failed to build pool");
    let app_state = AppState { pool };

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .service(Files::new("/static", "static/").show_files_listing())
            .wrap(Logger::default())
            .app_data(web::Data::new(app_state.clone()))
            .wrap(cors)
            .wrap(Authorization)
            .configure(router::routes)
    })
    .bind(("0.0.0.0", 7878))?
    .run()
    .await
}
