pub mod client;
pub mod commands;
pub mod events;
pub mod utils;

#[derive(Debug, Clone, Default)]
pub struct DiscordEmbed {
    pub title: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub color: Option<u64>,
    pub footer_text: Option<String>,
    pub footer_icon_url: Option<String>,
    pub image: Option<String>,
    pub thumbnail: Option<String>,
    pub author_name: Option<String>,
    pub author_icon_url: Option<String>,
    pub fields: Option<Vec<DiscordEmbedField>>,
}

#[derive(Debug, Clone)]
pub struct DiscordEmbedField {
    pub name: String,
    pub value: String,
    pub inline: Option<bool>,
}

pub enum MessageContent {
    Text(String),
    Embed(DiscordEmbed),
    TextAndEmbed(String, DiscordEmbed),
    None,
}
