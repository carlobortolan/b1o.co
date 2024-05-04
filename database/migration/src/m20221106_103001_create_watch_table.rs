use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Watch::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Watch::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Watch::Name).string().not_null())
                    .col(ColumnDef::new(Watch::Description).string().not_null())
                    .col(ColumnDef::new(Watch::Reference).string().not_null())
                    .col(ColumnDef::new(Watch::Movement).string().not_null())
                    .col(ColumnDef::new(Watch::Manufacturer).string().not_null())
                    .col(
                        ColumnDef::new(Watch::ManufacturerLocation)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Watch::AverageRating)
                            .decimal_len(3, 2)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Watch::ImageUrl).string().not_null())
                    .col(ColumnDef::new(Watch::Style).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Watch::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Watch {
    Table,
    Id,
    Name,
    Reference,
    Movement,
    Manufacturer,
    ManufacturerLocation,
    AverageRating,
    Description,
    ImageUrl,
    Style,
}
