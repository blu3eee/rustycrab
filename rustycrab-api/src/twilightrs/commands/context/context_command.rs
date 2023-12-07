use crate::{
    database::bot_guild_configurations,
    twilightrs::{
        discord_client::{ DiscordClient, MessageContent },
        utils::greedy::{ greedy_user, greedy_users, greedy_channel, greedy_channels },
        messages::DiscordEmbed,
    },
    locales::{ load_localization, get_localized_string },
};

use rustycrab_model::color::ColorResolvables;
use twilight_http::Client as HttpClient;
use twilight_model::{
    gateway::payload::incoming::MessageCreate,
    user::User,
    channel::Channel,
    guild::Permissions,
};
use std::{ error::Error, sync::Arc };

use async_trait::async_trait;

use super::{ ArgSpec, ParsedArg, ArgType };

pub type GuildConfigModel = bot_guild_configurations::Model;

/// Trait defining the structure and behavior of a context command.
///
/// A context command represents an actionable command in a Discord bot's context. This trait provides
/// the necessary interface for creating such commands, handling their execution, and managing subcommands.
/// Implementors of this trait must define at least the `name` and `run` methods. Other methods like `aliases`,
/// `args`, `subcommands`, `exec`, and `parse_args` are provided with default implementations but can be
/// overridden as needed.
///
/// ## Required Methods
///
/// - `name`: Specifies the primary name of the command.
///
/// ## Expected
/// - `run`: Contains the logic that will be executed when the command is invoked.
/// - `subcommands`: Allows nesting of commands within a parent command, enabling hierarchical command structures.
///
/// Inorder for the command to run, you need to either implement the run function or create at least one command
/// for subcommands
///
/// ## Optional Methods
///
/// - `aliases`: Returns a list of alternative names for the command.
/// - `args`: Defines the arguments that the command expects. Useful for automatic argument parsing.
/// - `subcommands`: Allows nesting of commands within a parent command, enabling hierarchical command structures.
/// - `parse_args`: Handles the parsing of command arguments based on the specifications provided in `args`.
#[async_trait]
pub trait ContextCommand: Send + Sync {
    fn name(&self) -> &'static str;

    fn aliases(&self) -> Vec<&'static str> {
        Vec::new()
    }

    fn permissions(&self) -> Vec<Permissions> {
        Vec::new()
    }

    fn description(&self, locale: &str) -> Option<String> {
        let bundle = load_localization(&locale);
        get_localized_string(&bundle, &format!("command-{}", self.name()), None)
    }

    fn args(&self) -> Vec<ArgSpec> {
        Vec::new()
    }

    fn subcommands(&self) -> Vec<Box<dyn ContextCommand>> {
        Vec::new()
    }

    fn parent_command(&self) -> Option<Box<dyn ContextCommand>> {
        None
    }

    #[allow(unused_variables)]
    async fn run(
        &self,
        client: DiscordClient,
        config: &GuildConfigModel,
        msg: &MessageCreate,
        command_args: Vec<ParsedArg>
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        Ok(())
    }

    /// Executes a context command with preprocessing.
    ///
    /// This function serves as the entry point for executing any command that implements the `ContextCommand` trait.
    /// It first checks if the message sender have the required permissions to execute the command.
    /// Then, it check the presence of subcommands. If a subcommand is specified in the command arguments (`cmd_args`),
    /// it delegates the execution to that subcommand. Otherwise, it proceeds with the current command execution.
    ///
    /// # Arguments
    ///
    /// * `client` - A reference to the `DiscordClient` which provides functionalities to interact with Discord's API.
    /// * `config` - A reference to the guild configuration model, which contains configuration details relevant to the command execution.
    /// * `msg` - The `MessageCreate` payload received from Discord, containing details about the message that triggered the command.
    /// * `cmd_args` - A slice of strings representing the arguments passed with the command. The first argument is typically the command or subcommand name.
    ///
    /// # Returns
    ///
    /// This function returns a `Result` type. On successful execution of the command (or a subcommand), it returns `Ok(())`.
    /// If an error occurs during the parsing of arguments or execution of the command, it returns an `Err` containing the error details.
    ///
    /// # Async
    ///
    /// This function is an `async` function. It awaits the resolution of asynchronous operations such as fetching data from a database,
    /// interacting with Discord's API, or processing command arguments.
    async fn exec(
        &self,
        client: DiscordClient,
        config: &GuildConfigModel,
        msg: &MessageCreate,
        cmd_args: &[&str]
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        // Check permissions
        if let Some(_) = msg.guild_id {
            let required_permissions = self.permissions();
            if !required_permissions.is_empty() {
                let user_permissions = client.cache
                    .permissions()
                    .in_channel(msg.author.id, msg.channel_id)?;

                let has_permission = required_permissions
                    .iter()
                    .any(|&req_perm| user_permissions.contains(req_perm));
                if !has_permission {
                    // User does not have any of the required permissions
                    println!("User does not have required permissions");
                    return Ok(()); // or return an appropriate error
                }
            }
        } else {
            // denies commands that wasn't coming from a guild
            return Ok(());
        }

        // run the command or subcommands
        if !self.subcommands().is_empty() && !cmd_args.is_empty() {
            let command_name = cmd_args[0];
            for subcommand in self.subcommands() {
                if
                    subcommand.name() == command_name ||
                    subcommand.aliases().contains(&command_name)
                {
                    return subcommand.exec(client, config, msg, &cmd_args[1..]).await;
                }
            }
        }

        // parse the arguments for the command if there is any
        let arg_specs = self.args();
        let parsed_args = self.parse_args(cmd_args, &arg_specs, &client.http).await;
        match parsed_args {
            // if the arguments are successfully parsed, we run the command
            Ok(args) => {
                if let Err(err) = self.run(Arc::clone(&client), config, msg, args).await {
                    let content = client.get_locale_string(&config.locale, &err.to_string(), None);
                    // if an error happened with the command, prompt the user of the erro
                    client.reply_message(
                        msg.channel_id,
                        msg.id,
                        MessageContent::DiscordEmbeds(
                            vec![DiscordEmbed {
                                description: Some(content),
                                color: Some(ColorResolvables::Red.as_u32()),
                                ..Default::default()
                            }]
                        )
                    ).await?;
                }
            }
            // if the message command does not have correct arguments, prompt user the command usage
            Err(_) => {
                let _ = client.reply_message(
                    msg.channel_id,
                    msg.id,
                    crate::twilightrs::discord_client::MessageContent::Text(
                        format!(
                            "```fix\n{}```",
                            self
                                .get_full_command()
                                .into_iter()
                                .map(|usage| format!("{}{}", config.prefix, usage))
                                .collect::<Vec<String>>()
                                .join("\n")
                        )
                    )
                ).await;
            }
        }

        Ok(())
    }

    /// Function to parse command arguments
    async fn parse_args(
        &self,
        cmd_args: &[&str],
        arg_specs: &Vec<ArgSpec>,
        http: &HttpClient
    ) -> Result<Vec<ParsedArg>, Box<dyn Error + Send + Sync>> {
        let mut parsed_args: Vec<ParsedArg> = Vec::new();
        let mut remaining_args: &[&str] = cmd_args;

        for (_, arg_spec) in arg_specs.iter().enumerate() {
            match arg_spec.arg_type {
                ArgType::Arg => {
                    if let Some(arg) = remaining_args.first() {
                        parsed_args.push(ParsedArg::Arg(arg.to_string()));
                        remaining_args = &remaining_args[1..];
                    } else if !arg_spec.optional {
                        return Err("Missing required number argument".into());
                    }
                }
                ArgType::Args => {
                    if remaining_args.is_empty() && !arg_spec.optional {
                        return Err("Missing required string argument".into());
                    }
                    if !remaining_args.is_empty() {
                        parsed_args.push(
                            ParsedArg::Args(
                                remaining_args
                                    .iter()
                                    .map(|arg| arg.to_string())
                                    .collect()
                            )
                        );
                    }
                    break; // Consume all remaining arguments
                }
                ArgType::Text => {
                    if remaining_args.is_empty() && !arg_spec.optional {
                        return Err("Missing required string argument".into());
                    }
                    let concatenated_string: String = remaining_args.join(" ");
                    parsed_args.push(ParsedArg::Text(concatenated_string));
                    break; // Consume all remaining arguments
                }
                ArgType::Number => {
                    if let Some(arg) = remaining_args.first() {
                        if let Ok(number) = arg.parse::<i64>() {
                            parsed_args.push(ParsedArg::Number(number));
                            remaining_args = &remaining_args[1..];
                        } else if !arg_spec.optional {
                            return Err("Invalid number argument".into());
                        }
                    } else if !arg_spec.optional {
                        return Err("Missing required number argument".into());
                    }
                }

                ArgType::User => {
                    let (user, args) = greedy_user(http, remaining_args).await;
                    if let Some(user) = user {
                        parsed_args.push(ParsedArg::User(user));
                    } else if !arg_spec.optional {
                        return Err("Missing required user argument".into());
                    }
                    remaining_args = args;
                }
                ArgType::Users => {
                    let (users, args) = greedy_users(http, remaining_args).await;
                    let user_ids: Vec<User> = users
                        .into_iter()
                        .map(|user| user)
                        .collect();
                    parsed_args.push(ParsedArg::Users(user_ids));
                    remaining_args = args;
                }
                ArgType::Channel => {
                    let (channel, args) = greedy_channel(http, remaining_args).await;
                    if let Some(channel) = channel {
                        parsed_args.push(ParsedArg::Channel(channel));
                    } else if !arg_spec.optional {
                        return Err("Missing required user argument".into());
                    }
                    remaining_args = args;
                }
                ArgType::Channels => {
                    let (channels, args) = greedy_channels(http, remaining_args).await;
                    let channels: Vec<Channel> = channels
                        .into_iter()
                        .map(|channel| channel)
                        .collect();
                    parsed_args.push(ParsedArg::Channels(channels));
                    remaining_args = args;
                }
            }
        }

        Ok(parsed_args)
    }

    fn get_help(
        &self,
        locale: &str,
        parent_prefix: String,
        args: &[String]
    ) -> (String, Vec<String>, Vec<String>, Option<String>) {
        let subcommands = self.subcommands();
        if let Some(arg) = args.first() {
            if
                let Some(subcommand) = subcommands
                    .iter()
                    .find(
                        |&subcmd| (subcmd.name() == arg || subcmd.aliases().contains(&arg.as_str()))
                    )
            {
                return subcommand.get_help(
                    locale,
                    format!("{}{} ", parent_prefix, self.name()),
                    &args[1..]
                );
            }
        }

        (
            format!("{}{} {}", parent_prefix, self.name(), self.get_args_string()),
            self
                .aliases()
                .into_iter()
                .map(|a| a.to_string())
                .collect(),
            self.get_full_command()[1..].to_vec(),
            self.description(locale),
        )
    }

    fn get_args_string(&self) -> String {
        self.args()
            .into_iter()
            .map(|arg| arg.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }

    fn get_root_command(&self) -> String {
        if let Some(parent_command) = self.parent_command() {
            return format!("{} {}", parent_command.get_root_command(), self.name());
        }

        self.name().to_string()
    }

    fn get_full_command(&self) -> Vec<String> {
        let root_command = self.get_root_command();
        let mut result: Vec<String> = vec![format!("{} {}", root_command, self.get_args_string())];

        if !root_command.is_empty() {
            for subcommand in self.subcommands() {
                result.extend(
                    subcommand
                        .get_usage()
                        .into_iter()
                        .map(|usage| format!("{} {}", root_command, usage))
                );
            }
        } else {
            for subcommand_usage in self.get_usage() {
                result.push(subcommand_usage.to_string());
            }
        }

        result
    }

    fn get_usage(&self) -> Vec<String> {
        let mut usages: Vec<String> = vec![format!("{} {}", self.name(), self.get_args_string())];
        for subcommand in self.subcommands() {
            usages.extend(
                subcommand
                    .get_usage()
                    .into_iter()
                    .map(|usage| format!("{} {}", self.name(), usage))
            );
        }
        usages
    }
}
