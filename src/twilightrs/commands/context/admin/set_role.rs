use async_trait::async_trait;
use fluent_bundle::FluentArgs;
use twilight_http::Client;
use twilight_model::{
    gateway::payload::incoming::MessageCreate,
    guild::Permissions,
    id::{ marker::{ GuildMarker, UserMarker, RoleMarker }, Id },
};
use std::error::Error;
use crate::{
    twilightrs::{
        commands::context::{
            ContextCommand,
            ParsedArg,
            ArgSpec,
            ArgType,
            context_command::GuildConfigModel,
        },
        discord_client::DiscordClient,
        messages::DiscordEmbed,
        utils::send_command_response,
    },
    utilities::utils::ColorResolvables,
};

pub struct RoleCommand;

#[async_trait]
impl ContextCommand for RoleCommand {
    fn name(&self) -> &'static str {
        "role"
    }

    fn aliases(&self) -> Vec<&'static str> {
        vec!["setrole"]
    }

    fn args(&self) -> Vec<ArgSpec> {
        vec![
            ArgSpec::new("users", ArgType::Users, false),
            ArgSpec::new("role", ArgType::Text, false)
        ]
    }

    fn permissions(&self) -> Vec<Permissions> {
        vec![Permissions::MANAGE_ROLES]
    }

    async fn run(
        &self,
        client: DiscordClient,
        config: &GuildConfigModel,
        msg: &MessageCreate,
        command_args: Vec<ParsedArg>
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let guild_id = msg.guild_id.ok_or(
            client.get_locale_string(&config.locale, "command-guildonly", None)
        )?;
        if let Some(ParsedArg::Users(users)) = command_args.get(0) {
            if let Some(ParsedArg::Text(role_arg)) = command_args.get(1) {
                // Find the role by name, ID, or mention
                let role = client.find_role(guild_id, role_arg).await?;
                let mut args = FluentArgs::new();
                args.set("role", format!("<@&{}>", role.id.to_string()));
                if
                    !can_bot_manage_role(
                        &client.http,
                        guild_id,
                        client.get_bot().await?.id,
                        role.id
                    ).await?
                {
                    let message = client.get_locale_string(
                        &config.locale,
                        "command-role-no-perm",
                        Some(&args)
                    );
                    let _ = client.reply_message(
                        msg.channel_id,
                        msg.id,
                        crate::twilightrs::discord_client::MessageContent::DiscordEmbeds(
                            vec![DiscordEmbed {
                                description: Some(message),
                                color: Some(ColorResolvables::Red.as_u32()),
                                ..Default::default()
                            }]
                        )
                    ).await;
                    return Ok(());
                }

                for user in users {
                    let mut args = FluentArgs::new();
                    args.set("role", format!("<@&{}>", role.id.to_string()));
                    args.set("user", format!("<@{}>", user.id.to_string()));
                    // Check if the user has the role
                    let has_role = client.user_has_role(guild_id, user.id, role.id).await?;

                    // Add or remove the role based on whether the user already has it
                    let (key, color) = if has_role {
                        match
                            client.http.remove_guild_member_role(guild_id, user.id, role.id).await
                        {
                            Ok(_) => { ("command-role-remove-success", ColorResolvables::Green) }
                            Err(e) => {
                                args.set("err", format!("{:?}", e));
                                ("command-role-remove-fail", ColorResolvables::Red)
                            }
                        }
                    } else {
                        match client.http.add_guild_member_role(guild_id, user.id, role.id).await {
                            Ok(_) => { ("command-role-add-success", ColorResolvables::Green) }
                            Err(e) => {
                                args.set("err", format!("{:?}", e));
                                ("command-role-add-fail", ColorResolvables::Red)
                            }
                        }
                    };
                    let _ = send_command_response(
                        &client,
                        &config,
                        &msg,
                        key,
                        Some(args),
                        color
                    ).await;
                }
            }
        }

        Ok(())
    }
}

async fn can_bot_manage_role(
    http_client: &Client,
    guild_id: Id<GuildMarker>,
    bot_id: Id<UserMarker>,
    target_role_id: Id<RoleMarker>
) -> Result<bool, Box<dyn Error + Send + Sync>> {
    // Fetch the bot's member information
    let bot_member = http_client.guild_member(guild_id, bot_id).await?.model().await?;

    // Fetch all roles in the guild
    let roles = http_client.roles(guild_id).await?.model().await?;

    // Determine the bot's highest role position
    let bot_highest_role_position = bot_member.roles
        .iter()
        .filter_map(|role_id| roles.iter().find(|role| &role.id == role_id))
        .map(|role| role.position)
        .max()
        .unwrap_or(0);

    // Fetch the target role information
    let target_role = roles
        .into_iter()
        .find(|role| role.id == target_role_id)
        .ok_or("Target role not found")?;

    // Check if the bot's highest role is higher than the target role
    Ok(bot_highest_role_position > target_role.position)
}
