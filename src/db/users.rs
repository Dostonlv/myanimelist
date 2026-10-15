use chrono::{Utc};
use sea_orm::{DbConn, EntityTrait};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DbErr};

use crate::db::repo::Repository;
use crate::entities::users::{self, Entity as UserEntity, Model};


pub struct CreateDto {
    username: String,
    email: String,
    password: String,
    avatar_url: String,
    about: String,
}

pub struct UpdateDto {
    id: i64,
    username: String,
    about: String,
    avatar_url: String
}

pub struct User {
  db: DbConn,
}

impl User {
}


