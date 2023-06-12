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

#[derive(Debug, Clone, Serialize)]
pub struct MessageResponse {
    pub message: String,
}

pub fn new_ok_response(message: &str) -> CommonResponse<MessageResponse> {
    CommonResponse::from(MessageResponse {
        message: message.to_string(),
    })
}
