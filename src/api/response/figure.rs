use std::collections::BTreeMap;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct GetPieChartDataResponse {
    pub resp: BTreeMap<i32, PieChartStateResponse>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PieChartStateResponse {
    pub open: i32,
    pub received: i32,
    pub closed: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetBarChartDataResponse {
    pub resp: BTreeMap<i32, BarChartStateResponse>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BarChartStateResponse {
    pub open: i32,
    pub closed: i32,
}
