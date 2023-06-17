use std::collections::BTreeMap;

use actix_web::{web, HttpRequest, HttpResponse};
use chrono::{Datelike, Weekday};

use crate::{
    api::{
        request::figure::GetChartDataRequest,
        response::figure::{BarChartStateResponse, GetPieChartDataResponse},
    },
    error::{new_ok_error, AppError},
    utils::response::CommonResponse,
    AppState,
};

fn weekday_to_string(weekday: Weekday) -> &'static str {
    match weekday as i32 {
        0 => "Monday",
        1 => "Tuesday",
        2 => "Wednesday",
        3 => "Thursday",
        4 => "Friday",
        5 => "Saturday",
        6 => "Sunday",
        _ => "???",
    }
}

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
                unapproved: 100,
                approving: 200,
                available: 250,
                received: 10,
                closed: 300,
            };
            Ok(HttpResponse::Ok().json(CommonResponse::from(resp)))
        }
        "weekly" => {
            let resp = GetPieChartDataResponse {
                unapproved: 300,
                approving: 100,
                available: 50,
                received: 10,
                closed: 150,
            };
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
            let weekday = chrono::Local::now().weekday();
            let times = vec![
                "0:00-4:00",
                "4:00-8:00",
                "8:00-12:00",
                "12:00-16:00",
                "16:00-20:00",
                "20:00-24:00",
            ];
            let resp = {
                let mut m = vec![];
                for i in 0..6 {
                    m.push(BarChartStateResponse {
                        weekday: weekday as i32,
                        period: Some(times[i].into()),
                        open: a + (i as i32) * 100,
                        closed: b - (i as i32) * 100,
                    })
                }
                m
            };
            Ok(HttpResponse::Ok().json(CommonResponse::from(resp)))
        }
        "weekly" => {
            let mut weekday = chrono::Local::now().weekday();
            let mut m = vec![];
            for _ in 0..7 {
                // get_closed_ticket_count_n_day_ago(i)
                // get_open_ticket_count_n_day_ago(i)
                m.push(BarChartStateResponse {
                    weekday: weekday as i32,
                    period: None,
                    open: a,
                    closed: b,
                });
                weekday = weekday.pred();
            }
            let resp = m;
            Ok(HttpResponse::Ok().json(CommonResponse::from(resp)))
        }
        _ => Err(new_ok_error("参数不合法")),
    }
}
