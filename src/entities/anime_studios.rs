use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "anime_studios")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub anime_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub studio_id: i64,

    #[sea_orm(belongs_to, from = "anime_id", to = "id")]
    pub anime: BelongsTo<super::animes::Entity>,
    #[sea_orm(belongs_to, from = "studio_id", to = "id")]
    pub studio: BelongsTo<super::studios::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
