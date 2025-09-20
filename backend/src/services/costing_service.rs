use std::collections::HashMap;
use bigdecimal::BigDecimal;

use crate::{
    models::{
        CostParameters, JobSpecifications, JobType, CostBreakdown, Currency,
    },
    services::{cost_parameters_service::CostParametersService, currency_service::CurrencyService},
    utils::errors::AppError,
};

pub struct CostingService<'a> {
    db: &'a sea_orm::DatabaseConnection,
}

impl<'a> CostingService<'a> {
    pub fn new(db: &'a sea_orm::DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn calculate_cost(
        &self,
        job_type: &JobType,
        quantity: i32,
        specifications: &JobSpecifications,
    ) -> Result<CostCalculationResult, AppError> {
        let cost_params_service = CostParametersService::new(self.db);
        let cost_params = cost_params_service.get_current_parameters().await?;

        // Calculate paper cost
        let paper_cost = cost_params.paper_cost_per_sheet.clone() * BigDecimal::from(quantity);
        let plate_cost = cost_params.plate_cost_per_job.clone();
        let labor_cost = cost_params.labor_cost_per_hour.clone() * BigDecimal::from(2); // Assume 2 hours
        
        let binding_cost = cost_params.binding_cost_per_unit.clone() * BigDecimal::from(quantity);

        // Calculate finishing cost
        let finishing_cost = self.calculate_finishing_cost(quantity, specifications)?;

        // Calculate subtotal
        let subtotal = paper_cost.clone() + plate_cost.clone() + labor_cost.clone() + binding_cost.clone() + finishing_cost.clone();

        // Calculate overhead
        let overhead = subtotal.clone() * cost_params.overhead_percentage.clone();

        // Calculate total before margin
        let total_before_margin = subtotal + overhead.clone();

        // Calculate profit margin
        let profit_margin_amount = total_before_margin.clone() * cost_params.profit_margin_percentage.clone();

        // Calculate final cost
        let final_cost = total_before_margin + profit_margin_amount;

        // Calculate unit cost
        let unit_cost = final_cost.clone() / BigDecimal::from(quantity);

        // Create breakdown details (simplified for now)
        let _breakdown_details: HashMap<String, BigDecimal> = HashMap::new();

        let cost_breakdown = CostBreakdown {
            paper_cost,
            plate_cost,
            labor_cost,
            binding_cost,
            finishing_cost,
            overhead,
        };

        let cost_calculation_result = CostCalculationResult {
            cost_breakdown,
            total_cost: final_cost,
            unit_cost,
        };

        Ok(cost_calculation_result)
    }

    fn calculate_finishing_cost(
        &self,
        quantity: i32,
        specifications: &JobSpecifications,
    ) -> Result<BigDecimal, AppError> {
        use std::str::FromStr;
        let mut finishing_cost = BigDecimal::from_str("0.00").unwrap();

        // Lamination cost
        if let Some(_lamination) = &specifications.lamination {
            finishing_cost += BigDecimal::from(quantity) * BigDecimal::from_str("0.005").unwrap(); // $0.005 per unit
        }

        // Other finishing costs
        for _finishing in &specifications.finishing {
            finishing_cost += BigDecimal::from(quantity) * BigDecimal::from_str("0.002").unwrap(); // $0.002 per unit per finishing
        }

        Ok(finishing_cost)
    }

    pub async fn calculate_cost_with_currency(
        &self,
        job_type: &JobType,
        quantity: i32,
        specifications: &JobSpecifications,
        target_currency: Currency,
    ) -> Result<CostCalculationResultWithCurrency, AppError> {
        // First calculate in USD (base currency)
        let usd_result = self.calculate_cost(job_type, quantity, specifications).await?;
        
        if target_currency.code() == "USD" {
            return Ok(CostCalculationResultWithCurrency {
                cost_breakdown: usd_result.cost_breakdown,
                total_cost: usd_result.total_cost,
                unit_cost: usd_result.unit_cost,
                currency: Currency::USD,
                exchange_rate: 1.0,
            });
        }

        // Convert to target currency
        let currency_service = CurrencyService::new();
        
        // Convert each cost component
        let paper_cost_f64: f64 = usd_result.cost_breakdown.paper_cost.to_string().parse().unwrap_or(0.0);
        let plate_cost_f64: f64 = usd_result.cost_breakdown.plate_cost.to_string().parse().unwrap_or(0.0);
        let labor_cost_f64: f64 = usd_result.cost_breakdown.labor_cost.to_string().parse().unwrap_or(0.0);
        let binding_cost_f64: f64 = usd_result.cost_breakdown.binding_cost.to_string().parse().unwrap_or(0.0);
        let finishing_cost_f64: f64 = usd_result.cost_breakdown.finishing_cost.to_string().parse().unwrap_or(0.0);
        let overhead_f64: f64 = usd_result.cost_breakdown.overhead.to_string().parse().unwrap_or(0.0);
        let total_cost_f64: f64 = usd_result.total_cost.to_string().parse().unwrap_or(0.0);
        let unit_cost_f64: f64 = usd_result.unit_cost.to_string().parse().unwrap_or(0.0);

        let converted_paper = currency_service.convert_amount(paper_cost_f64, Currency::USD, target_currency.clone()).await?;
        let converted_plate = currency_service.convert_amount(plate_cost_f64, Currency::USD, target_currency.clone()).await?;
        let converted_labor = currency_service.convert_amount(labor_cost_f64, Currency::USD, target_currency.clone()).await?;
        let converted_binding = currency_service.convert_amount(binding_cost_f64, Currency::USD, target_currency.clone()).await?;
        let converted_finishing = currency_service.convert_amount(finishing_cost_f64, Currency::USD, target_currency.clone()).await?;
        let converted_overhead = currency_service.convert_amount(overhead_f64, Currency::USD, target_currency.clone()).await?;
        let converted_total = currency_service.convert_amount(total_cost_f64, Currency::USD, target_currency.clone()).await?;
        let converted_unit = currency_service.convert_amount(unit_cost_f64, Currency::USD, target_currency.clone()).await?;

        // Get exchange rate for display
        let rates = currency_service.get_exchange_rates().await?;
        let exchange_rate = rates.rates.get(target_currency.code()).copied().unwrap_or(1.0);

        use std::str::FromStr;
        Ok(CostCalculationResultWithCurrency {
            cost_breakdown: CostBreakdown {
                paper_cost: BigDecimal::from_str(&converted_paper.to_string()).unwrap(),
                plate_cost: BigDecimal::from_str(&converted_plate.to_string()).unwrap(),
                labor_cost: BigDecimal::from_str(&converted_labor.to_string()).unwrap(),
                binding_cost: BigDecimal::from_str(&converted_binding.to_string()).unwrap(),
                finishing_cost: BigDecimal::from_str(&converted_finishing.to_string()).unwrap(),
                overhead: BigDecimal::from_str(&converted_overhead.to_string()).unwrap(),
            },
            total_cost: BigDecimal::from_str(&converted_total.to_string()).unwrap(),
            unit_cost: BigDecimal::from_str(&converted_unit.to_string()).unwrap(),
            currency: target_currency,
            exchange_rate,
        })
    }
}

#[derive(Debug)]
pub struct CostCalculationResult {
    pub cost_breakdown: CostBreakdown,
    pub total_cost: BigDecimal,
    pub unit_cost: BigDecimal,
}

#[derive(Debug)]
pub struct CostCalculationResultWithCurrency {
    pub cost_breakdown: CostBreakdown,
    pub total_cost: BigDecimal,
    pub unit_cost: BigDecimal,
    pub currency: Currency,
    pub exchange_rate: f64,
}
