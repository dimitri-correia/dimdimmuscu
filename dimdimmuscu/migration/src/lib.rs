#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_users;
mod m20241018_223143_update_users;
mod m20241018_225603_muscles;
mod m20241018_230450_movements;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20241018_223143_update_users::Migration),
            Box::new(m20241018_225603_muscles::Migration),
            Box::new(m20241018_230450_movements::Migration),
        ]
    }
}