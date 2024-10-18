use sea_orm::entity::prelude::*;
use super::_entities::sessions::{ActiveModel, Entity};
pub type Sessions = Entity;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}
