use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "anime_list")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,

    pub user_id: i64,

    pub anime_id: i64,

    pub status: String,

    pub score: i64,

    pub episodes_watched: i64,

    pub start_date: chrono::NaiveDate,

    pub finish_date: chrono::NaiveDate,

    pub rewatching: bool,

    pub tags: String,

    pub comments: String,

    pub updated_at: chrono::NaiveDateTime,
}

impl ActiveModelBehavior for ActiveModel {}
