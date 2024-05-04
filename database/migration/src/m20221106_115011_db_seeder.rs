use csv::ReaderBuilder;
use entity::watch;
use fake::faker::internet::en::Username;
use fake::faker::lorem::en::Sentences;
use fake::Fake;
use include_dir::include_dir;
use rand::Rng;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::prelude::Decimal;
use sea_orm_migration::sea_orm::{ActiveModelTrait, EntityTrait, Set};

#[derive(DeriveMigrationName)]
pub struct Migration {
    pub watch_csv: Option<String>,
}

impl Migration {
    pub fn init_csv(&mut self) {
        self.watch_csv = Some(
            include_dir!("$CARGO_MANIFEST_DIR/data")
                .get_file("watches.csv")
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
        let watch_csv = self.watch_csv.clone().unwrap();
        let mut reader = ReaderBuilder::new().from_reader(watch_csv.as_bytes());

        for result in reader.deserialize::<watch::Model>() {
            let record = result.map_err(|err| DbErr::Custom(err.to_string()))?;
            let active_record: watch::ActiveModel = record.into();
            let watch = active_record.insert(db).await?;

            let mut ratings_sum = 0;
            let mut ratings_count = 0;

            for _ in 0..rng.gen_range(0..5) {
                let review_text: Vec<String> = Sentences(1..10).fake_with_rng(&mut rng);
                let review = entity::review::ActiveModel {
                    reviewer_name: Set(Username().fake_with_rng(&mut rng)),
                    rating: Set(rng.gen_range(0..5)),
                    review_text: Set(review_text.join(" ")),
                    watch_id: Set(watch.id as i32),
                    ..Default::default()
                }
                .insert(db)
                .await?;

                ratings_sum += review.rating;
                ratings_count += 1;
            }

            let average_rating =
                Decimal::from_f32_retain((ratings_sum as f32) / ratings_count as f32)
                    .unwrap_or(Decimal::from(0));

            let mut active_watch = watch::ActiveModel::from(watch);
            active_watch.average_rating = Set(average_rating);

            active_watch.update(db).await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        entity::review::Entity::delete_many().exec(db).await?;
        entity::watch::Entity::delete_many().exec(db).await?;

        Ok(())
    }
}
