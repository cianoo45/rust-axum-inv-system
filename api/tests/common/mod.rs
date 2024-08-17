use anyhow::Result;
use entity::ingredient;
use inventorysystem_api::database::{database_connection, get_test_database_url};
use migration::{Migrator, MigratorTrait};
use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, DatabaseConnection, Set};

pub async fn test_database_setup() -> Result<DatabaseConnection> {
    let test_db_url = get_test_database_url()?;
    let test_db = database_connection(&test_db_url).await?;

    // Drop all tables and re-run all migrations
    Migrator::fresh(&test_db).await?;

    Ok(test_db)
}

pub async fn create_test_data(db: &DatabaseConnection) -> Result<()> {
    let ingredient = ingredient::ActiveModel {
        id: NotSet,
        name: Set("Tomato".to_string()),
        quantity_in_stock: Set(1),
    };

    ingredient.save(db).await?;
    Ok(())
}
