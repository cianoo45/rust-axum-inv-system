// Ingredient Commands to decouple the route handler from the data access logic

use crate::errors::AppError;
use axum::http::StatusCode;
use entity::ingredient;
use entity::ingredient::Entity as Ingredients;
use entity::ingredient::Model as Ingredient;
use sea_orm::QueryFilter;
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, Set,
};
use serde::Deserialize;

pub async fn find_ingredient_by_id(
    ingredient_id: i32,
    db: &DatabaseConnection,
) -> Result<Ingredient, AppError> {
    let ingredient = Ingredients::find_by_id(ingredient_id).one(db).await?;

    match ingredient {
        Some(ingredient) => Ok(ingredient),
        None => Err(AppError::new(
            StatusCode::NOT_FOUND,
            "Specified Ingredient Not Found",
            format!("Ingredient with Id: {} Not Found", ingredient_id),
        )),
    }
}

#[derive(Deserialize, Debug)]
pub struct CreateIngredient {
    pub name: String,
    pub quantity_in_stock: i32,
}

pub async fn create_and_save_ingredient(
    ingredient: CreateIngredient,
    db: &DatabaseConnection,
) -> Result<(), AppError> {
    let ingredient = ingredient::ActiveModel {
        id: NotSet,
        name: Set(ingredient.name),
        quantity_in_stock: Set(ingredient.quantity_in_stock),
    };

    ingredient.save(db).await?;
    Ok(())
}

#[derive(Deserialize, Debug)]
pub struct UpdateIngredient {
    pub name: Option<String>,
    pub quantity_in_stock: Option<i32>,
}

pub async fn update_and_save_ingredient(
    ingredient_id: i32,
    update_ingredient: UpdateIngredient,
    db: &DatabaseConnection,
) -> Result<(), AppError> {
    let mut db_ingredient = find_ingredient_by_id(ingredient_id, db)
        .await?
        .into_active_model();

    if let Some(name) = update_ingredient.name {
        db_ingredient.name = Set(name);
    }
    if let Some(quantity_in_stock) = update_ingredient.quantity_in_stock {
        db_ingredient.quantity_in_stock = Set(quantity_in_stock);
    }

    Ingredients::update(db_ingredient)
        .filter(ingredient::Column::Id.eq(ingredient_id))
        .exec(db)
        .await?;

    Ok(())
}
