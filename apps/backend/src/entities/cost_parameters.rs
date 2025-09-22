use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "cost_parameters")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub paper_cost_per_sheet: Decimal,
    pub plate_cost_per_job: Decimal,
    pub labor_cost_per_hour: Decimal,
    pub binding_cost_per_unit: Decimal,
    pub overhead_percentage: Decimal,
    pub profit_margin_percentage: Decimal,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
