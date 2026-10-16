use sea_orm::entity::prelude::*;
use utility_types::{Omit, Pick};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(model_attrs(
    derive(Omit, Pick),
    omit(arg(
        ident = CreateReview,
        fields(id, created_at),
        derive(Debug, Clone, sea_orm::DeriveIntoActiveModel),
        forward_attrs()
    )),
    pick(arg(
        ident = UpdateReview,
        fields(id, score, body),
        derive(Debug, Clone, sea_orm::DeriveIntoActiveModel),
        forward_attrs()
    ))
))]
#[sea_orm(table_name = "reviews")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub user_id: i64,
    pub anime_id: i64,
    pub score: Option<i32>,
    #[sea_orm(column_type = "Text")]
    pub body: Option<String>,

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: chrono::NaiveDateTime,

    #[sea_orm(belongs_to, from = "user_id", to = "id")]
    pub user: BelongsTo<super::users::Entity>,
    #[sea_orm(belongs_to, from = "anime_id", to = "id")]
    pub anime: BelongsTo<super::animes::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}