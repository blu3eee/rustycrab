// routes/bot/get_all_bots.rs
use crate::{ queries::bot_queries, utilities::app_error::AppError, app_state::AppState };
use axum::{ extract::Extension, Json };

use super::{ ResponseDataBots, ResponseBot };

// You would define ResponseDataBots and ResponseBot in a similar way to ResponseDataTasks and ResponseTask.

pub async fn get_all_bots(Extension(state): Extension<AppState>) -> Result<
    Json<ResponseDataBots>,
    AppError
> {
    let bots = bot_queries
        ::get_all_bots(&state.db).await?
        .into_iter()
        .map(|bot_model| ResponseBot {
            id: bot_model.id,
            bot_id: bot_model.bot_id,
            token: bot_model.token, // Be careful with exposing tokens!
            theme_hex_color: bot_model.theme_hex_color,
            discord_secret: bot_model.discord_secret,
            discord_callback_url: bot_model.discord_callback_url,
            premium_flags: bot_model.premium_flags,
        })
        .collect::<Vec<ResponseBot>>();

    Ok(Json(ResponseDataBots { data: bots }))
}
