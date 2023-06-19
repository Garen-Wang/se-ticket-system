use actix_web::{web, HttpRequest, HttpResponse};
use chrono::{Datelike, NaiveDateTime};

use crate::{
    api::{
        request::figure::{GetChartDataRequest, GetPieChartDataRequest},
        response::figure::BarChartState,
    },
    error::{new_ok_error, AppError},
    models::ticket::Ticket,
    utils::{auth::get_current_system, response::CommonResponse},
    AppState,
};

pub async fn get_pie_chart_data(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Query<GetPieChartDataRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let system = get_current_system(&req, &mut conn)?;

    let t = NaiveDateTime::parse_from_str(&format!("{} 23:59:59", form.date), "%Y-%m-%d %H:%M:%S");
    if t.is_err() {
        return Err(new_ok_error("日期不合法"));
    }
    let t = t.unwrap();

    if form.t == "daily" {
        let resp = Ticket::get_pie_chart_data(&mut conn, system.id, t)?;
        Ok(HttpResponse::Ok().json(resp))
    } else if form.t == "weekly" {
        let mut resp = Ticket::get_pie_chart_data(&mut conn, system.id, t)?;
        for i in 1..7 {
            let temp =
                Ticket::get_pie_chart_data(&mut conn, system.id, t - chrono::Duration::days(i))?;
            resp.unapproved += temp.unapproved;
            resp.approving += temp.approving;
            resp.available += temp.available;
            resp.received += temp.received;
            resp.closed += temp.closed;
        }
        Ok(HttpResponse::Ok().json(resp))
    } else {
        Err(new_ok_error("t 参数不合法"))
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
    // TODO:
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
                    m.push(BarChartState {
                        weekday: weekday as i32,
                        period: Some(times[i].into()),
                        open: a + (i as i32) * 100,
                        closed: b + (i as i32) * 100,
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
                m.push(BarChartState {
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
