use sea_orm::{
    ColumnTrait,
    DatabaseConnection,
    EntityTrait,
    QueryFilter,
    QuerySelect,
    RelationTrait,
    Condition,
    JoinType::LeftJoin,
    ActiveModelTrait,
    Set,
};

use crate::{
    utilities::{ app_error::AppError, convert_seaorm_error::convert_seaorm_error },
    database::{
        bots,
        bot_users::{ Entity as BotUser, Model as BotUserModel, Relation as BotUserRelations, self },
        users,
    },
    routes::bot_users::{ RequestUpdateBotUser, RequestCreateBotUser },
};

use super::{ bot_queries::get_bot_from_discord_id, user_queries::get_user_or_create };

pub async fn get_bot_user(
    db: &DatabaseConnection,
    bot_id: &str,
    user_id: &str
) -> Result<BotUserModel, AppError> {
    BotUser::find()
        .join(LeftJoin, BotUserRelations::Bots.def())
        .join(LeftJoin, BotUserRelations::Users.def())
        .filter(
            Condition::all()
                .add(bots::Column::BotId.eq(bot_id))
                .add(users::Column::DiscordId.eq(user_id))
        )
        .one(db).await
        .map_err(convert_seaorm_error)
        .and_then(|bot_user| bot_user.ok_or_else(|| AppError::not_found("Bot User not found")))
}

pub async fn get_all_bot_users(
    db: &DatabaseConnection,
    bot_id: &str
) -> Result<Vec<BotUserModel>, AppError> {
    BotUser::find()
        .join(LeftJoin, BotUserRelations::Bots.def())
        .join(LeftJoin, BotUserRelations::Users.def())
        .filter(Condition::all().add(bots::Column::BotId.eq(bot_id)))
        .all(db).await
        .map_err(convert_seaorm_error)
}

pub async fn update_bot_user(
    db: &DatabaseConnection,
    bot_id: &str,
    user_id: &str,
    update_dto: &RequestUpdateBotUser
) -> Result<BotUserModel, AppError> {
    let mut bot_user: bot_users::ActiveModel = get_bot_user(db, bot_id, user_id).await?.into();

    // Update the fields if they have Some value
    if let Some(balance) = update_dto.balance {
        bot_user.balance = Set(balance);
    }
    if let Some(locale) = update_dto.pray_points {
        bot_user.pray_points = Set(locale);
    }
    // if let Some(module_flags) = update_dto.inventory {
    //     bot_user.inventory = Set(module_flags);
    // }

    bot_user.update(db).await.map_err(convert_seaorm_error)

    // Ok(bot_user)
}

pub async fn get_bot_user_or_create(
    db: &DatabaseConnection,
    bot_id: &str,
    user_id: &str
) -> Result<BotUserModel, AppError> {
    match get_bot_user(db, bot_id, user_id).await {
        Ok(bot_user) => Ok(bot_user),
        Err(_) =>
            create_bot_user(db, RequestCreateBotUser {
                bot_id: bot_id.to_owned(),
                user_id: user_id.to_owned(),
            }).await,
    }
}

pub async fn create_bot_user(
    db: &DatabaseConnection,
    create_dto: RequestCreateBotUser
) -> Result<BotUserModel, AppError> {
    if
        let Ok(Some(bot_user)) = BotUser::find()
            .filter(Condition::all().add(bots::Column::BotId.eq(&create_dto.bot_id)))
            .filter(Condition::all().add(users::Column::DiscordId.eq(&create_dto.user_id)))
            .one(db).await
    {
        return Ok(bot_user);
    }

    let bot = get_bot_from_discord_id(db, &create_dto.bot_id).await?;
    let user = get_user_or_create(db, &create_dto.user_id).await?;

    let bot_user: bot_users::ActiveModel = bot_users::ActiveModel {
        bot_id: Set(Some(bot.id)),
        user_id: Set(Some(user.id)),
        ..Default::default()
    };

    bot_user.insert(db).await.map_err(convert_seaorm_error)
}

// pub async fn delete_bot_user(db: &DatabaseConnection, bot_id: &str, user_id: &str) {}
