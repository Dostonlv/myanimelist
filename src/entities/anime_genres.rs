use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "genres")]
pub struct Model {
    #[sea_orm(primary_key, has_one)]
    pub id: i64,
    #[sea_orm(unique)]
    pub name: String,
}

impl ActiveModelBehavior for ActiveModel {}
