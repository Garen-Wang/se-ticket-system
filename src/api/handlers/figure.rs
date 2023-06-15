use std::collections::BTreeMap;

use actix_web::{web, HttpRequest, HttpResponse};
use chrono::Datelike;

use crate::{
    api::{
        request::figure::GetChartDataRequest,
        response::figure::{
            BarChartStateResponse, GetBarChartDataResponse, GetPieChartDataResponse,
            PieChartStateResponse,
        },
    },
    error::{new_ok_error, AppError},
    utils::response::CommonResponse,
    AppState,
};

pub async fn get_pie_chart_data(
    _app_state: web::Data<AppState>,
    _req: HttpRequest,
    form: web::Query<GetChartDataRequest>,
) -> Result<HttpResponse, AppError> {
    // let mut conn = app_state.conn()?;
    // let system = get_current_system(&req, &mut conn)?;
    let a = 100;
    let b = 200;
    let c = 300;
    match form.t.as_str() {
        "daily" => {
            // get_daily_closed_ticket_count()
            // get_daily_opening_ticket_count()
            let resp = GetPieChartDataResponse {
                resp: {
                    let mut m: BTreeMap<i32, PieChartStateResponse> = BTreeMap::new();
                    let weekday = chrono::Local::now().weekday() as i32 + 1;
                    m.insert(
                        weekday,
                        PieChartStateResponse {
                            open: a,
                            received: b,
                            closed: c,
                        },
                    );
                    m
                },
            };
            Ok(HttpResponse::Ok().json(CommonResponse::from(resp)))
        }
        "weekly" => {
            let weekday = chrono::Local::now().weekday() as i32;
            let mut m = BTreeMap::new();
            for i in 0..7 {
                // get_closed_ticket_count_n_day_ago(i)
                // get_open_ticket_count_n_day_ago(i)
                m.insert(
                    (weekday - i + 7) % 7 + 1,
                    PieChartStateResponse {
                        open: a,
                        received: b,
                        closed: c,
                    },
                );
            }
            let resp = GetPieChartDataResponse { resp: m };
            Ok(HttpResponse::Ok().json(CommonResponse::from(resp)))
        }
        _ => Err(new_ok_error("参数不合法")),
    }
}

pub async fn get_bar_chart_data(
    _app_state: web::Data<AppState>,
    _req: HttpRequest,
    form: web::Query<GetChartDataRequest>,
) -> Result<HttpResponse, AppError> {
    // let mut conn = app_state.conn()?;
    // let system = get_current_system(&req, &mut conn)?;
    let a = 100;
    let b = 200;
    match form.t.as_str() {
        "daily" => {
            // get_daily_closed_ticket_count()
            // get_daily_opening_ticket_count()
            let resp = GetBarChartDataResponse {
                resp: {
                    let mut m = BTreeMap::new();
                    let weekday = chrono::Local::now().weekday() as i32 + 1;
                    m.insert(weekday, BarChartStateResponse { open: a, closed: b });
                    m
                },
            };
            Ok(HttpResponse::Ok().json(CommonResponse::from(resp)))
        }
        "weekly" => {
            let weekday = chrono::Local::now().weekday() as i32;
            let mut m = BTreeMap::new();
            for i in 0..7 {
                let temp = (weekday - i + 7) % 7 + 1;
                // get_closed_ticket_count_n_day_ago(i)
                // get_open_ticket_count_n_day_ago(i)
                m.insert(temp, BarChartStateResponse { open: a, closed: b });
            }
            let resp = GetBarChartDataResponse { resp: m };
            Ok(HttpResponse::Ok().json(CommonResponse::from(resp)))
        }
        _ => Err(new_ok_error("参数不合法")),
    }
}
