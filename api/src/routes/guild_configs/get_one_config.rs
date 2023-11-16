use axum::{ extract::{ Extension, Path }, Json };

use crate::{
    app_state::AppState,
    queries::guild_config_queries::GuildConfigQueries,
    utilities::app_error::AppError,
    routes::guild_configs::{ ResponseGuildConfig, ResponseDataGuildConfig },
    bot_guild_entity_queries::BotGuildEntityQueries,
};

pub async fn get_one_config_by_discord_id(
    Extension(state): Extension<AppState>,
    Path((bot_id, guild_id)): Path<(String, String)>
) -> Result<Json<ResponseDataGuildConfig>, AppError> {
    println!("req to get guild config");
    let config = GuildConfigQueries::find_by_discord_ids(&state.db, &bot_id, &guild_id).await?;
    // println!("config {:?}", config);
    let response_config: ResponseGuildConfig = config.into(); // Make sure you have `From` trait implemented for this conversion

    // Wrap your config data in your ResponseDataGuildConfig struct
    let response_data = ResponseDataGuildConfig {
        data: response_config,
    };

    // Return the JSON response
    Ok(Json(response_data))
}
