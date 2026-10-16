use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utility_types::Pick;

#[derive(Pick)]
#[pick(arg(ident = CreateStudio, fields(name), forward_attrs()))]
#[pick(arg(ident = UpdateStudio, fields(id, name), forward_attrs()))]
#[sea_orm::model]
#[derive(DeriveEntityModel, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[sea_orm(table_name = "studios")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub name: String,

    #[sea_orm(has_many, via = "anime_studios")]
    pub animes: HasMany<super::animes::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
