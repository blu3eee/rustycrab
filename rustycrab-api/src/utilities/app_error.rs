// utilities/app_error.rs
use axum::{ http::StatusCode, response::IntoResponse, Json };
use sea_orm::DbErr;
use serde::{ Deserialize, Serialize };
use std::fmt;
use std::error::Error;

pub type BoxedError = Box<dyn Error + Send + Sync + 'static>;

#[derive(Debug)]
pub struct AppError {
    code: StatusCode,
    message: String,
}

impl AppError {
    pub fn new(code: StatusCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    pub fn internal_server_error(message: impl Into<String>) -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: message.into(),
        }
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self {
            code: StatusCode::BAD_REQUEST,
            message: message.into(),
        }
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self {
            code: StatusCode::NOT_FOUND,
            message: message.into(),
        }
    }

    // You can add other methods here for different types of errors
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            self.code,
            Json(ErrorResponse {
                error: self.message.clone(),
            }),
        ).into_response()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ErrorResponse {
    error: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl Error for AppError {}

impl From<DbErr> for AppError {
    fn from(err: DbErr) -> Self {
        eprintln!("Database error: {:?}", err); // Make sure to use the log crate to log the error.

        // You can add specific matches for different error types if needed.
        // For example, if you have unique constraint violations that might be caused by client input,
        // you can return a 400 error instead.
        match err {
            DbErr::Query(query_error) => {
                // Handle specific query errors if necessary
                AppError::internal_server_error(format!("Database query error: {}", query_error))
            }
            DbErr::RecordNotFound(_) => {
                // This might happen due to a client error, if they reference a non-existent record
                AppError::not_found("The requested record does not exist.")
            }
            // Add more matches as necessary for different kinds of errors
            _ => {
                // For any other database error, return a 500 internal server error
                AppError::internal_server_error(
                    "An internal error occurred while accessing the database."
                )
            }
        }
    }
}
