use std::marker::PhantomData;

use sea_orm::{
    ActiveModelTrait, DbConn, DbErr, EntityTrait, IntoActiveModel, sea_query::IntoCondition,
};
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

// all repo has database connection and maybe has other field each other so for this i use phantom data

pub struct Repo<E: EntityTrait> {
    db: DbConn,
    _entity: PhantomData<E>,
}

impl<E: EntityTrait> Repo<E> {
    pub fn new(db: DbConn) -> Self {
        Self {
            db,
            _entity: PhantomData,
        }
    }
}

// // implement repository for repo struct
// // #[async_trait]
// impl<E> Repository for Repo<E>  where  {
//     type Model;

//     type ActiveModel;

//     type Id;

//     #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
//     fn insert<'life0, 'async_trait>(&'life0 self, dto: impl 'async_trait+IntoActiveModel<Self::ActiveModel> +Send,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Self::Model, DbErr> > + ::core::marker::Send+'async_trait> >where 'life0: 'async_trait, Self: 'async_trait {
//         todo!()
//     }

//     #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
//     fn update<'life0, 'async_trait>(&'life0 self, dto: impl 'async_trait+IntoActiveModel<Self::ActiveModel> +Send,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Self::Model, DbErr> > + ::core::marker::Send+'async_trait> >where 'life0: 'async_trait, Self: 'async_trait {
//         todo!()
//     }

//     #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
//     fn get_by_id<'life0, 'async_trait>(&'life0 self, id: Self::Id) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Self::Model, DbErr> > + ::core::marker::Send+'async_trait> >where 'life0: 'async_trait, Self: 'async_trait {
//         todo!()
//     }

//     #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
//     fn find<'life0, 'async_trait>(&'life0 self, cond: impl 'async_trait+IntoCondition+Send) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Vec<Self::Model> , DbErr> > + ::core::marker::Send+'async_trait> >where 'life0: 'async_trait, Self: 'async_trait {
//         todo!()
//     }

//     #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
//     fn delete_by_id<'life0, 'async_trait>(&'life0 self, id: Self::Id) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<(), DbErr> > + ::core::marker::Send+'async_trait> >where 'life0: 'async_trait, Self: 'async_trait {
//         todo!()
//     }
// }
