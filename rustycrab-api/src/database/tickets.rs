//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.4

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "tickets")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_name = "userID")]
    pub user_id: String,
    #[sea_orm(column_name = "openedTime")]
    pub opened_time: i32,
    #[sea_orm(column_name = "panelID")]
    pub panel_id: i32,
    #[sea_orm(column_name = "channelID")]
    pub channel_id: Option<String>,
    pub status: Option<String>,
    #[sea_orm(column_name = "notificationMessageID")]
    pub notification_message_id: Option<String>,
    #[sea_orm(column_name = "transcriptChannelID")]
    pub transcript_channel_id: Option<String>,
    #[sea_orm(column_name = "transcriptMessageID")]
    pub transcript_message_id: Option<String>,
    #[sea_orm(column_name = "botId")]
    pub bot_id: i32,
    #[sea_orm(column_name = "guildId")]
    pub guild_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bots::Entity",
        from = "Column::BotId",
        to = "super::bots::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Bots,
    #[sea_orm(
        belongs_to = "super::guild_info::Entity",
        from = "Column::GuildId",
        to = "super::guild_info::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    GuildInfo,
}

impl Related<super::bots::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Bots.def()
    }
}

impl Related<super::guild_info::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GuildInfo.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}