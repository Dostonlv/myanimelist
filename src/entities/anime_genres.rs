use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "anime_genres")]
pub struct Model {
    #[sea_orm(primary_key, has_one)]
    pub anime_id: i64,
    #[sea_orm(primary_key, has_one)]
    pub genre_id: i64,
}

impl ActiveModelBehavior for ActiveModel {}
