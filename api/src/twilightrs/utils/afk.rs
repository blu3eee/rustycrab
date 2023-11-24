use std::{ sync::Arc, error::Error };

use fluent_bundle::FluentArgs;
use twilight_model::{
    gateway::payload::incoming::MessageCreate,
    id::{ Id, marker::{ GuildMarker, UserMarker } },
    channel::message::{ component::{ Button, ActionRow }, Component, Embed },
};

use crate::{
    twilightrs::{
        discord_client::DiscordClient,
        commands::context::context_command::GuildConfigModel,
        messages::DiscordEmbed,
    },
    utilities::utils::color_to_button_style,
};

pub async fn check_afk(
    client: &Arc<DiscordClient>,
    config: &GuildConfigModel,
    msg: &MessageCreate,
    guild_id: Id<GuildMarker>
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Check AFK status
    let (user_no_longer_afk, notify_users) = {
        let mut afk_users = client.afk_users.write().unwrap();
        if
            let Some(user_afk) = afk_users
                .get_mut(&guild_id)
                .and_then(|g| g.get_mut(&msg.author.id))
        {
            user_afk.activities_count += 1;
            if user_afk.activities_count > 3 {
                // Mark the user as no longer AFK, and remove them from the hashmap
                let notify_users = user_afk.notify.clone();
                afk_users.get_mut(&guild_id).and_then(|g| g.remove(&msg.author.id));
                (true, Some(notify_users)) // Indicates that user is no longer AFK
            } else {
                (false, None)
            }
        } else {
            (false, None)
        }
    };
    // Initiate the args for fluent bundle
    let mut args = FluentArgs::new();
    args.set("user", format!("<@{}>", msg.author.id));

    // Notify the channel if the user is no longer AFK
    if user_no_longer_afk {
        let content = client.get_locale_string(&config.locale, "afk-is-back", Some(&args));
        let message = client.http.create_message(msg.channel_id).embeds(
            &vec![
                Embed::from(DiscordEmbed {
                    description: Some(content),
                    ..Default::default()
                })
            ]
        )?.await;

        let client_cloned = client.clone();
        tokio::spawn(async move {
            if let Ok(message) = message {
                tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
                let message = message.model().await;
                if let Ok(message) = message {
                    let _ = client_cloned.http.delete_message(message.channel_id, message.id).await;
                }
            }
        });

        if let Some(users) = notify_users {
            let guild = client.http.guild(guild_id).await?.model().await?;
            for user_id in users {
                let channel = client.http.create_private_channel(user_id).await;
                if let Ok(channel) = channel {
                    if let Ok(channel) = channel.model().await {
                        args.set("server", guild.name.clone());
                        let content = client.get_locale_string(
                            &config.locale,
                            "afk-notification",
                            Some(&args)
                        );
                        let _ = client.http.create_message(channel.id).content(&content)?.await;
                    }
                }
            }
        }
    }

    // Check for user mentions and AFK status
    let content_words = msg.content.split_whitespace();

    for word in content_words {
        if word.starts_with("<@") && word.ends_with(">") {
            if
                let Ok(mentioned_user_id) = word
                    .trim_matches(|c: char| (c == '<' || c == '@' || c == '>'))
                    .parse::<u64>()
            {
                let mentioned_user_id = Id::<UserMarker>::new(mentioned_user_id);

                // Scope for the read lock
                let (afk_duration, afk_message) = {
                    let afk_users_read = client.afk_users.read().unwrap();
                    if let Some(guild_afk_users) = afk_users_read.get(&guild_id) {
                        if let Some(afk_status) = guild_afk_users.get(&mentioned_user_id) {
                            let afk_message = afk_status.message.clone().unwrap_or_default();
                            (Some(afk_status.since), Some(afk_message))
                        } else {
                            (None, None)
                        }
                    } else {
                        (None, None)
                    }
                };

                // Check if the user is AFK and send a message if necessary
                let mentioned_user = client.http.user(mentioned_user_id).await?.model().await?;

                if let (Some(since), Some(message)) = (afk_duration, afk_message) {
                    args.set("afk_name", mentioned_user.name.clone());
                    args.set("since", format!("<t:{}:R>", since));
                    args.set("message", if !message.is_empty() {
                        format!(": {}", message)
                    } else {
                        String::new()
                    });
                    let content = client.get_locale_string(
                        &config.locale,
                        "afk-is-afk",
                        Some(&args)
                    );

                    let notify_me = client.get_locale_string(&config.locale, "afk-notifyme", None);

                    let message = client.http
                        .create_message(msg.channel_id)
                        .content(&content)?
                        .components(
                            &vec![
                                Component::ActionRow(ActionRow {
                                    components: vec![
                                        Component::Button(Button {
                                            custom_id: Some(format!("2:{}", mentioned_user_id)),
                                            disabled: false,
                                            emoji: None,
                                            label: Some(notify_me),
                                            style: color_to_button_style("Red"),
                                            url: None,
                                        })
                                    ],
                                })
                            ]
                        )?.await;

                    if let Ok(message) = message {
                        if let Ok(message) = message.model().await {
                            // Schedule a task to update the message after 5 minutes
                            let channel_id = msg.channel_id;
                            tokio::time::sleep(tokio::time::Duration::from_secs(30)).await; // Wait for 5 minutes
                            let _ = client.http
                                .update_message(channel_id, message.id)
                                .components(Some(&vec![]))?.await;
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
