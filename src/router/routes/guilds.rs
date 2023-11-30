use rustycrab_model::response::guilds::ResponseGuild;
use crate::database::guild_info::Model as GuildModel;

impl From<GuildModel> for ResponseGuild {
    fn from(model: GuildModel) -> Self {
        Self {
            id: model.id,
            guild_id: model.guild_id,
        }
    }
}
