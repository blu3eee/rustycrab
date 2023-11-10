// utilities/app_error.rs
use axum::{ http::StatusCode, response::IntoResponse, Json };
use serde::{ Deserialize, Serialize };

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

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    error: String,
}

// Add DiscordError to the imports.
use discord::Error as DiscordError;

impl From<DiscordError> for AppError {
    fn from(err: DiscordError) -> Self {
        match err {
            // Handle specific Discord errors if necessary
            DiscordError::Other(msg) => AppError::internal_server_error(msg),
            // Add cases for other specific Discord errors you want to handle differently
            _ => AppError::internal_server_error("An error occurred with Discord operation."),
        }
    }
}
