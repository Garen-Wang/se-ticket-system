use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct UploadFileV2Request {
    pub name: String,
    pub file: String,
}
