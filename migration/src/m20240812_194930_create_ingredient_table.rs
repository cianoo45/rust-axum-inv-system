use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Ingredient::Table)
                    .if_not_exists()
                    .col(pk_auto(Ingredient::Id))
                    .col(string(Ingredient::Name))
                    .col(integer(Ingredient::QuantityInStock))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Ingredient::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Ingredient {
    Table,
    Id,
    Name,
    QuantityInStock,
}
