use database_migration::migrations;

use dotenv::dotenv;
use sea_orm::Database;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url_old: String = env
        ::var("DATABASE_URL_NEW")
        .expect("DATABASE_URL_NEW must be set in the .env file");
    let database_url_new: String = env
        ::var("DATABASE_URL_BOTS")
        .expect("DATABASE_URL_BOTS must be set in the .env file");

    println!("connecting database");

    // Establish a database connection
    let db_old = match Database::connect(database_url_old).await {
        Ok(db) => db,
        Err(error) => {
            print!("Error connecting to the database: {:?}", error);
            panic!();
        }
    };

    // Establish a database connection
    let db_new = match Database::connect(database_url_new).await {
        Ok(db) => db,
        Err(error) => {
            print!("Error connecting to the database: {:?}", error);
            panic!();
        }
    };

    // let _ = migrations::bot_config::migrate(db_old.clone(), db_new.clone()).await;
    // let _ = migrations::items::migrate(db_old.clone(), db_new.clone()).await;
    // let _ = migrations::bot_users::migrate(db_old.clone(), db_new.clone()).await;
    let _ = migrations::auto_res::migrate(db_old.clone(), db_new.clone()).await;
}
