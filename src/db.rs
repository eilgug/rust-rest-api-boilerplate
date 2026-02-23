use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection, DbErr};

use crate::config::Config;

pub async fn connect(config: &Config) -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(&config.database_url).await?;
    tracing::info!("Connected to database");

    Migrator::up(&db, None).await?;
    tracing::info!("Migrations applied successfully");

    Ok(db)
}
