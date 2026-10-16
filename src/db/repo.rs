use sea_orm::{ActiveModelTrait, DbErr, IntoActiveModel, sea_query::IntoCondition};
use sea_orm_migration::async_trait::async_trait;

#[async_trait]
pub trait Repository {
    type Model;
    type ActiveModel: ActiveModelTrait;
    type Id;

    async fn insert(
        &self,
        dto: impl IntoActiveModel<Self::ActiveModel> + Send,
    ) -> Result<Self::Model, DbErr>;

    async fn update(
        &self,
        dto: impl IntoActiveModel<Self::ActiveModel> + Send,
    ) -> Result<Self::Model, DbErr>;

    async fn get_by_id(&self, id: Self::Id) -> Result<Self::Model, DbErr>;

    async fn find(&self, cond: impl IntoCondition + Send) -> Result<Vec<Self::Model>, DbErr>;

    async fn delete_by_id(&self, id: Self::Id) -> Result<(), DbErr>;
}
