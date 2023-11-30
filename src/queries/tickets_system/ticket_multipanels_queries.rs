use std::collections::HashSet;

use async_trait::async_trait;
use rustycrab_model::response::{
    ticket::{
        multipanel::{
            RequestCreateTicketMultiPanel,
            RequestUpdateTicketMultiPanel,
            ResponseTicketMultiPanelDetails,
        },
        panel::ResponseTicketPanel,
    },
    bots::ResponseBot,
    guilds::ResponseGuild,
    discord_message::ResponseMessageDetails,
};
use sea_orm::{
    Set,
    DatabaseConnection,
    EntityTrait,
    ActiveValue,
    PrimaryKeyTrait,
    DeleteResult,
    JoinType::LeftJoin,
    QuerySelect,
    Condition,
    RelationTrait,
    ColumnTrait,
    QueryFilter,
};
use twilight_model::{ channel::message::{ Component, component::Button, ReactionType }, id::Id };

use crate::{
    default_queries::DefaultSeaQueries,
    database::{
        ticket_multi_panels::{
            self,
            ActiveModel as TicketMultiPanelActiveModel,
            Entity as TicketMultiPanels,
        },
        ticket_multi_panels_panels_ticket_panels::{
            self as PanelLink,
            Entity as PanelLinks,
            Relation as PanelLinksRelations,
        },
        bots,
        guild_info,
    },
    utilities::{ app_error::AppError, utils::color_to_button_style },
    queries::{
        bot_queries::BotQueries,
        guild_queries::GuildQueries,
        message_queries::MessageQueries,
        message_button_queries::MessageButtonQueries,
    },
};

use super::{
    ticket_panels_links_queries::TicketPanelLinksQueries,
    ticket_panels_queries::TicketPanelsQueries,
};

pub struct TicketMultiPanelQueries {}

impl TicketMultiPanelQueries {
    pub async fn find_panels_by_discord_ids(
        db: &DatabaseConnection,
        bot_discord_id: &str,
        guild_discord_id: &str
    ) -> Result<Vec<<<Self as DefaultSeaQueries>::Entity as EntityTrait>::Model>, AppError> {
        <<Self as DefaultSeaQueries>::Entity as EntityTrait>
            ::find()
            .join(LeftJoin, ticket_multi_panels::Relation::Bots.def())
            .join(LeftJoin, ticket_multi_panels::Relation::GuildInfo.def())
            .filter(
                Condition::all()
                    .add(bots::Column::BotId.eq(bot_discord_id))
                    .add(guild_info::Column::GuildId.eq(guild_discord_id))
            )
            .all(db).await
            .map_err(AppError::from)
    }

    pub async fn fetch_multipanel_details(
        db: &DatabaseConnection,
        id: i32
    ) -> Result<ResponseTicketMultiPanelDetails, AppError> {
        let multipanel = Self::find_by_id(db, id).await?;
        let bot: ResponseBot = BotQueries::find_by_id(db, multipanel.bot_id).await?.into();
        let guild: ResponseGuild = GuildQueries::find_by_id(db, multipanel.guild_id).await?.into();
        let message: Option<ResponseMessageDetails> = if let Some(id) = multipanel.message_id {
            Some(MessageQueries::fetch_message_response(db, id).await?)
        } else {
            None
        };

        let panel_links = PanelLinks::find()
            .join(LeftJoin, PanelLinksRelations::TicketMultiPanels.def())
            .join(LeftJoin, PanelLinksRelations::TicketPanels.def())
            .filter(PanelLink::Column::TicketMultiPanelsId.eq(multipanel.id))
            .all(db).await
            .map_err(AppError::from)?;

        let mut panels: Vec<ResponseTicketPanel> = Vec::new();
        for link in panel_links {
            let panel: ResponseTicketPanel = TicketPanelsQueries::find_by_id(
                db,
                link.ticket_panels_id
            ).await?.into();
            panels.push(panel.into());
        }

        Ok(ResponseTicketMultiPanelDetails {
            id: multipanel.id,
            channel_id: multipanel.channel_id.clone(),
            sent_message_id: multipanel.sent_message_id.clone(),
            bot,
            guild,
            message,
            panels,
        })
    }
}

#[async_trait]
impl DefaultSeaQueries for TicketMultiPanelQueries {
    type Entity = TicketMultiPanels;
    type ActiveModel = TicketMultiPanelActiveModel;

    type CreateData = RequestCreateTicketMultiPanel;
    type UpdateData = RequestUpdateTicketMultiPanel;

    async fn create_entity(
        db: &DatabaseConnection,
        create_data: Self::CreateData
    ) -> Result<<Self::Entity as EntityTrait>::Model, AppError> {
        let bot = BotQueries::find_by_discord_id(db, &create_data.bot_discord_id).await?;
        let guild = GuildQueries::find_one_or_create(db, &create_data.bot_discord_id).await?;

        let message = MessageQueries::create_entity(db, create_data.message_data).await?;

        let active_model = Self::ActiveModel {
            bot_id: Set(bot.id),
            guild_id: Set(guild.id),
            channel_id: Set(create_data.channel_discord_id),
            message_id: Set(Some(message.id)),
            ..Default::default()
        };

        let model = Self::save_active_model(db, active_model).await?;

        Ok(model)
    }

    async fn apply_updates(
        db: &DatabaseConnection,
        active_model: &mut Self::ActiveModel,
        update_data: Self::UpdateData
    ) -> Result<(), AppError> {
        if let Some(data) = update_data.message_data {
            if let ActiveValue::Unchanged(Some(id)) = active_model.message_id {
                MessageQueries::update_by_id(db, id, data).await?;
            }
        }

        if let Some(value) = update_data.channel_discord_id {
            active_model.channel_id = Set(value);
        }

        if let Some(panel_ids) = update_data.panel_ids {
            println!("updating panel ids");
            if let ActiveValue::Unchanged(id) = active_model.id {
                // Get existing panel links
                let existing_panel_links = TicketPanelLinksQueries::get_links(db, id).await?;

                // Determine panels to remove
                let existing_panel_ids: HashSet<i32> = existing_panel_links
                    .into_iter()
                    .map(|link| link.ticket_panels_id)
                    .collect();
                let new_panel_ids: HashSet<i32> = panel_ids.into_iter().collect();

                let panels_to_remove: HashSet<_> = existing_panel_ids
                    .difference(&new_panel_ids)
                    .cloned()
                    .collect();

                // Remove outdated links
                for panel_id in panels_to_remove {
                    println!("delete link {} {}", id, panel_id);
                    TicketPanelLinksQueries::delete(db, id, panel_id).await?;
                }

                // Create new links for panels that aren't already linked
                for panel_id in new_panel_ids {
                    println!("add link {} {}", id, panel_id);
                    if !existing_panel_ids.contains(&panel_id) {
                        TicketPanelLinksQueries::create_entity(db, id, panel_id).await?;
                    }
                }
            }
        }

        Ok(())
    }

    async fn delete_by_id<K>(db: &DatabaseConnection, id: K) -> Result<DeleteResult, AppError>
        where
            K: Into<<<Self::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType> +
                Send +
                Sync
    {
        // Retrieve the ticket panel
        let model = Self::Entity::find_by_id(id.into())
            .one(db).await
            .map_err(AppError::from)?
            .ok_or_else(|| AppError::not_found("Ticket panel not found"))?;

        // Delete related message and button entities
        if let Some(message_id) = model.message_id {
            MessageQueries::delete_by_id(db, message_id).await?;
        }

        TicketPanelLinksQueries::delete_multi_panel_links(db, model.id).await?;

        // Finally, delete the ticket panel
        Self::Entity::delete_by_id(model.id).exec(db).await.map_err(AppError::from)
    }
}

pub async fn create_button_components(
    db: &DatabaseConnection,
    multi_panel_id: i32
) -> Result<Vec<Component>, AppError> {
    let subpanels = TicketPanelLinksQueries::get_subpanels(db, multi_panel_id).await?;

    println!("panel links {:?}", subpanels);

    let mut components = Vec::new();
    for subpanel in subpanels {
        if let Some(button_id) = subpanel.button_id {
            let button: crate::database::buttons::Model = MessageButtonQueries::find_by_id(
                db,
                button_id
            ).await?;

            components.push(
                Component::Button(Button {
                    custom_id: Some(format!("1:1:{}", subpanel.id)),
                    disabled: false,
                    emoji: if button.emoji.len() > 10 {
                        let emoji_id = u64
                            ::from_str_radix(&button.emoji, 10)
                            .map_err(|_| { AppError::bad_request("Invalid emoji ID") })?;
                        Some(ReactionType::Custom {
                            animated: false,
                            id: Id::new(emoji_id),
                            name: None,
                        })
                    } else {
                        Some(ReactionType::Unicode { name: button.emoji })
                    },
                    label: Some(format!("{}", button.text)),
                    style: color_to_button_style(&button.color),
                    url: None,
                })
            );
        }
    }

    Ok(components)
}
