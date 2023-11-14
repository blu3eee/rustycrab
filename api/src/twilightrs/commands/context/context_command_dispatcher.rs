use std::collections::HashMap;

use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::{
    twilightrs::{
        commands::context::{
            ContextCommandCategory,
            general::GeneralCommands,
            ContextCommandHandler,
        },
        discord_client::DiscordClient,
    },
    database::bot_guild_configurations::Model as GuildConfigModel,
};

pub struct ContextCommandDispatcher {
    pub commands_aliases: HashMap<String, String>,
    pub handlers: HashMap<String, ContextCommandHandler>,
}

impl ContextCommandDispatcher {
    pub fn new() -> Self {
        println!("creating new command dispatcher");
        let mut handlers: HashMap<String, ContextCommandHandler> = HashMap::new();

        let categories = Vec::from([
            Box::new(GeneralCommands {}) as Box<dyn ContextCommandCategory>,
        ]);

        let mut commands_aliases: HashMap<String, String> = HashMap::new();

        for category in categories {
            let category_name = category.name();
            for command in category.collect_commands() {
                let command_name: &str = command.name();
                let aliases: Vec<&str> = command.aliases();

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
        client: &DiscordClient,
        config: &GuildConfigModel,
        message: &MessageCreate,
        command_name: &str,
        args: &[&str]
    ) {
        // check commands
        if let Some(name) = self.commands_aliases.get(command_name) {
            // println!("dispatching commands");
            if let Some(handler) = self.handlers.get(name) {
                let _ = handler.command.exec(client, config, message, args).await;
            } else {
                // handler not mapped
            }
        }
    }

    pub async fn get_categories(&self) -> Vec<Box<dyn ContextCommandCategory>> {
        Vec::from([Box::new(GeneralCommands {}) as Box<dyn ContextCommandCategory>])
    }
}
