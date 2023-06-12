use std::fmt::Display;

use actix_web::{error::BlockingError, http::StatusCode, HttpResponse};
use serde::Serialize;
use thiserror::Error;

use crate::utils::response::CommonResponse;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("OK: {0}")]
    Ok(ErrMessage), // 200

    #[error("Unauthorized: {0}")]
    Unauthorized(ErrMessage), // 401

    #[error("Forbidden: {0}")]
    Forbidden(ErrMessage), // 403

    #[error("Not Found: {0}")]
    NotFound(ErrMessage), // 404

    #[error("Unprocessable Entity: {0}")]
    UnprocessableEntity(ErrMessage), // 422

    #[error("Internal Server Error")]
    InternalServerError,
}

#[derive(Debug, Clone, Serialize)]
pub struct ErrMessage {
    pub error: String,
}

impl Display for ErrMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.error)
    }
}

impl actix_web::error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match self {
            AppError::Unauthorized(val) => {
                HttpResponse::Unauthorized().json(CommonResponse::from(val))
            }
            AppError::Forbidden(val) => HttpResponse::Forbidden().json(CommonResponse::from(val)),
            AppError::NotFound(val) => HttpResponse::NotFound().json(CommonResponse::from(val)),
            AppError::UnprocessableEntity(val) => {
                HttpResponse::UnprocessableEntity().json(CommonResponse::from(val))
            }
            AppError::InternalServerError => {
                HttpResponse::InternalServerError().json("internal server error")
            }
            AppError::Ok(val) => HttpResponse::Ok().json(CommonResponse::from(val)),
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            AppError::Ok(_) => StatusCode::OK,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<bcrypt::BcryptError> for AppError {
    fn from(e: bcrypt::BcryptError) -> Self {
        log::error!("bcrypt::BcryptError: {}", e);
        AppError::InternalServerError
    }
}

impl From<diesel::result::Error> for AppError {
    fn from(e: diesel::result::Error) -> Self {
        match e {
            diesel::result::Error::NotFound => AppError::Ok(ErrMessage {
                error: "requested record not found".into(),
            }),
            e => {
                log::error!("diesel::result::Error: {}", e);
                AppError::InternalServerError
            }
        }
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        match e.kind() {
            jsonwebtoken::errors::ErrorKind::InvalidToken => AppError::Unauthorized(ErrMessage {
                error: "invalid token".into(),
            }),
            jsonwebtoken::errors::ErrorKind::InvalidIssuer => AppError::Unauthorized(ErrMessage {
                error: "invalid issuer".into(),
            }),
            _ => AppError::Unauthorized(ErrMessage {
                error: "a issue was found with token provided".into(),
            }),
        }
    }
}

impl From<r2d2::Error> for AppError {
    fn from(e: r2d2::Error) -> Self {
        log::error!("r2d2::Error: {}", e);
        AppError::InternalServerError
    }
}

impl From<BlockingError> for AppError {
    fn from(e: BlockingError) -> Self {
        log::error!("BlockingError: {}", e);
        AppError::InternalServerError
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        log::error!("std::io::Error: {}", e);
        AppError::InternalServerError
    }
}
