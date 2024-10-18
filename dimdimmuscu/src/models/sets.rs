use sea_orm::entity::prelude::*;
use super::_entities::sets::{ActiveModel, Entity};
pub type Sets = Entity;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}
