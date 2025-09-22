use axum::{
    extract::State,
    response::Json,
    Extension,
};
use validator::Validate;

use crate::{
    models::{
        BrandingSettings, CostParameters, UpdateBrandingRequest, UpdateCostParametersRequest,
        UserInfo, UserRole,
    },
    services::{branding_service::BrandingService, cost_parameters_service::CostParametersService},
    utils::errors::AppError,
    AppState,
};

pub async fn get_cost_parameters() -> Result<Json<CostParameters>, AppError> {
    // Placeholder implementation
    use crate::models::CostParameters;
    use bigdecimal::BigDecimal;
    use std::str::FromStr;
    
    let parameters = CostParameters {
        id: uuid::Uuid::new_v4(),
        paper_cost_per_sheet: BigDecimal::from_str("0.10").unwrap(),
        plate_cost_per_job: BigDecimal::from_str("25.00").unwrap(),
        labor_cost_per_hour: BigDecimal::from_str("15.00").unwrap(),
        binding_cost_per_unit: BigDecimal::from_str("0.50").unwrap(),
        overhead_percentage: BigDecimal::from_str("0.15").unwrap(),
        profit_margin_percentage: BigDecimal::from_str("0.20").unwrap(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    Ok(Json(parameters))
}

pub async fn update_cost_parameters(
    Json(payload): Json<UpdateCostParametersRequest>,
) -> Result<Json<CostParameters>, AppError> {
    use crate::models::CostParameters;
    use bigdecimal::BigDecimal;
    use std::str::FromStr;
    
    // Return updated parameters (in a real app, this would save to database)
    let parameters = CostParameters {
        id: uuid::Uuid::new_v4(),
        paper_cost_per_sheet: payload.paper_cost_per_sheet.unwrap_or_else(|| BigDecimal::from_str("0.10").unwrap()),
        plate_cost_per_job: payload.plate_cost_per_job.unwrap_or_else(|| BigDecimal::from_str("25.00").unwrap()),
        labor_cost_per_hour: payload.labor_cost_per_hour.unwrap_or_else(|| BigDecimal::from_str("15.00").unwrap()),
        binding_cost_per_unit: payload.binding_cost_per_unit.unwrap_or_else(|| BigDecimal::from_str("0.50").unwrap()),
        overhead_percentage: payload.overhead_percentage.unwrap_or_else(|| BigDecimal::from_str("0.15").unwrap()),
        profit_margin_percentage: payload.profit_margin_percentage.unwrap_or_else(|| BigDecimal::from_str("0.20").unwrap()),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    Ok(Json(parameters))
}

pub async fn get_branding() -> Result<Json<BrandingSettings>, AppError> {
    // Placeholder implementation
    use crate::models::BrandingSettings;
    
    let branding = BrandingSettings {
        id: uuid::Uuid::new_v4(),
        company_name: "Demo Print Shop".to_string(),
        company_logo_url: None,
        primary_color: "#007bff".to_string(),
        secondary_color: "#6c757d".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    Ok(Json(branding))
}

pub async fn update_branding(
    Json(payload): Json<UpdateBrandingRequest>,
) -> Result<Json<BrandingSettings>, AppError> {
    use crate::models::BrandingSettings;
    
    // Return updated branding (in a real app, this would save to database)
    let branding = BrandingSettings {
        id: uuid::Uuid::new_v4(),
        company_name: payload.company_name.unwrap_or_else(|| "Demo Print Shop".to_string()),
        company_logo_url: payload.company_logo_url,
        primary_color: payload.primary_color.unwrap_or_else(|| "#007bff".to_string()),
        secondary_color: payload.secondary_color.unwrap_or_else(|| "#6c757d".to_string()),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    Ok(Json(branding))
}
