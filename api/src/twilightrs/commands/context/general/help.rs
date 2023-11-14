use async_trait::async_trait;
use twilight_model::gateway::payload::incoming::MessageCreate;
use std::{ error::Error, collections::HashMap };

use crate::{
    database::bot_guild_configurations,
    twilightrs::{
        commands::context::{
            ContextCommand,
            context_command_dispatcher::ContextCommandDispatcher,
            ParsedArg,
            ArgSpec,
            ArgType,
            ContextCommandHandler,
        },
        discord_client::{ DiscordClient, MessageContent, ColorTypes },
        embeds::{ DiscordEmbed, DiscordEmbedField },
    },
    cdn_guild_icon,
    cdn_avatar,
    queries::bot_queries,
};

pub struct HelpCommand;

impl HelpCommand {
    async fn display_general_help(
        &self,
        client: &DiscordClient,
        config: &bot_guild_configurations::Model,
        msg: &MessageCreate,
        dispatcher: ContextCommandDispatcher
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let guild = if let Some(guild_id) = msg.guild_id {
            Some(client.http.guild(guild_id).await?.model().await?)
        } else {
            None
        };
        let bot = client.get_bot().await?;
        let bot_id = bot.id.to_string();
        let bot_info = bot_queries::get_bot_from_discord_id(&client.db, &bot_id).await;
        // General help logic
        // Display a list of commands with brief descriptions
        let mut categories: HashMap<String, Vec<String>> = HashMap::new();

        for (command_name, command_handler) in &dispatcher.handlers {
            let category = command_handler.category_name.to_lowercase();
            let command = command_name.to_lowercase();

            categories
                .entry(category)
                .and_modify(|commands| commands.push(command.clone()))
                .or_insert_with(|| vec![command]);
        }
        client.send_message(
            msg.channel_id,
            MessageContent::DiscordEmbeds(
                vec![DiscordEmbed {
                    title: Some(format!("{}'s Commands", bot.name)),
                    description: Some(
                        format!(
                            "More details on a command, use:\n`{}help [command name]`",
                            config.prefix
                        )
                    ),
                    fields: Some(
                        categories
                            .into_iter()
                            .map(|(category, command_names)| {
                                let capitalized_category =
                                    category
                                        .chars()
                                        .next()
                                        .map(|c| c.to_uppercase().collect::<String>())
                                        .unwrap_or_default() + &category[1..];

                                DiscordEmbedField {
                                    name: capitalized_category,
                                    value: command_names
                                        .into_iter()
                                        .filter(|command_name| command_name != "help")
                                        .map(|command_name| format!("`{}` ", command_name))
                                        .collect(),
                                    inline: false,
                                }
                            })
                            .collect()
                    ),
                    author_name: guild.as_ref().map(|guild| guild.name.clone()),
                    author_icon_url: guild
                        .as_ref()
                        .and_then(|guild|
                            guild.icon.map(|icon_hash|
                                cdn_guild_icon!(guild.id.to_string(), icon_hash)
                            )
                        ),
                    thumbnail: bot.avatar.map(|avatar_hash|
                        cdn_avatar!(bot.id.to_string(), avatar_hash)
                    ),
                    timestamp: Some(true),
                    color: if let Ok(info) = bot_info {
                        Some(
                            client.convert_color_u64(
                                ColorTypes::String(format!("{}", &info.theme_hex_color))
                            )
                        )
                    } else {
                        None
                    },
                    ..Default::default()
                }]
            )
        ).await?;

        Ok(())
    }

    async fn display_command_help(
        &self,
        client: &DiscordClient,
        config: &bot_guild_configurations::Model,
        msg: &MessageCreate,
        command_handler: &ContextCommandHandler,
        args: &[String]
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let guild = if let Some(guild_id) = msg.guild_id {
            Some(client.http.guild(guild_id).await?.model().await?)
        } else {
            None
        };

        let bot = client.get_bot().await?;
        let bot_info = bot_queries::get_bot_from_discord_id(&client.db, &bot.id.to_string()).await;

        let (command_usage, command_aliases, subcommand_usage) = command_handler.command.get_help(
            config.prefix.to_string(),
            args
        );

        let _ = client.send_message(
            msg.channel_id,
            MessageContent::DiscordEmbeds(
                vec![DiscordEmbed {
                    timestamp: Some(true),
                    fields: {
                        let mut discord_fields: Vec<DiscordEmbedField> = Vec::new();
                        discord_fields.push(DiscordEmbedField {
                            name: "Category".to_string(),
                            value: format!(
                                "{}",
                                command_handler.category_name
                                    .chars()
                                    .next()
                                    .map(|c| c.to_uppercase().collect::<String>())
                                    .unwrap_or_default() + &command_handler.category_name[1..]
                            ),
                            inline: false,
                        });
                        // if command_handler.command.aliases
                        if command_aliases.len() > 0 {
                            discord_fields.push(DiscordEmbedField {
                                name: "Aliases".to_string(),
                                value: format!("{}", command_aliases.join(", ")),
                                inline: false,
                            });
                        }
                        if command_handler.command.permissions().len() > 0 {
                            discord_fields.push(DiscordEmbedField {
                                name: "Permission(s)".to_string(),
                                value: format!(
                                    "{}",
                                    command_handler.command
                                        .permissions()
                                        .into_iter()
                                        .map(|perm| format!("{:?}", perm).to_lowercase())
                                        .collect::<Vec<String>>()
                                        .join(", ")
                                ),
                                inline: false,
                            });
                        }
                        discord_fields.push(DiscordEmbedField {
                            name: "Usage".to_string(),
                            value: format!("```{}```", command_usage),
                            inline: false,
                        });

                        if subcommand_usage.len() > 0 {
                            discord_fields.push(DiscordEmbedField {
                                name: "Subcommands".to_string(),
                                value: format!(
                                    "```{}```",
                                    subcommand_usage
                                        .into_iter()
                                        .map(|sub| format!("{}{}", config.prefix, sub))
                                        .collect::<Vec<String>>()
                                        .join("\n")
                                ),
                                inline: false,
                            });
                        }
                        Some(discord_fields)
                    },
                    color: if let Ok(info) = bot_info {
                        Some(
                            client.convert_color_u64(
                                ColorTypes::String(format!("{}", &info.theme_hex_color))
                            )
                        )
                    } else {
                        None
                    },
                    thumbnail: bot.avatar.map(|avatar_hash|
                        cdn_avatar!(bot.id.to_string(), avatar_hash)
                    ),
                    author_name: guild.as_ref().map(|guild| guild.name.clone()),
                    author_icon_url: guild
                        .as_ref()
                        .and_then(|guild|
                            guild.icon.map(|icon_hash|
                                cdn_guild_icon!(guild.id.to_string(), icon_hash)
                            )
                        ),
                    ..Default::default()
                }]
            )
        ).await?;

        Ok(())
    }
}

#[async_trait]
impl ContextCommand for HelpCommand {
    fn name(&self) -> &'static str {
        "help"
    }

    fn args(&self) -> Vec<ArgSpec> {
        vec![ArgSpec::new(ArgType::Words, true)] // User argument is optional
    }

    async fn run(
        &self,
        client: &DiscordClient,
        config: &bot_guild_configurations::Model,
        msg: &MessageCreate,
        command_args: Vec<ParsedArg>
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let dispatcher = ContextCommandDispatcher::new();

        match command_args.get(0) {
            Some(ParsedArg::Words(args)) if !args.is_empty() => {
                let command_name = &args[0];
                if let Some(command_name) = dispatcher.commands_aliases.get(command_name) {
                    if let Some(command_handler) = dispatcher.handlers.get(command_name) {
                        let _ = self.display_command_help(
                            client,
                            config,
                            msg,
                            command_handler,
                            &args[1..]
                        ).await?;
                        return Ok(());
                    }
                    // Specific command help logic
                    // Retrieve information about the command and display it
                    // For example: usage, description, examples, etc.
                    // let help_message =
                    //     format!("Help for command '{}': [Command specific help]", command_name);

                    // client.send_message(msg.channel_id, MessageContent::Text(help_message)).await?;
                }
            }
            _ => {
                // No arguments or not a valid command, display general help
            }
        }

        self.display_general_help(client, config, msg, dispatcher).await?;
        Ok(())
    }
}
