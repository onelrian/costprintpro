use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(CreateUsersTable),
            Box::new(CreateCostParametersTable),
            Box::new(CreateJobsTable),
            Box::new(CreateBrandingSettingsTable),
        ]
    }
}

#[derive(DeriveMigrationName)]
pub struct CreateUsersTable;

#[async_trait::async_trait]
impl MigrationTrait for CreateUsersTable {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()".to_string()),
                    )
                    .col(ColumnDef::new(Users::Email).string().not_null().unique_key())
                    .col(ColumnDef::new(Users::PasswordHash).string().not_null())
                    .col(ColumnDef::new(Users::Role).string().not_null().default("user"))
                    .col(ColumnDef::new(Users::FirstName).string())
                    .col(ColumnDef::new(Users::LastName).string())
                    .col(ColumnDef::new(Users::IsActive).boolean().not_null().default(true))
                    .col(
                        ColumnDef::new(Users::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Users::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveMigrationName)]
pub struct CreateCostParametersTable;

#[async_trait::async_trait]
impl MigrationTrait for CreateCostParametersTable {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CostParameters::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CostParameters::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()".to_string()),
                    )
                    .col(ColumnDef::new(CostParameters::PaperCostPerSheet).decimal().not_null().default(0.10))
                    .col(ColumnDef::new(CostParameters::PlateCostPerJob).decimal().not_null().default(25.00))
                    .col(ColumnDef::new(CostParameters::LaborCostPerHour).decimal().not_null().default(50.00))
                    .col(ColumnDef::new(CostParameters::BindingCostPerUnit).decimal().not_null().default(0.50))
                    .col(ColumnDef::new(CostParameters::OverheadPercentage).decimal().not_null().default(0.15))
                    .col(ColumnDef::new(CostParameters::ProfitMarginPercentage).decimal().not_null().default(0.20))
                    .col(
                        ColumnDef::new(CostParameters::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(CostParameters::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CostParameters::Table).to_owned())
            .await
    }
}

#[derive(DeriveMigrationName)]
pub struct CreateJobsTable;

#[async_trait::async_trait]
impl MigrationTrait for CreateJobsTable {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Jobs::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Jobs::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()".to_string()),
                    )
                    .col(ColumnDef::new(Jobs::UserId).uuid().not_null())
                    .col(ColumnDef::new(Jobs::Title).string().not_null())
                    .col(ColumnDef::new(Jobs::JobType).string().not_null())
                    .col(ColumnDef::new(Jobs::Quantity).integer().not_null())
                    .col(ColumnDef::new(Jobs::TotalCost).decimal().not_null())
                    .col(ColumnDef::new(Jobs::UnitCost).decimal().not_null())
                    .col(ColumnDef::new(Jobs::Status).string().not_null().default("draft"))
                    .col(
                        ColumnDef::new(Jobs::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Jobs::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Jobs::Table).to_owned())
            .await
    }
}

#[derive(DeriveMigrationName)]
pub struct CreateBrandingSettingsTable;

#[async_trait::async_trait]
impl MigrationTrait for CreateBrandingSettingsTable {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BrandingSettings::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(BrandingSettings::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()".to_string()),
                    )
                    .col(ColumnDef::new(BrandingSettings::CompanyName).string().not_null())
                    .col(ColumnDef::new(BrandingSettings::PrimaryColor).string().not_null().default("#3B82F6"))
                    .col(ColumnDef::new(BrandingSettings::SecondaryColor).string().not_null().default("#1F2937"))
                    .col(
                        ColumnDef::new(BrandingSettings::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(BrandingSettings::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BrandingSettings::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
    Email,
    PasswordHash,
    Role,
    FirstName,
    LastName,
    IsActive,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum CostParameters {
    Table,
    Id,
    PaperCostPerSheet,
    PlateCostPerJob,
    LaborCostPerHour,
    BindingCostPerUnit,
    OverheadPercentage,
    ProfitMarginPercentage,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Jobs {
    Table,
    Id,
    UserId,
    Title,
    JobType,
    Quantity,
    TotalCost,
    UnitCost,
    Status,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum BrandingSettings {
    Table,
    Id,
    CompanyName,
    PrimaryColor,
    SecondaryColor,
    CreatedAt,
    UpdatedAt,
}
