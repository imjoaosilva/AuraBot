pub mod queries;
pub mod models;
pub use models::repository::Repository;

use sqlx::{migrate::MigrateDatabase, sqlite::{Sqlite, SqlitePool}};
use anyhow::{Context, Result};

pub async fn init_db(database_url: &str) -> Result<SqlitePool> {
    if !Sqlite::database_exists(&database_url)
        .await
        .unwrap_or(false)
    {
        println!("[INFO] - Creating database {}", database_url);

        Sqlite::create_database(&database_url)
            .await
            .context("Error creating database")?;
    }

    let db = SqlitePool::connect(&database_url)
        .await
        .context("Error connecting to database")?;

    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .context("Error running the migrations")?;

    Ok(db)
}
