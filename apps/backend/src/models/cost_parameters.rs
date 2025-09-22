use chrono::{DateTime, Utc};
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostParameters {
    pub id: Uuid,
    #[serde(rename = "paperCostPerSheet")]
    pub paper_cost_per_sheet: BigDecimal,
    #[serde(rename = "plateCostPerJob")]
    pub plate_cost_per_job: BigDecimal,
    #[serde(rename = "laborCostPerHour")]
    pub labor_cost_per_hour: BigDecimal,
    #[serde(rename = "bindingCostPerUnit")]
    pub binding_cost_per_unit: BigDecimal,
    #[serde(rename = "overheadPercentage")]
    pub overhead_percentage: BigDecimal,
    #[serde(rename = "profitMarginPercentage")]
    pub profit_margin_percentage: BigDecimal,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCostParametersRequest {
    #[serde(rename = "paperCostPerSheet")]
    pub paper_cost_per_sheet: Option<BigDecimal>,
    #[serde(rename = "plateCostPerJob")]
    pub plate_cost_per_job: Option<BigDecimal>,
    #[serde(rename = "laborCostPerHour")]
    pub labor_cost_per_hour: Option<BigDecimal>,
    #[serde(rename = "bindingCostPerUnit")]
    pub binding_cost_per_unit: Option<BigDecimal>,
    #[serde(rename = "overheadPercentage")]
    pub overhead_percentage: Option<BigDecimal>,
    #[serde(rename = "profitMarginPercentage")]
    pub profit_margin_percentage: Option<BigDecimal>,
}

impl Default for CostParameters {
    fn default() -> Self {
        use std::str::FromStr;
        
        CostParameters {
            id: Uuid::new_v4(),
            paper_cost_per_sheet: BigDecimal::from_str("0.10").unwrap(),
            plate_cost_per_job: BigDecimal::from_str("25.00").unwrap(),
            labor_cost_per_hour: BigDecimal::from_str("15.00").unwrap(),
            binding_cost_per_unit: BigDecimal::from_str("0.50").unwrap(),
            overhead_percentage: BigDecimal::from_str("0.15").unwrap(),
            profit_margin_percentage: BigDecimal::from_str("0.20").unwrap(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
