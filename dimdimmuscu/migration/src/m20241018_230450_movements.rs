use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(Movements::Table)
                    .col(pk_auto(Movements::Id))
                    .col(string(Movements::Name))
                    .col(integer(Movements::MuscleId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-movements-muscles")
                            .from(Movements::Table, Movements::MuscleId)
                            .to(Muscles::Table, Muscles::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Movements::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Movements {
    Table,
    Id,
    Name,
    MuscleId,
    
}


#[derive(DeriveIden)]
enum Muscles {
    Table,
    Id,
}
