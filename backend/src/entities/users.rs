use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub is_active: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::jobs::Entity")]
    Jobs,
}

impl Related<super::jobs::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Jobs.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
