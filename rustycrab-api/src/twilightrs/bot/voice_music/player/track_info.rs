use songbird::input::AuxMetadata;

use crate::{
    twilightrs::{ messages::DiscordEmbedField, discord_client::DiscordClient },
    utilities::format_duration,
};

pub fn track_info_fields(
    client: &DiscordClient,
    locale: &str,
    metadata: &AuxMetadata,
    position_inqueue: Option<usize>
) -> Vec<DiscordEmbedField> {
    let mut result = vec![
        DiscordEmbedField {
            name: format!("Track"),
            value: format!(
                "[**{}**]{}",
                metadata.title.as_ref().unwrap_or(&"<UNKNOWN>".to_string()).to_string(),
                metadata.source_url.clone().map_or_else(
                    || String::new(),
                    |url| format!("({})", url)
                )
            ),
            inline: false,
        },
        DiscordEmbedField {
            name: client.get_locale_string(&locale, "music-duration", None),
            value: if let Some(duration) = metadata.duration.as_ref() {
                format_duration(duration)
            } else {
                format!("<Unknown duration>")
            },
            inline: true,
        }
    ];

    if let Some(creator) = &metadata.artist {
        result.push(DiscordEmbedField {
            name: client.get_locale_string(&locale, "music-content-creator", None),
            value: creator.clone(),
            inline: true,
        });
    }
    if let Some(position) = position_inqueue {
        result.push(DiscordEmbedField {
            name: client.get_locale_string(&locale, "music-position-inqueue", None),
            value: position.to_string(),
            inline: true,
        });
    }
    if let Some(url) = &metadata.source_url {
        result.push(DiscordEmbedField {
            name: client.get_locale_string(&locale, "music-content-credits", None),
            value: if url.contains("soundcloud") {
                format!(
                    "[{}]({})",
                    client.get_locale_string(&locale, "music-content-credits-soundcloud", None),
                    url
                )
            } else {
                format!(
                    "[{}]({})",
                    client.get_locale_string(&locale, "music-content-credits-youtube", None),
                    url
                )
            },
            inline: false,
        });
    }
    result
}
