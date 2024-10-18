use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(Lifts::Table)
                    .col(pk_auto(Lifts::Id))
                    .col(integer(Lifts::SessionId))
                    .col(integer(Lifts::MovementId))
                    .col(integer(Lifts::LiftNumber))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-lifts-sessions")
                            .from(Lifts::Table, Lifts::SessionId)
                            .to(Sessions::Table, Sessions::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-lifts-movements")
                            .from(Lifts::Table, Lifts::MovementId)
                            .to(Movements::Table, Movements::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Lifts::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Lifts {
    Table,
    Id,
    SessionId,
    MovementId,
    LiftNumber,
    
}


#[derive(DeriveIden)]
enum Sessions {
    Table,
    Id,
}
#[derive(DeriveIden)]
enum Movements {
    Table,
    Id,
}
