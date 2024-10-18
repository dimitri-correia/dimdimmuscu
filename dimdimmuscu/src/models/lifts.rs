use sea_orm::entity::prelude::*;
use super::_entities::lifts::{ActiveModel, Entity};
pub type Lifts = Entity;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}
