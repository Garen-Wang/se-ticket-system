use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct GetPieChartDataRequest {
    pub t: String,
    pub date: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetChartDataRequest {
    pub t: String,
}
