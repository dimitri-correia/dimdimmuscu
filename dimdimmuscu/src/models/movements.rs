use sea_orm::entity::prelude::*;
use super::_entities::movements::{ActiveModel, Entity};
pub type Movements = Entity;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}
