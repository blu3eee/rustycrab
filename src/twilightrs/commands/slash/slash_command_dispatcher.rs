use std::{ error::Error, sync::Arc, collections::HashMap };

use twilight_model::{
    application::interaction::application_command::CommandData,
    gateway::payload::incoming::InteractionCreate,
};

use crate::twilightrs::discord_client::DiscordClient;

use super::{ slash_command::SlashCommand, tickets::close_ticket::CloseTicketSlashCommand };

pub struct SlashCommandDispatcher {
    pub commands: HashMap<String, Box<dyn SlashCommand>>,
}

impl SlashCommandDispatcher {
    pub fn new() -> Self {
        let mut commands: HashMap<String, Box<dyn SlashCommand>> = HashMap::new();
        let slash_commands: Vec<Box<dyn SlashCommand>> = vec![Box::new(CloseTicketSlashCommand {})];
        for command in slash_commands {
            commands.entry(command.name().to_string()).or_insert(command);
        }
        Self {
            commands,
        }
    }

    pub async fn register_commands(
        &self,
        client: &Arc<DiscordClient>
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        for (_, command) in &self.commands {
            if let Ok(register_command) = command.register(client).await {
                println!("Registered command {}", register_command.name);
            } else {
                eprintln!("Failed to register command {}", command.name());
            }
        }

        Ok(())
    }

    pub async fn dispatch(
        &self,
        client: &Arc<DiscordClient>,
        interaction: &Box<InteractionCreate>,
        command_data: &Box<CommandData>
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(command) = self.commands.get(&command_data.name) {
            let _ = command.exec(client, interaction, command_data).await;
        }
        Ok(())
    }
}
