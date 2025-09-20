use crate::{
    db::Database,
    models::{CostParameters, UpdateCostParametersRequest},
    utils::errors::AppError,
};

pub struct CostParametersService<'a> {
    db: &'a sea_orm::DatabaseConnection,
}

impl<'a> CostParametersService<'a> {
    pub fn new(db: &'a sea_orm::DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_current_parameters(&self) -> Result<CostParameters, AppError> {
        // For now, return default parameters
        // In a real implementation, this would query the database
        Ok(CostParameters::default())
    }

    pub async fn update_parameters(
        &self,
        request: UpdateCostParametersRequest,
    ) -> Result<CostParameters, AppError> {
        let mut current_params = self.get_current_parameters().await?;

        if let Some(paper_cost_per_sheet) = request.paper_cost_per_sheet {
            current_params.paper_cost_per_sheet = paper_cost_per_sheet;
        }

        if let Some(plate_cost_per_job) = request.plate_cost_per_job {
            current_params.plate_cost_per_job = plate_cost_per_job;
        }

        if let Some(labor_cost_per_hour) = request.labor_cost_per_hour {
            current_params.labor_cost_per_hour = labor_cost_per_hour;
        }

        if let Some(binding_cost_per_unit) = request.binding_cost_per_unit {
            current_params.binding_cost_per_unit = binding_cost_per_unit;
        }

        if let Some(overhead_percentage) = request.overhead_percentage {
            current_params.overhead_percentage = overhead_percentage;
        }

        if let Some(profit_margin_percentage) = request.profit_margin_percentage {
            current_params.profit_margin_percentage = profit_margin_percentage;
        }

        current_params.updated_at = chrono::Utc::now();

        // TODO: Save to database
        Ok(current_params)
    }
}
