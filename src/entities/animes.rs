use sea_orm::entity::prelude::*;
use utility_types::{Omit, Pick};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(model_attrs(
    derive(Omit, Pick),
    omit(arg(
        ident = CreateAnime,
        fields(id, created_at),
        derive(Debug, Clone, sea_orm::DeriveIntoActiveModel),
        forward_attrs()
    )),
    omit(arg(
        ident = UpdateAnime,
        fields(created_at),
        derive(Debug, Clone, sea_orm::DeriveIntoActiveModel),
        forward_attrs()
    ))
))]
#[sea_orm(table_name = "anime")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub title: String,
    pub title_english: Option<String>,
    pub title_japanese: Option<String>,
    pub r#type: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub synopsis: Option<String>,
    pub episodes: Option<i32>,
    pub status: Option<String>,
    pub aired_from: Option<chrono::NaiveDate>,
    pub aired_to: Option<chrono::NaiveDate>,
    pub season: Option<String>,
    pub year: Option<i32>,
    pub source: Option<String>,
    pub duration_min: Option<i32>,
    pub rating: Option<String>,
    pub image_url: Option<String>,

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: chrono::NaiveDateTime,

    #[sea_orm(has_many)]
    pub anime_list: HasMany<super::anime_list::Entity>,
    #[sea_orm(has_many)]
    pub reviews: HasMany<super::reviews::Entity>,
    #[sea_orm(has_many)]
    pub favorites: HasMany<super::favorites::Entity>,
    #[sea_orm(has_many, via = "anime_genres")]
    pub genres: HasMany<super::genres::Entity>,
    #[sea_orm(has_many, via = "anime_studios")]
    pub studios: HasMany<super::studios::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
