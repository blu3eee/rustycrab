//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.4

use super::sea_orm_active_enums::Role;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "bot_staffs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub role: Role,
    #[sea_orm(column_name = "botUserId", unique)]
    pub bot_user_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bot_users::Entity",
        from = "Column::BotUserId",
        to = "super::bot_users::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    BotUsers,
}

impl Related<super::bot_users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BotUsers.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
