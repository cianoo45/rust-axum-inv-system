use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Meal::Table)
                    .if_not_exists()
                    .col(pk_auto(Meal::Id))
                    .col(string(Meal::Name))
                    .col(integer(Meal::Price))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Meal::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Meal {
    Table,
    Id,
    Name,
    Price,
}
