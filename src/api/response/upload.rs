use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct UploadFileV2Response {
    pub url: String,
}
