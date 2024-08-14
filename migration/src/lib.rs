pub use sea_orm_migration::prelude::*;

mod m20240812_194930_create_ingredient_table;
mod m20240812_201251_create_meals_table;
mod m20240812_201341_create_ingredients_to_meals_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240812_194930_create_ingredient_table::Migration),
            Box::new(m20240812_201251_create_meals_table::Migration),
            Box::new(m20240812_201341_create_ingredients_to_meals_table::Migration),
        ]
    }
}
