use anyhow::{Ok, Result};
use dotenvy::dotenv;
use sea_orm::{Database, DatabaseConnection};

pub async fn database_connection(database_uri: &str) -> Result<DatabaseConnection> {
    let database = Database::connect(database_uri).await?;
    Ok(database)
}

pub fn get_database_url() -> Result<String> {
    dotenv()?;
    let database_uri = std::env::var("DATABASE_URL")?;
    Ok(database_uri)
}

pub fn get_test_database_url() -> Result<String> {
    dotenv()?;
    let database_uri = std::env::var("TEST_DATABASE_URL")?;
    Ok(database_uri)
}
