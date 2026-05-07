use sea_orm::Iterable;
use sea_orm::EnumIter;
use sea_orm_migration::{prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .alter_table(Table::alter().table(Issue::Table).modify_column(ColumnDef::new(Issue::ToOffline)
                .enumeration(ToOffline::Table, ToOffline::iter().skip(1))).to_owned())
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        //not tested, probably doesn't actually do anything anyway
        manager
            .alter_table(Table::alter().table(Issue::Table).modify_column(ColumnDef::new(Issue::ToOffline)
                .enumeration(ToOffline::Table, ToOffline::iter().skip(1).filter(|x| ! x.to_string().contains("None")))).to_owned())
            .await
    }
}

#[allow(dead_code)]
#[derive(DeriveIden)]
enum Issue {
    Table,
    Id,
    Title,
    Description,
    Status,
    TargetId,
    ToOffline,
    AssignedTo,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}

//if null means don't enforce offlining
#[derive(Iden, EnumIter)]
enum ToOffline {
    Table,
    Node,
    Card,
    Blade,
    None,
}