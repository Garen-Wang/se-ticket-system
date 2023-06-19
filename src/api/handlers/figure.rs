use actix_web::{web, HttpRequest, HttpResponse};
use chrono::{Datelike, NaiveDateTime};

use crate::{
    api::{request::figure::GetPieChartDataRequest, response::figure::GetBarChartDataResponse},
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
        // TODO: 日期兜底
        return Err(new_ok_error("日期不合法"));
    }
    let t = t.unwrap();

    if form.t == "daily" {
        let resp = Ticket::get_pie_chart_data(&mut conn, system.id, t)?;
        Ok(HttpResponse::Ok().json(CommonResponse::from(resp)))
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
        Ok(HttpResponse::Ok().json(CommonResponse::from(resp)))
    } else {
        Err(new_ok_error("t 参数不合法"))
    }
}

pub async fn get_bar_chart_data(
    app_state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Query<GetPieChartDataRequest>,
) -> Result<HttpResponse, AppError> {
    let mut conn = app_state.conn()?;
    let system = get_current_system(&req, &mut conn)?;

    let date = form.date.clone();
    // let date = if form.date.len() > 0 {
    //     form.date
    // } else {
    //     "haha".to_owned() // TODO: d d
    // };

    let _start_times = vec![
        "0:00:00", "4:00:00", "8:00:00", "12:00:00", "16:00:00", "20:00:00",
    ];
    let end_times = vec![
        "3:59:59", "7:59:59", "11:59:59", "15:59:59", "21:59:59", "23:59:59",
    ];
    let periods = vec![
        "0:00-4:00",
        "4:00-8:00",
        "8:00-12:00",
        "12:00-16:00",
        "16:00-20:00",
        "20:00-24:00",
    ];
    match form.t.as_str() {
        "daily" => {
            let mut resp: GetBarChartDataResponse = vec![];
            for i in 0..6 {
                let t = NaiveDateTime::parse_from_str(
                    &format!("{} {}", date, end_times[i]),
                    "%Y-%m-%d %H:%M:%S",
                );
                if t.is_err() {
                    return Err(new_ok_error("日期不合法"));
                }
                let t = t.unwrap();
                let state = Ticket::get_bar_chart_data(
                    &mut conn,
                    system.id,
                    t,
                    t.weekday() as i32,
                    Some(periods[i].to_owned()),
                )?;
                resp.push(state);
            }
            Ok(HttpResponse::Ok().json(CommonResponse::from(resp)))
        }
        "weekly" => {
            let mut now =
                NaiveDateTime::parse_from_str(&format!("{} 23:59:59", date), "%Y-%m-%d %H:%M:%S")
                    .unwrap();
            let mut weekday = now.weekday();
            let mut resp = vec![];
            for _ in 0..7 {
                // get_closed_ticket_count_n_day_ago(i)
                // get_open_ticket_count_n_day_ago(i)
                let state =
                    Ticket::get_bar_chart_data(&mut conn, system.id, now, weekday as i32, None)?;
                resp.push(state);
                weekday = weekday.pred();
                now = now - chrono::Duration::days(1);
            }
            Ok(HttpResponse::Ok().json(CommonResponse::from(resp)))
        }
        _ => Err(new_ok_error("参数不合法")),
    }
}

// TODO: 表格
// pub async fn get_approval_table(
//     app_state: web::Data<AppState>,
//     req: HttpRequest,
//     form: web::Query<GetApprovalTableRequest>,
// ) -> Result<HttpResponse, AppError> {
// }
