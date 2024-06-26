//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "sys_user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub username: String,
    pub password: String,
    pub real_name: Option<String>,
    pub nickname: Option<String>,
    pub gender: Option<String>,
    pub mobile: Option<String>,
    pub user_type: Option<String>,
    pub email: Option<String>,
    pub avatar: Option<String>,
    pub status: Option<String>,
    pub del_flag: Option<String>,
    pub login_ip: Option<String>,
    pub login_date: Option<DateTime>,
    pub remark: Option<String>,
    pub created_at: i64,
    pub create_by: Option<String>,
    pub updated_at: i64,
    pub update_by: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
