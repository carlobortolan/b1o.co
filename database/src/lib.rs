use ::entity::player;
pub use migration;
pub use migration::sea_orm_migration::MigratorTrait;
use migration::DbErr;
use sea_orm::{sea_query, ConnectionTrait, FromQueryResult, QueryOrder, QuerySelect, QueryTrait};
use sea_orm::{ActiveModelTrait, ActiveValue, QueryFilter};
use sea_orm::{ColumnTrait, PaginatorTrait};
use sea_orm::{DatabaseConnection, Order};
use sea_orm::TransactionTrait;
use sea_orm::{DbConn, EntityTrait};
use sea_orm::TryIntoModel;
use sea_query::Func;

pub struct PlayerQueries;

impl PlayerQueries {
    pub async fn find_all(
        db: &DbConn,
        page: u32,
        per_page: u32,
    ) -> Result<Vec<player::Model>, DbErr> {
        let paginator = player::Entity::find()
            .order_by(player::Column::Rating, sea_orm::Order::Desc)
            .paginate(db, per_page.into());

        let players = paginator.fetch_page((page - 1).into()).await?;
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

        // TODO: AI pre-calculation
        // calc_avg_rating(&txn, id, *new_player.avg_rating.as_ref()).await?;
        // calc_rating(&txn, id, *new_player.rating.as_ref()).await?;

        txn.commit().await?;

        Ok(new_player.try_into_model()?)
    }

    pub async fn update_player(
        db: &DbConn,
        player: entity::player::Model,
    ) -> Result<entity::player::ActiveModel, DbErr> {
        let active_player = entity::player::ActiveModel {
            id: ActiveValue::Set(player.id),
            name: ActiveValue::Set(player.name),
            image_url: ActiveValue::Set(player.image_url),
            upvotes: ActiveValue::Set(player.upvotes),
            downvotes: ActiveValue::Set(player.downvotes),
            source: ActiveValue::Set(player.source),
            date: ActiveValue::Set(player.date),
            average_rating: ActiveValue::Set(player.average_rating),
            rating: ActiveValue::Set(player.rating),
        };

        let updated_player = active_player.save(db).await?;

        Ok(updated_player)
    }

    pub async fn find_random_pair(
        db: &DatabaseConnection,
    ) -> Result<(entity::player::Model, entity::player::Model), DbErr> {
        let select = entity::player::Entity::find()
            .as_query()
            .to_owned()
            .order_by_expr(Func::random(), Order::Asc)
            .limit(2)
            .to_owned();
        let statement = db.get_database_backend().build(&select);

        let result = entity::player::Model::find_by_statement(statement)
            .all(db)
            .await?;

        if result.len() == 2 {
            Ok((result[0].clone(), result[1].clone()))
        } else {
            Err(DbErr::RecordNotFound(
                "Not enough players in the database".to_string(),
            ))
        }
    }

    pub async fn find_next(
        db: &DbConn,
        player: player::ActiveModel,
        visited_ids: Vec<i32>,
    ) -> Result<Option<player::Model>, DbErr> {
        let rating = match player.rating {
            ActiveValue::Set(val) => val,
            _ => 1400f32,
        };

        // TODO: Adapt proximity to nearest not visited entry
        let mut query = player::Entity::find()
            .filter(player::Column::Rating.between(rating - 100f32, rating + 100f32))
            .order_by(player::Column::Rating, Order::Asc)
            .limit(1);

        for id in visited_ids {
            query = query.filter(player::Column::Id.ne(id));
        }

        let similar_players = query.one(db).await?;

        Ok(similar_players)
    }
}

pub use sea_orm;
