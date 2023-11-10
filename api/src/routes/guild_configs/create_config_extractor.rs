use crate::utilities::app_error::AppError;
use axum::{
    async_trait,
    body::HttpBody,
    extract::FromRequest,
    http::Request,
    BoxError,
    Json,
    RequestExt,
};
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct ValidateCreateConfig {
    #[validate(required(message = "missing bot discord id"))]
    bot_id: Option<String>,
    #[validate(required(message = "missing guild discord id"))]
    guild_id: Option<String>,
}

#[async_trait]
impl<S, B> FromRequest<S, B>
    for ValidateCreateConfig
    where B: HttpBody + Send + 'static, B::Data: Send, B::Error: Into<BoxError>, S: Send + Sync
{
    type Rejection = AppError;

    async fn from_request(
        req: Request<B>,
        _state: &S
    ) -> Result<ValidateCreateConfig, Self::Rejection> {
        let Json(config) = req.extract::<Json<ValidateCreateConfig>, _>().await.map_err(|error| {
            eprintln!("Error extracting new config: {:?}", error);
            AppError::internal_server_error("Something went wrong, please try again")
        })?;

        if let Err(errors) = config.validate() {
            let field_errors = errors.field_errors();
            for (_, error) in field_errors {
                return Err(
                    AppError::bad_request(
                        // feel safe unwrapping because we know there is at least one error, and we only care about the first for now
                        error.first().unwrap().clone().message.unwrap().to_string()
                    )
                );
            }
        }

        Ok(config)
    }
}
