use sea_orm::{
    DatabaseConnection,
    Set,
    EntityTrait,
    QuerySelect,
    JoinType::LeftJoin,
    RelationTrait,
    QueryFilter,
    Condition,
    ColumnTrait,
};
use async_trait::async_trait;
use rustycrab_model::response::ticket::support_team::{
    RequestCreateTicketSupportTeam,
    RequestUpdateTicketSupportTeam,
};

use crate::{
    default_queries::DefaultSeaQueries,
    database::{
        ticket_support_teams::{
            Entity as TicketSupportTeams,
            ActiveModel as TicketSupportTeamActiveModel,
            self,
        },
        bots,
        guild_info,
    },
    utilities::app_error::AppError,
    queries::{ bot_queries::BotQueries, guild_queries::GuildQueries },
};

pub struct TicketSupportTeamQueries {}

impl TicketSupportTeamQueries {
    pub async fn find_guild_support_teams_by_discord_ids(
        db: &DatabaseConnection,
        bot_discord_id: &str,
        guild_discord_id: &str
    ) -> Result<Vec<<<Self as DefaultSeaQueries>::Entity as EntityTrait>::Model>, AppError> {
        <<Self as DefaultSeaQueries>::Entity as EntityTrait>
            ::find()
            .join(LeftJoin, ticket_support_teams::Relation::Bots.def())
            .join(LeftJoin, ticket_support_teams::Relation::GuildInfo.def())
            .filter(
                Condition::all()
                    .add(bots::Column::BotId.eq(bot_discord_id))
                    .add(guild_info::Column::GuildId.eq(guild_discord_id))
            )
            .all(db).await
            .map_err(AppError::from)
    }

    pub async fn find_support_team_by_name(
        db: &DatabaseConnection,
        bot_discord_id: &str,
        guild_discord_id: &str,
        name: &str
    ) -> Result<<<Self as DefaultSeaQueries>::Entity as EntityTrait>::Model, AppError> {
        <<Self as DefaultSeaQueries>::Entity as EntityTrait>
            ::find()
            .join(LeftJoin, ticket_support_teams::Relation::Bots.def())
            .join(LeftJoin, ticket_support_teams::Relation::GuildInfo.def())
            .filter(
                Condition::all()
                    .add(bots::Column::BotId.eq(bot_discord_id))
                    .add(guild_info::Column::GuildId.eq(guild_discord_id))
                    .add(ticket_support_teams::Column::Name.eq(name))
            )
            .one(db).await
            .map_err(AppError::from)?
            .ok_or_else(|| AppError::not_found("Support team not found"))
    }

    pub async fn update_support_team_by_name(
        db: &DatabaseConnection,
        bot_discord_id: &str,
        guild_discord_id: &str,
        name: &str,
        update_data: <Self as DefaultSeaQueries>::UpdateData
    ) -> Result<<<Self as DefaultSeaQueries>::Entity as EntityTrait>::Model, AppError> {
        let model = Self::find_support_team_by_name(
            db,
            bot_discord_id,
            guild_discord_id,
            name
        ).await?;
        let mut active_model: <Self as DefaultSeaQueries>::ActiveModel = model.into();

        Self::apply_updates(db, &mut active_model, update_data).await?;

        Self::save_active_model(db, active_model).await
    }
}

#[async_trait]
impl DefaultSeaQueries for TicketSupportTeamQueries {
    type Entity = TicketSupportTeams;

    type ActiveModel = TicketSupportTeamActiveModel;

    type CreateData = RequestCreateTicketSupportTeam;
    type UpdateData = RequestUpdateTicketSupportTeam;

    async fn create_entity(
        db: &DatabaseConnection,
        create_data: Self::CreateData
    ) -> Result<<Self::Entity as EntityTrait>::Model, AppError> {
        let bot = BotQueries::find_by_discord_id(db, &create_data.bot_discord_id).await?;
        let guild = GuildQueries::find_one_or_create(db, &create_data.guild_discord_id).await?;

        Self::save_active_model(db, Self::ActiveModel {
            bot_id: Set(bot.id),
            guild_id: Set(guild.id),
            name: Set(create_data.name),
            ..Default::default()
        }).await
    }

    async fn apply_updates(
        _: &DatabaseConnection,
        active_model: &mut Self::ActiveModel,
        update_data: Self::UpdateData
    ) -> Result<(), AppError> {
        if let Some(value) = update_data.name {
            active_model.name = Set(value);
        }

        if let Some(value) = update_data.roles {
            active_model.roles = Set(value.join(","));
        }
        if let Some(value) = update_data.users {
            active_model.users = Set(value.join(","));
        }

        Ok(())
    }
}
