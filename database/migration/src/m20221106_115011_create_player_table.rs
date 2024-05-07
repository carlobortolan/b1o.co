use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Player::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Player::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Player::ImageUrl).string().not_null())
                    .col(ColumnDef::new(Player::Name).string().not_null())
                    .col(ColumnDef::new(Player::Upvotes).integer().not_null())
                    .col(ColumnDef::new(Player::Downvotes).integer().not_null())
                    .col(ColumnDef::new(Player::Source).string().not_null())
                    .col(
                        ColumnDef::new(Player::Date)
                            .timestamp()
                            .default("now()")
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Player::AverageRating)
                            .float()
                            .not_null()
                            .default(0f32),
                    )
                    .col(
                        ColumnDef::new(Player::Rating)
                            .float()
                            .not_null()
                            .default(1400f32),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Player::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Player {
    Table,
    Id,
    ImageUrl,
    Name,
    Upvotes,
    Downvotes,
    Source,
    Date,
    AverageRating,
    Rating,
}
