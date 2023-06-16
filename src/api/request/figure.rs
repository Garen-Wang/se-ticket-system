use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct GetChartDataRequest {
    pub t: String,
}
