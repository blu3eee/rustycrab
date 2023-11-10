use discord::builders::{ EmbedBuilder, EmbedFooterBuilder, EmbedAuthorBuilder, EmbedFieldsBuilder };

use crate::discordrs::DiscordEmbed;

pub fn build_embed(mut builder: EmbedBuilder, embed_data: DiscordEmbed) -> EmbedBuilder {
    builder = builder.color(u64::from_str_radix("2B2D31", 16).unwrap());
    if let Some(title) = embed_data.title {
        builder = builder.title(&title);
    }
    if let Some(description) = embed_data.description {
        builder = builder.description(&description);
    }
    if let Some(url) = embed_data.url {
        builder = builder.url(&url);
    }
    // Skipping timestamp as it is not included in DiscordEmbed.
    if let Some(color) = embed_data.color {
        builder = builder.color(color);
    }

    if let Some(text) = embed_data.footer_text {
        builder = builder.footer(|f: EmbedFooterBuilder| f.text(&text));
        if let Some(url) = embed_data.footer_icon_url {
            builder = builder.footer(|f| f.icon_url(&url));
        }
    }

    if let Some(image) = embed_data.image {
        builder = builder.image(&image);
    }

    if let Some(thumbnail) = embed_data.thumbnail {
        builder = builder.thumbnail(&thumbnail);
    }

    if let Some(name) = embed_data.author_name {
        builder = builder.author(|a: EmbedAuthorBuilder| a.name(&name));
        if let Some(url) = embed_data.author_icon_url {
            builder = builder.author(|f| f.icon_url(&url));
        }
    }

    if let Some(fields) = embed_data.fields {
        builder = builder.fields(|mut f: EmbedFieldsBuilder| {
            for field in fields {
                f = f.field(&field.name, &field.value, field.inline.unwrap_or(false));
            }
            f
        });
    }

    builder // return the modified EmbedBuilder
}
