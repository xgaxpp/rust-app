//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "sys_position")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub name: Option<String>,
    pub code: Option<String>,
    pub sort: Option<i16>,
    pub status: Option<String>,
    pub remark: Option<String>,
    pub created_at: Option<DateTime>,
    pub create_by: Option<String>,
    pub updated_at: Option<DateTime>,
    pub update_by: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}