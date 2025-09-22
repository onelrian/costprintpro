use sea_orm::{Database as SeaDatabase, DatabaseConnection, DbErr};
use sea_orm_migration::MigratorTrait;
use tracing::info;

pub mod migration_simple;
use migration_simple::Migrator;

#[derive(Clone)]
pub struct Database {
    pub connection: DatabaseConnection,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, DbErr> {
        info!("Connecting to database: {}", database_url);
        let connection = SeaDatabase::connect(database_url).await?;
        
        Ok(Database { connection })
    }

    pub async fn migrate(&self) -> Result<(), DbErr> {
        info!("Running database migrations");
        Migrator::up(&self.connection, None).await?;
        info!("Database migrations completed successfully");
        Ok(())
    }
}
