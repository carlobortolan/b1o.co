use ::entity::{player, review, watch};
use entity::watch::Relation;
pub use migration;
pub use migration::sea_orm_migration::MigratorTrait;
use migration::DbErr;
use sea_orm::prelude::Decimal;
use sea_orm::QueryOrder;
use sea_orm::{ActiveModelTrait, ActiveValue, QueryFilter};
use sea_orm::{ColumnTrait, PaginatorTrait};
use sea_orm::{DatabaseTransaction, TransactionTrait};
use sea_orm::{DbConn, EntityTrait, ModelTrait};
use sea_orm::{Set, TryIntoModel};

pub struct PlayerQueries;

pub struct WatchQueries;

impl PlayerQueries {
    pub async fn find_all(
        db: &DbConn,
        page: u32,
        per_page: u32,
    ) -> Result<Vec<player::Model>, DbErr> {
        let paginator = player::Entity::find()
            .order_by(player::Column::Rating, sea_orm::Order::Desc)
            .paginate(db, per_page.into());

        let players = paginator.fetch_page(page.into()).await?;

        Ok(players)
    }

    pub async fn find_one(db: &DbConn, id: i32) -> Result<Option<player::Model>, DbErr> {
        player::Entity::find_by_id(id).one(db).await
    }

    pub async fn create_player(db: &DbConn, player: player::Model) -> Result<player::Model, DbErr> {
        let txn = db.begin().await?;

        let mut new_player = player::ActiveModel::from(player);
        new_player.id = ActiveValue::NotSet;
        new_player.date = ActiveValue::NotSet;
        let new_player = new_player.save(&txn).await?;

        // AI precalculation
        // calc_avg_rating(&txn, id, *new_player.avg_rating.as_ref()).await?;
        // calc_rating(&txn, id, *new_player.rating.as_ref()).await?;

        txn.commit().await?;

        Ok(new_player.try_into_model()?)
    }

    pub async fn update_player(db: &DbConn, player: player::Model) -> Result<player::Model, DbErr> {
        let txn = db.begin().await?;

        let active_player = player::ActiveModel::from(player);
        let updated_player = active_player.save(&txn).await?;

        txn.commit().await?;

        Ok(updated_player.try_into_model()?)
    }

    /*   pub async fn find_next(
        db: &DbConn,
        player: player::Model,
        visited_ids: Vec<i32>,
    ) -> Result<Option<player::Model>, DbErr> {
        let visited_ids = visited_ids
            .into_iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");
        let raw_query = format!(
            "SELECT * FROM player WHERE id != {} AND id NOT IN ({}) ORDER BY ABS(rating - {}) LIMIT 1",
            player.id, visited_ids, player.rating
        );

        let next_player: Option<player::Model> =
            sqlx::query_as(&raw_query).fetch_optional(db).await?;

        Ok(next_player)
    } */
}

impl WatchQueries {
    pub async fn find_all(db: &DbConn) -> Result<Vec<watch::Model>, DbErr> {
        watch::Entity::find().all(db).await
    }

    pub async fn find_one(db: &DbConn, id: i32) -> Result<Option<watch::Model>, DbErr> {
        watch::Entity::find_by_id(id).one(db).await
    }

    pub async fn find_with_related(
        db: &DbConn,
        id: i32,
        relation: Relation,
    ) -> Result<(Option<watch::Model>, Vec<review::Model>), sea_orm::DbErr> {
        let watch = watch::Entity::find_by_id(id).one(db).await?;

        let reviews = if let Some(watch) = &watch {
            match relation {
                Relation::Review => watch.find_related(review::Entity).all(db).await?,
            }
        } else {
            vec![]
        };

        Ok((watch, reviews))
    }

    pub async fn add_review(
        db: &DbConn,
        id: i32,
        mut review: review::Model,
    ) -> Result<review::Model, DbErr> {
        let txn = db.begin().await?;

        review.watch_id = id;
        let mut new_review = review::ActiveModel::from(review);
        new_review.id = ActiveValue::NotSet;
        new_review.date = ActiveValue::NotSet;
        let new_review = new_review.save(&txn).await?;

        calc_avg_review(&txn, id, *new_review.rating.as_ref()).await?;

        txn.commit().await?;

        Ok(new_review.try_into_model()?)
    }
}

async fn calc_avg_review(
    db: &DatabaseTransaction,
    watch_id: i32,
    new_rating: i32,
) -> Result<(), sea_orm::DbErr> {
    let review_count: Decimal = review::Entity::find()
        .filter(review::Column::WatchId.eq(watch_id))
        .count(db)
        .await?
        .into();

    let reviewed_watch = watch::Entity::find_by_id(watch_id)
        .one(db)
        .await?
        .expect("the reviewed watch to exist");

    let current_average_rating = reviewed_watch.average_rating;
    let old_total = current_average_rating * (review_count - Decimal::from(1));

    let new_total = old_total + Decimal::from(new_rating);

    let new_average = new_total
        .checked_div(review_count)
        .unwrap_or(Decimal::from(0));

    let mut reviewed_watch = watch::ActiveModel::from(reviewed_watch);
    reviewed_watch.average_rating = Set(new_average);

    reviewed_watch.save(db).await?;

    Ok(())
}

pub use sea_orm;
