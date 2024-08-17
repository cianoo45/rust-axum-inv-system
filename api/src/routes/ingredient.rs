use crate::commands::ingredient::{
    create_and_save_ingredient, find_ingredient_by_id, update_and_save_ingredient,
    CreateIngredient, UpdateIngredient,
};
use crate::errors::AppError;
use axum::{extract::Path, Extension, Json};
use entity::ingredient::Entity as Ingredients;
use entity::ingredient::Model as Ingredient;
use sea_orm::{DatabaseConnection, EntityTrait};

pub async fn get_ingredient(
    Path(ingredient_id): Path<i32>,
    Extension(db): Extension<DatabaseConnection>,
) -> Result<Json<Ingredient>, AppError> {
    let ingredient = find_ingredient_by_id(ingredient_id, &db).await?;

    Ok(Json(ingredient))
}

pub async fn create_ingredient(
    Extension(db): Extension<DatabaseConnection>,
    Json(data): Json<CreateIngredient>,
) -> Result<(), AppError> {
    create_and_save_ingredient(data, &db).await?;

    Ok(())
}

pub async fn update_ingredient(
    Extension(db): Extension<DatabaseConnection>,
    Path(ingredient_id): Path<i32>,
    Json(data): Json<UpdateIngredient>,
) -> Result<(), AppError> {
    update_and_save_ingredient(ingredient_id, data, &db).await?;

    Ok(())
}

pub async fn delete_ingredient(
    Path(ingredient_id): Path<i32>,
    Extension(db): Extension<DatabaseConnection>,
) -> Result<(), AppError> {
    Ingredients::delete_by_id(ingredient_id).exec(&db).await?;

    Ok(())
}
