pub enum ColorTypes {
    String(String),
}

pub fn convert_color_u32(color: ColorTypes) -> u32 {
    match color {
        ColorTypes::String(color_string) => {
            u32::from_str_radix(color_string.trim_start_matches("#"), 16).unwrap_or_else(|_|
                u32::from_str_radix("2B2D31", 16).unwrap()
            )
        }
    }
}

use sea_orm::DbErr;

use super::app_error::AppError;

/// Converts a SeaORM database error into an application-specific error.
pub fn convert_seaorm_error(err: DbErr) -> AppError {
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

use twilight_model::channel::message::component::ButtonStyle;

pub fn color_to_button_style(color: &str) -> ButtonStyle {
    match color.to_lowercase().as_str() {
        "red" => ButtonStyle::Danger,
        "green" => ButtonStyle::Success,
        "blue" => ButtonStyle::Primary,
        "gray" | "grey" => ButtonStyle::Secondary,
        "link" => ButtonStyle::Link,
        _ => ButtonStyle::Secondary, // Default case
    }
}
