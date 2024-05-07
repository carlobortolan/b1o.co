use csv::ReaderBuilder;
use entity::player;
use fake::faker::internet::en::Username;
use fake::Fake;
use include_dir::include_dir;
use rand::Rng;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::{ActiveModelTrait, EntityTrait, Set};

#[derive(DeriveMigrationName)]
pub struct Migration {
    pub player_csv: Option<String>,
}

impl Migration {
    pub fn init_csv(&mut self) {
        self.player_csv = Some(
            include_dir!("$CARGO_MANIFEST_DIR/data")
                .get_file("players.csv")
                .unwrap()
                .contents_utf8()
                .unwrap()
                .to_string(),
        );
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let mut rng = rand::rngs::OsRng;
        let watch_csv = self.player_csv.clone().unwrap();
        let mut reader = ReaderBuilder::new().from_reader(watch_csv.as_bytes());

        for result in reader.deserialize::<player::Model>() {
            let record = result.map_err(|err| DbErr::Custom(err.to_string()))?;
            let active_record: player::ActiveModel = record.into();
            let player = active_record.insert(db).await?;

            let average_rating = rng.gen_range(0f32..1000f32);
            let rating = rng.gen_range(800f32..2400f32);
            let mut active_player = player::ActiveModel::from(player);
            active_player.average_rating = Set(average_rating);
            active_player.rating = Set(rating);
            active_player.name = Set(Username().fake_with_rng(&mut rng));

            active_player.update(db).await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        entity::player::Entity::delete_many().exec(db).await?;

        Ok(())
    }
}
