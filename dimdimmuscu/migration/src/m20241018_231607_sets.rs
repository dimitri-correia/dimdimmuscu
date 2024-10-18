use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(Sets::Table)
                    .col(pk_auto(Sets::Id))
                    .col(integer(Sets::LiftId))
                    .col(integer(Sets::SetNumber))
                    .col(integer(Sets::Weight))
                    .col(integer(Sets::Rep))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-sets-lifts")
                            .from(Sets::Table, Sets::LiftId)
                            .to(Lifts::Table, Lifts::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Sets::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Sets {
    Table,
    Id,
    LiftId,
    SetNumber,
    Weight,
    Rep,
    
}


#[derive(DeriveIden)]
enum Lifts {
    Table,
    Id,
}
