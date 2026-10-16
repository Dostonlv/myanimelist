use sea_orm::entity::prelude::*;
use utility_types::{Omit, Pick};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(model_attrs(
    derive(Omit, Pick),
    omit(arg(
        ident = CreateUser,
        fields(id, created_at),
        derive(Debug, Clone, sea_orm::DeriveIntoActiveModel),
        forward_attrs()
    )),
    pick(arg(
        ident = UpdateUser,
        fields(id, username, avatar_url, about),
        derive(Debug, Clone, sea_orm::DeriveIntoActiveModel),
        forward_attrs()
    )),
    pick(arg(
        ident = UpdatePassword,
        fields(id, password),
        derive(Debug, Clone, sea_orm::DeriveIntoActiveModel),
        forward_attrs()
    ))
))]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub username: String,
    #[sea_orm(unique)]
    pub email: String,
    pub password: String,
    pub avatar_url: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub about: Option<String>,

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: chrono::NaiveDateTime,

    #[sea_orm(has_many)]
    pub anime_list: HasMany<super::anime_list::Entity>,
    #[sea_orm(has_many)]
    pub reviews: HasMany<super::reviews::Entity>,
    #[sea_orm(has_many)]
    pub favorites: HasMany<super::favorites::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
