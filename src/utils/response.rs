use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct CommonResponse<T> {
    pub code: i32,
    pub success: bool,
    pub data: T,
}

impl<T: std::fmt::Debug + Clone + Serialize> From<T> for CommonResponse<T> {
    fn from(data: T) -> Self {
        Self {
            code: 200,
            success: true,
            data,
        }
    }
}
