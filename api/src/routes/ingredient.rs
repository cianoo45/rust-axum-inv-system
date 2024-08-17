use crate::commands::ingredient::{
    create_and_save_ingredient, delete_ingredient_by_id, find_ingredient_by_id,
    update_and_save_ingredient, CreateIngredient, UpdateIngredient,
};
use crate::errors::Result;
use axum::{extract::Path, Extension, Json};
use entity::ingredient::Model as Ingredient;
use sea_orm::DatabaseConnection;

pub async fn get_ingredient(
    Path(ingredient_id): Path<i32>,
    Extension(db): Extension<DatabaseConnection>,
) -> Result<Json<Ingredient>> {
    let ingredient = find_ingredient_by_id(ingredient_id, &db).await?;

    Ok(Json(ingredient))
}

pub async fn create_ingredient(
    Extension(db): Extension<DatabaseConnection>,
    Json(data): Json<CreateIngredient>,
) -> Result<()> {
    create_and_save_ingredient(data, &db).await?;

    Ok(())
}

pub async fn update_ingredient(
    Extension(db): Extension<DatabaseConnection>,
    Path(ingredient_id): Path<i32>,
    Json(data): Json<UpdateIngredient>,
) -> Result<()> {
    update_and_save_ingredient(ingredient_id, data, &db).await?;

    Ok(())
}

pub async fn delete_ingredient(
    Path(ingredient_id): Path<i32>,
    Extension(db): Extension<DatabaseConnection>,
) -> Result<()> {
    delete_ingredient_by_id(ingredient_id, &db).await?;

    Ok(())
}
