use std::error::Error;

use twilight_model::{
    gateway::payload::incoming::InteractionCreate,
    application::interaction::application_command::CommandData,
};

use async_trait::async_trait;
use twilight_model::{ application::command::{ CommandOption, Command }, guild::Permissions };

use crate::twilightrs::discord_client::DiscordClient;

#[async_trait]
pub trait SlashCommand: Send + Sync {
    fn name(&self) -> &'static str;

    fn description(&self) -> &'static str {
        "no description"
    }

    fn command_options(&self) -> Vec<CommandOption> {
        Vec::new()
    }

    fn permissions(&self) -> Option<Permissions> {
        None
    }

    fn nsfw(&self) -> bool {
        false
    }

    async fn register(
        &self,
        client: DiscordClient
    ) -> Result<Command, Box<dyn Error + Sync + Send>> {
        let application = client.http.current_user_application().await?.model().await?;
        let interaction_client = client.http.interaction(application.id);
        let options = self.command_options();
        let mut command = interaction_client
            .create_global_command()
            .chat_input(self.name(), self.description())?;
        if options.len() > 0 {
            command = command.command_options(&options)?;
        }

        if let Some(perm) = self.permissions() {
            command = command.default_member_permissions(perm);
        }

        command = command.nsfw(self.nsfw());

        Ok(command.await?.model().await?)
    }

    async fn exec(
        &self,
        client: DiscordClient,
        interaction: &Box<InteractionCreate>,
        command_data: &Box<CommandData>
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        self.run(client, interaction, command_data).await
    }

    async fn run(
        &self,
        client: DiscordClient,
        interaction: &Box<InteractionCreate>,
        command_data: &Box<CommandData>
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;
}
