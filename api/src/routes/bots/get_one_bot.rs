// routes/bot/get_all_bots.rs
use crate::{
    queries::bot_queries::BotQueries,
    utilities::app_error::AppError,
    app_state::AppState,
};
use axum::{ extract::{ Extension, Path }, Json };

use super::{ ResponseBot, ResponseDataBot };

// You would define ResponseDataBots and ResponseBot in a similar way to ResponseDataTasks and ResponseTask.

pub async fn get_bot_from_discord_id(
    Extension(state): Extension<AppState>,
    Path(bot_discord_id): Path<String>
) -> Result<Json<ResponseDataBot>, AppError> {
    let bot_model = BotQueries::find_by_discord_id(&state.db, &bot_discord_id).await?;

    let response = ResponseBot {
        id: bot_model.id,
        bot_id: bot_model.bot_id,
        token: bot_model.token, // Be careful with exposing tokens!
        theme_hex_color: bot_model.theme_hex_color,
        discord_secret: bot_model.discord_secret,
        discord_callback_url: bot_model.discord_callback_url,
        premium_flags: bot_model.premium_flags,
    };

    Ok(
        Json(ResponseDataBot {
            data: response,
        })
    )
}
