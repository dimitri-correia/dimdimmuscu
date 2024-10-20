//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "lifts")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i32,
    pub session_id: i32,
    pub movement_id: i32,
    pub lift_number: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::movements::Entity",
        from = "Column::MovementId",
        to = "super::movements::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Movements,
    #[sea_orm(
        belongs_to = "super::sessions::Entity",
        from = "Column::SessionId",
        to = "super::sessions::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Sessions,
    #[sea_orm(has_many = "super::sets::Entity")]
    Sets,
}

impl Related<super::movements::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Movements.def()
    }
}

impl Related<super::sessions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Sessions.def()
    }
}

impl Related<super::sets::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Sets.def()
    }
}
