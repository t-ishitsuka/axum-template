use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

pub mod error_404;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    NotFoundError(String),
    #[error("{0}")]
    InternalServerError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status_code = match self {
            AppError::NotFoundError(_) => StatusCode::NOT_FOUND,
            AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status_code).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
