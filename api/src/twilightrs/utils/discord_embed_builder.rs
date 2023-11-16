use twilight_http::Client as HttpClient;
use twilight_model::{ user::User, guild::Guild };

use crate::{ twilightrs::embeds::DiscordEmbed, cdn_avatar, cdn_guild_icon };

#[allow(dead_code)]
pub struct DiscordEmbedBuilder<'a> {
    http: &'a HttpClient,
    embed: DiscordEmbed,
}

pub enum MyEmbedTypes {
    Guild(Guild),
    User(User),
}

impl<'a> DiscordEmbedBuilder<'a> {
    pub fn new(client: &'a HttpClient) -> Self {
        DiscordEmbedBuilder { http: client, embed: DiscordEmbed::new() }
    }

    pub fn set_guild_author(mut self, guild: Option<Guild>) -> Self {
        if let Some(guild) = guild {
            self.embed.author_name = Some(guild.name);
            self.embed.author_icon_url = guild.icon.map(|icon_hash|
                cdn_guild_icon!(guild.id, icon_hash)
            );
        }

        self
    }

    pub fn set_user_author(mut self, user: User) -> Self {
        self.embed.author_name = Some(user.name);
        self.embed.author_icon_url = user.avatar.map(|avatar_hash|
            cdn_avatar!(user.id, avatar_hash)
        );

        self
    }

    pub fn embed(self) -> DiscordEmbed {
        self.embed
    }
}
