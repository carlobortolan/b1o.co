pub use sea_orm_migration::prelude::*;
mod m20240504_235549_db_seeder;
mod m20221106_115011_create_player_table;
pub use sea_orm_migration;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        let mut seeder = m20240504_235549_db_seeder::Migration { player_csv: None };
        seeder.init_csv();
        vec![
            Box::new(m20221106_115011_create_player_table::Migration),
            Box::new(seeder),
        ]
    }
}
