use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct GetPieChartDataResponse {
    pub unapproved: i32,
    pub approving: i32,
    pub available: i32,
    pub received: i32,
    pub closed: i32,
    pub rejected: i32,
}

// #[derive(Debug, Clone, Serialize)]
pub type GetBarChartDataResponse = Vec<BarChartState>;

#[derive(Debug, Clone, Serialize)]
pub struct BarChartState {
    pub weekday: i32,
    pub period: Option<String>,
    pub open: i32,
    pub closed: i32,
}

pub type GetTableResponse = Vec<TableState>;

#[derive(Debug, Clone, Serialize)]
pub struct TableState {
    pub range: String,
    pub open: i32,
    pub closed: i32,
}
