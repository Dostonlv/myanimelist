use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "animes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub title: String,
    pub title_english: String,
    pub title_japanese: String,
    pub r#type: String,
    pub synopsis: String,
    pub episodes: i32,
    pub status: String,
    pub aired_from: chrono::NaiveDate,
    pub aired_to: chrono::NaiveDate,
    pub season: String,
    pub year: i32,
    pub source: String,
    pub duration_min: i32,
    pub rating: f32,
    pub image_url: String,

    #[sea_orm(default = "chrono::Utc::now()")]
    pub created_at: chrono::NaiveDateTime,
}

impl ActiveModelBehavior for ActiveModel {}
