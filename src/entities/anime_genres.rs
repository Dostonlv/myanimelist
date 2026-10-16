use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "anime_genres")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub anime_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub genre_id: i64,

    #[sea_orm(belongs_to, from = "anime_id", to = "id")]
    pub anime: BelongsTo<super::animes::Entity>,
    #[sea_orm(belongs_to, from = "genre_id", to = "id")]
    pub genre: BelongsTo<super::genres::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
