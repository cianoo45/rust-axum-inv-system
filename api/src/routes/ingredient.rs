use crate::errors::AppError;
use axum::http::StatusCode;
use axum::{extract::Path, Extension, Json};
use entity::ingredient;
use entity::ingredient::Entity as Ingredients;
use entity::ingredient::Model as Ingredient;
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, DatabaseConnection, EntityTrait, Set,
};
use sea_orm::{IntoActiveModel, QueryFilter};
use serde::Deserialize;

pub async fn get_ingredient(
    Path(ingredient_id): Path<i32>,
    Extension(db): Extension<DatabaseConnection>,
) -> Result<Json<Ingredient>, AppError> {
    let ingredient = Ingredients::find_by_id(ingredient_id).one(&db).await?;

    match ingredient {
        Some(ingredient) => Ok(Json(ingredient)),
        None => Err(AppError::new(
            StatusCode::NOT_FOUND,
            "Specified Ingredient Not Found",
            format!("Ingredient with Id: {} Not Found", ingredient_id),
        )),
    }
}

#[derive(Deserialize, Debug)]
pub struct CreateIngredient {
    name: String,
    quantity_in_stock: i32,
}

pub async fn create_ingredient(
    Extension(database): Extension<DatabaseConnection>,
    Json(data): Json<CreateIngredient>,
) -> Result<(), AppError> {
    let ingredient = ingredient::ActiveModel {
        id: NotSet,
        name: Set(data.name),
        quantity_in_stock: Set(data.quantity_in_stock),
    };

    ingredient.save(&database).await?;
    Ok(())
}

#[derive(Deserialize, Debug)]
pub struct UpdateIngredient {
    name: Option<String>,
    quantity_in_stock: Option<i32>,
}

pub async fn update_ingredient(
    Extension(database): Extension<DatabaseConnection>,
    Path(ingredient_id): Path<i32>,
    Json(data): Json<UpdateIngredient>,
) -> Result<(), AppError> {
    let mut db_ingredient = if let Some(ingredient) = Ingredients::find_by_id(ingredient_id)
        .one(&database)
        .await?
    {
        ingredient.into_active_model()
    } else {
        return Err(AppError::new(
            StatusCode::NOT_FOUND,
            "Specified Ingredient Not Found",
            format!("Ingredient with Id: {} Not Found", ingredient_id),
        ));
    };

    if let Some(name) = data.name {
        db_ingredient.name = Set(name);
    }
    if let Some(quantity_in_stock) = data.quantity_in_stock {
        db_ingredient.quantity_in_stock = Set(quantity_in_stock);
    }

    Ingredients::update(db_ingredient)
        .filter(ingredient::Column::Id.eq(ingredient_id))
        .exec(&database)
        .await?;

    Ok(())
}

pub async fn delete_ingredient(
    Path(ingredient_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
) -> Result<(), AppError> {
    Ingredients::delete_by_id(ingredient_id)
        .exec(&database)
        .await?;

    Ok(())
}
