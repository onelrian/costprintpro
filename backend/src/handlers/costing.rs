use axum::{
    extract::State,
    response::Json,
};
use validator::Validate;

use crate::{
    models::{CostCalculationRequest, CostCalculationResponse, Currency},
    services::currency_service::CurrencyService,
    utils::errors::AppError,
    AppState,
};

pub async fn calculate_cost(
    Json(payload): Json<CostCalculationRequest>,
) -> Result<Json<CostCalculationResponse>, AppError> {
    use crate::models::CostBreakdown;
    use bigdecimal::BigDecimal;
    use std::str::FromStr;
    
    // Basic cost calculation based on job specifications
    let quantity = payload.quantity as f64;
    let pages = payload.specifications.pages.unwrap_or(1) as f64;
    
    // Base costs per unit
    let paper_cost_per_sheet = BigDecimal::from_str("0.10").unwrap();
    let plate_cost_base = BigDecimal::from_str("25.00").unwrap();
    let labor_cost_per_hour = BigDecimal::from_str("50.00").unwrap();
    let binding_cost_per_unit = BigDecimal::from_str("0.50").unwrap();
    
    // Calculate individual costs
    let paper_cost = &paper_cost_per_sheet * BigDecimal::from_str(&(quantity * pages).to_string()).unwrap();
    let plate_cost = &plate_cost_base * BigDecimal::from_str(&(payload.specifications.colors.front_colors + payload.specifications.colors.back_colors).to_string()).unwrap();
    let labor_hours = (quantity * pages / 1000.0).max(0.5); // Minimum 0.5 hours
    let labor_cost = &labor_cost_per_hour * BigDecimal::from_str(&labor_hours.to_string()).unwrap();
    let binding_cost = if payload.specifications.binding.is_some() && !payload.specifications.binding.as_ref().unwrap().is_empty() {
        &binding_cost_per_unit * BigDecimal::from_str(&quantity.to_string()).unwrap()
    } else {
        BigDecimal::from_str("0.00").unwrap()
    };
    let finishing_cost = BigDecimal::from_str(&(payload.specifications.finishing.len() as f64 * quantity * 0.25).to_string()).unwrap();
    
    // Calculate subtotal
    let subtotal = &paper_cost + &plate_cost + &labor_cost + &binding_cost + &finishing_cost;
    
    // Add overhead (15%)
    let overhead = &subtotal * BigDecimal::from_str("0.15").unwrap();
    let total_with_overhead = &subtotal + &overhead;
    
    // Add profit margin (20%)
    let profit = &total_with_overhead * BigDecimal::from_str("0.20").unwrap();
    let total_cost = &total_with_overhead + &profit;
    
    let unit_cost = &total_cost / BigDecimal::from_str(&quantity.to_string()).unwrap();
    
    let cost_breakdown = CostBreakdown {
        paper_cost,
        plate_cost,
        labor_cost,
        binding_cost,
        finishing_cost,
        overhead,
    };
    
    // Handle currency conversion if requested
    let (final_total, final_unit, final_breakdown, currency, exchange_rate) = if let Some(target_currency) = payload.currency {
        if target_currency.code() != "USD" {
            let currency_service = CurrencyService::new();
            
            // Convert all amounts
            let total_f64: f64 = total_cost.to_string().parse().unwrap_or(0.0);
            let unit_f64: f64 = unit_cost.to_string().parse().unwrap_or(0.0);
            
            let converted_total = currency_service.convert_amount(total_f64, Currency::USD, target_currency.clone()).await?;
            let converted_unit = currency_service.convert_amount(unit_f64, Currency::USD, target_currency.clone()).await?;
            
            // Convert cost breakdown
            let paper_f64: f64 = cost_breakdown.paper_cost.to_string().parse().unwrap_or(0.0);
            let plate_f64: f64 = cost_breakdown.plate_cost.to_string().parse().unwrap_or(0.0);
            let labor_f64: f64 = cost_breakdown.labor_cost.to_string().parse().unwrap_or(0.0);
            let binding_f64: f64 = cost_breakdown.binding_cost.to_string().parse().unwrap_or(0.0);
            let finishing_f64: f64 = cost_breakdown.finishing_cost.to_string().parse().unwrap_or(0.0);
            let overhead_f64: f64 = cost_breakdown.overhead.to_string().parse().unwrap_or(0.0);
            
            let converted_paper = currency_service.convert_amount(paper_f64, Currency::USD, target_currency.clone()).await?;
            let converted_plate = currency_service.convert_amount(plate_f64, Currency::USD, target_currency.clone()).await?;
            let converted_labor = currency_service.convert_amount(labor_f64, Currency::USD, target_currency.clone()).await?;
            let converted_binding = currency_service.convert_amount(binding_f64, Currency::USD, target_currency.clone()).await?;
            let converted_finishing = currency_service.convert_amount(finishing_f64, Currency::USD, target_currency.clone()).await?;
            let converted_overhead = currency_service.convert_amount(overhead_f64, Currency::USD, target_currency.clone()).await?;
            
            let converted_breakdown = CostBreakdown {
                paper_cost: BigDecimal::from_str(&converted_paper.to_string()).unwrap(),
                plate_cost: BigDecimal::from_str(&converted_plate.to_string()).unwrap(),
                labor_cost: BigDecimal::from_str(&converted_labor.to_string()).unwrap(),
                binding_cost: BigDecimal::from_str(&converted_binding.to_string()).unwrap(),
                finishing_cost: BigDecimal::from_str(&converted_finishing.to_string()).unwrap(),
                overhead: BigDecimal::from_str(&converted_overhead.to_string()).unwrap(),
            };
            
            // Get exchange rate
            let rates = currency_service.get_exchange_rates().await?;
            let rate = rates.rates.get(target_currency.code()).copied().unwrap_or(1.0);
            
            (
                BigDecimal::from_str(&converted_total.to_string()).unwrap(),
                BigDecimal::from_str(&converted_unit.to_string()).unwrap(),
                converted_breakdown,
                Some(target_currency),
                Some(rate)
            )
        } else {
            (total_cost, unit_cost, cost_breakdown, Some(Currency::USD), Some(1.0))
        }
    } else {
        (total_cost, unit_cost, cost_breakdown, None, None)
    };

    let response = CostCalculationResponse {
        total_cost: final_total,
        unit_cost: final_unit,
        cost_breakdown: final_breakdown,
        estimated_delivery_days: estimate_delivery_days(&payload.job_type, payload.quantity),
        currency,
        exchange_rate,
    };
    
    Ok(Json(response))
}

pub async fn preview_cost(
    Json(payload): Json<CostCalculationRequest>,
) -> Result<Json<CostCalculationResponse>, AppError> {
    // Same as calculate_cost for now
    calculate_cost(Json(payload)).await
}

pub async fn quick_calculate(
    Json(payload): Json<CostCalculationRequest>,
) -> Result<Json<CostCalculationResponse>, AppError> {
    // Quick calculation with simplified logic
    use crate::models::CostBreakdown;
    use bigdecimal::BigDecimal;
    use std::str::FromStr;
    
    let quantity = payload.quantity as f64;
    let base_cost = quantity * 0.5; // Simple base cost calculation
    
    let cost_breakdown = CostBreakdown {
        paper_cost: BigDecimal::from_str(&(base_cost * 0.4).to_string()).unwrap(),
        plate_cost: BigDecimal::from_str(&(base_cost * 0.2).to_string()).unwrap(),
        labor_cost: BigDecimal::from_str(&(base_cost * 0.2).to_string()).unwrap(),
        binding_cost: BigDecimal::from_str(&(base_cost * 0.1).to_string()).unwrap(),
        finishing_cost: BigDecimal::from_str(&(base_cost * 0.05).to_string()).unwrap(),
        overhead: BigDecimal::from_str(&(base_cost * 0.05).to_string()).unwrap(),
    };
    
    let total_cost = BigDecimal::from_str(&base_cost.to_string()).unwrap();
    let unit_cost = &total_cost / BigDecimal::from_str(&quantity.to_string()).unwrap();
    
    // Handle currency conversion for quick calculation too
    let (final_total, final_unit, final_breakdown, currency, exchange_rate) = if let Some(target_currency) = payload.currency {
        if target_currency.code() != "USD" {
            let currency_service = CurrencyService::new();
            let total_f64: f64 = total_cost.to_string().parse().unwrap_or(0.0);
            let unit_f64: f64 = unit_cost.to_string().parse().unwrap_or(0.0);
            
            let converted_total = currency_service.convert_amount(total_f64, Currency::USD, target_currency.clone()).await?;
            let converted_unit = currency_service.convert_amount(unit_f64, Currency::USD, target_currency.clone()).await?;
            
            // Convert cost breakdown components
            let paper_f64: f64 = cost_breakdown.paper_cost.to_string().parse().unwrap_or(0.0);
            let plate_f64: f64 = cost_breakdown.plate_cost.to_string().parse().unwrap_or(0.0);
            let labor_f64: f64 = cost_breakdown.labor_cost.to_string().parse().unwrap_or(0.0);
            let binding_f64: f64 = cost_breakdown.binding_cost.to_string().parse().unwrap_or(0.0);
            let finishing_f64: f64 = cost_breakdown.finishing_cost.to_string().parse().unwrap_or(0.0);
            let overhead_f64: f64 = cost_breakdown.overhead.to_string().parse().unwrap_or(0.0);
            
            let converted_breakdown = CostBreakdown {
                paper_cost: BigDecimal::from_str(&currency_service.convert_amount(paper_f64, Currency::USD, target_currency.clone()).await?.to_string()).unwrap(),
                plate_cost: BigDecimal::from_str(&currency_service.convert_amount(plate_f64, Currency::USD, target_currency.clone()).await?.to_string()).unwrap(),
                labor_cost: BigDecimal::from_str(&currency_service.convert_amount(labor_f64, Currency::USD, target_currency.clone()).await?.to_string()).unwrap(),
                binding_cost: BigDecimal::from_str(&currency_service.convert_amount(binding_f64, Currency::USD, target_currency.clone()).await?.to_string()).unwrap(),
                finishing_cost: BigDecimal::from_str(&currency_service.convert_amount(finishing_f64, Currency::USD, target_currency.clone()).await?.to_string()).unwrap(),
                overhead: BigDecimal::from_str(&currency_service.convert_amount(overhead_f64, Currency::USD, target_currency.clone()).await?.to_string()).unwrap(),
            };
            
            let rates = currency_service.get_exchange_rates().await?;
            let rate = rates.rates.get(target_currency.code()).copied().unwrap_or(1.0);
            
            (
                BigDecimal::from_str(&converted_total.to_string()).unwrap(),
                BigDecimal::from_str(&converted_unit.to_string()).unwrap(),
                converted_breakdown,
                Some(target_currency),
                Some(rate)
            )
        } else {
            (total_cost, unit_cost, cost_breakdown, Some(Currency::USD), Some(1.0))
        }
    } else {
        (total_cost, unit_cost, cost_breakdown, None, None)
    };

    let response = CostCalculationResponse {
        total_cost: final_total,
        unit_cost: final_unit,
        cost_breakdown: final_breakdown,
        estimated_delivery_days: 3,
        currency,
        exchange_rate,
    };
    
    Ok(Json(response))
}

fn estimate_delivery_days(job_type: &crate::models::JobType, quantity: i32) -> i32 {
    use crate::models::JobType;
    
    let base_days = match job_type {
        JobType::BusinessCard => 1,
        JobType::Flyer => 2,
        JobType::Brochure => 3,
        JobType::Book => 5,
        JobType::Poster => 2,
        JobType::Banner => 3,
        JobType::Sticker => 2,
        JobType::Custom => 5,
    };

    // Add extra days for large quantities
    let quantity_factor = match quantity {
        0..=100 => 0,
        101..=500 => 1,
        501..=1000 => 2,
        1001..=5000 => 3,
        _ => 5,
    };

    base_days + quantity_factor
}
