use axum::{http::StatusCode, response::IntoResponse, Json};
use sea_orm::DbErr;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct AppError {
    code: StatusCode,
    message: String,
}

// All errors should be created with new() to fire to the logs
impl AppError {
    pub fn new(
        code: StatusCode,
        message: impl Into<String>,
        log_message: impl Into<String>,
    ) -> Self {
        let message = message.into();
        let log_message = log_message.into();
        match code.as_u16() {
            500..599 => tracing::error!("{} - {}", code, log_message.clone()),
            400..500 => tracing::info!("{} - {}", code, log_message.clone()),
            _ => (),
        }

        Self { code, message }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {}", self.code, self.message)
    }
}

impl PartialEq for AppError {
    fn eq(&self, other: &Self) -> bool {
        (self.code == other.code) & (self.message == other.message)
    }
}

impl Error for AppError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            self.code,
            Json(ErrorResponse {
                error: self.message.clone(),
            }),
        )
            .into_response()
    }
}

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    error: String,
}

impl From<DbErr> for AppError {
    fn from(err: DbErr) -> Self {
        Self::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong",
            err.to_string(),
        )
    }
}
