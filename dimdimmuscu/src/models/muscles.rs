use sea_orm::entity::prelude::*;
use super::_entities::muscles::{ActiveModel, Entity};
pub type Muscles = Entity;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}
