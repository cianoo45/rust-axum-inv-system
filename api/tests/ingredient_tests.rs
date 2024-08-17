mod common;
use anyhow::Result;
use axum::http::StatusCode;
use common::{create_test_data, test_database_setup};
use inventorysystem_api::{
    commands::ingredient::{
        create_and_save_ingredient, delete_ingredient_by_id, find_ingredient_by_id,
        update_and_save_ingredient, CreateIngredient, UpdateIngredient,
    },
    errors::AppError,
};

// Run these tests with 'cargo test -- --test-threads=1'
#[tokio::test]
async fn test_get_ingredient() -> Result<()> {
    let test_db = test_database_setup().await?;
    create_test_data(&test_db).await?;

    let ingredient = find_ingredient_by_id(1, &test_db).await?;
    assert!(ingredient.id == 1);

    let ingredient = find_ingredient_by_id(2, &test_db).await;
    assert_eq!(
        ingredient.err().unwrap(),
        AppError::new(StatusCode::NOT_FOUND, "Specified Ingredient Not Found", "",)
    );

    Ok(())
}

#[tokio::test]
async fn test_create_ingredient() -> Result<()> {
    let test_db = test_database_setup().await?;
    let ingredient = CreateIngredient {
        name: "Tomato".to_string(),
        quantity_in_stock: 1,
    };

    create_and_save_ingredient(ingredient, &test_db).await?;

    let ingredient = find_ingredient_by_id(1, &test_db).await?;
    assert!(ingredient.name == "Tomato".to_string());

    Ok(())
}

#[tokio::test]
async fn test_update_ingredient() -> Result<()> {
    let test_db = test_database_setup().await?;
    create_test_data(&test_db).await?;
    let ingredient = UpdateIngredient {
        name: Some("Lettuce".to_string()),
        quantity_in_stock: Some(1),
    };

    update_and_save_ingredient(1, ingredient, &test_db).await?;

    let ingredient = find_ingredient_by_id(1, &test_db).await?;
    assert!(ingredient.name == "Lettuce".to_string());

    Ok(())
}

#[tokio::test]
async fn test_delete_ingredient() -> Result<()> {
    let test_db = test_database_setup().await?;
    create_test_data(&test_db).await?;

    delete_ingredient_by_id(1, &test_db).await?;

    let ingredient = find_ingredient_by_id(1, &test_db).await;
    assert_eq!(
        ingredient.err().unwrap(),
        AppError::new(StatusCode::NOT_FOUND, "Specified Ingredient Not Found", "",)
    );

    Ok(())
}
