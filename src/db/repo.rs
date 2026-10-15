use sea_orm::{DbErr};
use sea_orm_migration::async_trait::async_trait;


#[async_trait]
pub trait Repository {
    type Model;
    type CreateDto;
    type UpdateDto;
    type FilterDto;
    
    async fn create(&self,dto: &Self::CreateDto) -> Result<Self::Model, DbErr>;
    async fn get_by_id(&self, id: i64) -> Result<Self::Model, DbErr>;
    async fn get_all(&self, filter: &Self::FilterDto) -> Result<Vec<Self::Model>, DbErr>;
    async fn update(&self,dto: &Self::UpdateDto) -> Result<i64, DbErr>;
    async fn delete(&self, id: i64) -> Result<(), DbErr>;
}
