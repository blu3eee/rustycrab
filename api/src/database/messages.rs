//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.4

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "messages")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Text", nullable)]
    pub content: Option<String>,
    pub r#type: String,
    #[sea_orm(column_name = "embedId", unique)]
    pub embed_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::auto_responses::Entity")]
    AutoResponses,
    #[sea_orm(has_one = "super::bot_guild_welcomes::Entity")]
    BotGuildWelcomes,
    #[sea_orm(
        belongs_to = "super::embed_info::Entity",
        from = "Column::EmbedId",
        to = "super::embed_info::Column::Id",
        on_update = "NoAction",
        on_delete = "SetNull"
    )]
    EmbedInfo,
    #[sea_orm(has_one = "super::ticket_multi_panels::Entity")]
    TicketMultiPanels,
}

impl Related<super::auto_responses::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AutoResponses.def()
    }
}

impl Related<super::bot_guild_welcomes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BotGuildWelcomes.def()
    }
}

impl Related<super::embed_info::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EmbedInfo.def()
    }
}

impl Related<super::ticket_multi_panels::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TicketMultiPanels.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
