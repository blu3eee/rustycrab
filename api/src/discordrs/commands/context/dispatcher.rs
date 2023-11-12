use std::collections::HashMap;

use crate::{
    database::bot_guild_configurations::Model as GuildConfig,
    discordrs::{
        client::DiscordClient,
        commands::context::{ ContextCommandCategory, general::GeneralCommands },
    },
};

use discord::model::Message;

use super::ContextCommandHandler;
pub struct ContextCommandDispatcher {
    commands_aliases: HashMap<String, String>,
    handlers: HashMap<String, ContextCommandHandler>,
}

impl ContextCommandDispatcher {
    pub fn new() -> Self {
        println!("creating new command dispatcher");
        let mut handlers: HashMap<String, ContextCommandHandler> = HashMap::new();

        let categories: Vec<Box<dyn ContextCommandCategory>> = Vec::from([
            Box::new(GeneralCommands {}) as Box<dyn ContextCommandCategory>,
        ]);

        let mut commands_aliases: HashMap<String, String> = HashMap::new();

        for category in categories {
            let category_name = category.name();
            for command in category.collect_commands() {
                let command_name = command.name();
                let aliases = command.aliases();

                if let Some(_) = commands_aliases.get(command_name) {
                    println!("Command name conflicted, there are more than one command with name or alias {}", command_name);
                } else {
                    handlers.insert(command_name.to_string(), ContextCommandHandler {
                        command_name,
                        category_name,
                        command: command,
                    });

                    commands_aliases.insert(command_name.to_string(), command_name.to_string());

                    // Register aliases
                    for alias in aliases {
                        if let Some(_) = commands_aliases.get(alias) {
                            println!("Command alias conflicted, there are more than one command with name or alias {}", alias);
                        } else {
                            commands_aliases.insert(alias.to_string(), command_name.to_string());
                        }
                    }
                }
            }
        }

        ContextCommandDispatcher { commands_aliases, handlers }
    }

    pub async fn dispatch_command(
        &self,
        client: &mut DiscordClient,
        command_name: &str,
        config: &GuildConfig,
        message: &Message,
        args: &[&str]
    ) {
        // check commands
        if let Some(name) = self.commands_aliases.get(command_name) {
            // println!("dispatching commands");
            if let Some(handler) = self.handlers.get(name) {
                handler.command.handle_command(client, config, message, args).await;
            } else {
                // handler not mapped
            }
        }
    }
}
