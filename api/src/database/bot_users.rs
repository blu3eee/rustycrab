//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.4

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "bot_users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub balance: i32,
    #[sea_orm(column_name = "prayPoints")]
    pub pray_points: i32,
    #[sea_orm(column_type = "Text")]
    pub inventory: String,
    #[sea_orm(column_name = "botId")]
    pub bot_id: Option<i32>,
    #[sea_orm(column_name = "userId")]
    pub user_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::bot_staffs::Entity")]
    BotStaffs,
    #[sea_orm(
        belongs_to = "super::bots::Entity",
        from = "Column::BotId",
        to = "super::bots::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Bots,
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Users,
}

impl Related<super::bot_staffs::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BotStaffs.def()
    }
}

impl Related<super::bots::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Bots.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
