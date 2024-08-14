use sea_orm_migration::{prelude::*, schema::*};

use super::m20240812_194930_create_ingredient_table::Ingredient;
use super::m20240812_201251_create_meals_table::Meal;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(IngredientsToMeals::Table)
                    .if_not_exists()
                    .col(pk_auto(IngredientsToMeals::Id))
                    .col(integer(IngredientsToMeals::IngredientId))
                    .col(integer(IngredientsToMeals::MealId))
                    .col(integer(IngredientsToMeals::Quantity))
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_ingredients_to_meals_ingredient_id")
                            .from(IngredientsToMeals::Table, IngredientsToMeals::IngredientId)
                            .to(Ingredient::Table, Ingredient::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_ingredients_to_meals_meal_id")
                            .from(IngredientsToMeals::Table, IngredientsToMeals::MealId)
                            .to(Meal::Table, Meal::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(IngredientsToMeals::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum IngredientsToMeals {
    Table,
    Id,
    IngredientId,
    MealId,
    Quantity,
}
