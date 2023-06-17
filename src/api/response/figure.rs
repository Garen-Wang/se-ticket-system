use std::collections::BTreeMap;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct GetPieChartDataResponse {
    pub unapproved: i32,
    pub approving: i32,
    pub available: i32,
    pub received: i32,
    pub closed: i32,
}

// #[derive(Debug, Clone, Serialize)]
pub type GetBarChartDataResponse = BTreeMap<i32, BarChartStateResponse>;

#[derive(Debug, Clone, Serialize)]
pub struct BarChartStateResponse {
    pub weekday: i32,
    pub period: Option<String>,
    pub open: i32,
    pub closed: i32,
}