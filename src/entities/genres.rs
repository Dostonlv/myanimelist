use sea_orm::entity::prelude::*;
use utility_types::{Omit, Pick};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(model_attrs(
    derive(Omit, Pick),
    omit(arg(
        ident = CreateGenre,
        fields(id),
        derive(Debug, Clone, sea_orm::DeriveIntoActiveModel),
        forward_attrs()
    )),
    pick(arg(
        ident = UpdateGenre,
        fields(id, name),
        derive(Debug, Clone, sea_orm::DeriveIntoActiveModel),
        forward_attrs()
    ))
))]
#[sea_orm(table_name = "genres")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub name: String,

    #[sea_orm(has_many, via = "anime_genres")]
    pub animes: HasMany<super::animes::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
