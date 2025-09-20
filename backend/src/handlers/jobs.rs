use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    Extension,
};
use uuid::Uuid;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use once_cell::sync::Lazy;

use crate::{
    models::{
        CreateJobRequest, Job, JobListQuery, JobListResponse, UpdateJobRequest, UserInfo,
    },
    services::{job_service::JobService, costing_service::CostingService},
    utils::errors::AppError,
    AppState,
};

// In-memory job storage (for demo purposes)
static JOB_STORAGE: Lazy<Arc<Mutex<HashMap<Uuid, Job>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(HashMap::new()))
});

pub async fn list_jobs(
    Query(query): Query<JobListQuery>,
) -> Result<Json<JobListResponse>, AppError> {
    let storage = JOB_STORAGE.lock().map_err(|_| AppError::InternalServerError("Failed to access job storage".to_string()))?;
    
    let jobs: Vec<Job> = storage.values().cloned().collect();
    let total = jobs.len();
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    let total_pages = (total as f64 / limit as f64).ceil() as u64;
    
    // Apply pagination
    let start = ((page - 1) * limit) as usize;
    let end = (start + limit as usize).min(total);
    let paginated_jobs = if start < total {
        jobs[start..end].to_vec()
    } else {
        vec![]
    };
    
    Ok(Json(JobListResponse {
        jobs: paginated_jobs,
        total: total as u64,
        page,
        limit,
        total_pages,
    }))
}

pub async fn create_job(
    State(state): State<AppState>,
    Json(payload): Json<CreateJobRequest>,
) -> Result<Json<Job>, AppError> {
    use crate::models::{JobStatus, CostCalculationRequest};
    use crate::handlers::costing;
    
    // Calculate cost for the job
    let cost_request = CostCalculationRequest {
        job_type: payload.job_type.clone(),
        quantity: payload.quantity,
        specifications: payload.specifications.clone(),
        currency: None, // Default to USD
    };
    
    // Use the costing handler to calculate costs
    let cost_response = costing::calculate_cost(Json(cost_request)).await?;
    let cost_data = cost_response.0;
    
    let job_id = Uuid::new_v4();
    let user_id = Uuid::new_v4(); // In a real app, this would come from authentication
    
    let job = Job {
        id: job_id,
        user_id,
        title: payload.title,
        job_type: payload.job_type,
        specifications: payload.specifications,
        quantity: payload.quantity,
        status: JobStatus::Draft,
        total_cost: cost_data.total_cost,
        unit_cost: cost_data.unit_cost,
        cost_breakdown: cost_data.cost_breakdown,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    // Store the job in memory
    let mut storage = JOB_STORAGE.lock().map_err(|_| AppError::InternalServerError("Failed to access job storage".to_string()))?;
    storage.insert(job_id, job.clone());
    
    Ok(Json(job))
}

pub async fn get_job(
    Path(job_id): Path<Uuid>,
) -> Result<Json<Job>, AppError> {
    let storage = JOB_STORAGE.lock().map_err(|_| AppError::InternalServerError("Failed to access job storage".to_string()))?;
    
    match storage.get(&job_id) {
        Some(job) => Ok(Json(job.clone())),
        None => Err(AppError::NotFound("Job not found".to_string())),
    }
}

pub async fn update_job(
    Path(job_id): Path<Uuid>,
    Json(payload): Json<UpdateJobRequest>,
) -> Result<Json<Job>, AppError> {
    let mut storage = JOB_STORAGE.lock().map_err(|_| AppError::InternalServerError("Failed to access job storage".to_string()))?;
    
    match storage.get_mut(&job_id) {
        Some(job) => {
            // Update job fields if provided
            if let Some(title) = payload.title {
                job.title = title;
            }
            if let Some(status) = payload.status {
                job.status = status;
            }
            job.updated_at = chrono::Utc::now();
            
            Ok(Json(job.clone()))
        }
        None => Err(AppError::NotFound("Job not found".to_string())),
    }
}

pub async fn delete_job(
    Path(job_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let mut storage = JOB_STORAGE.lock().map_err(|_| AppError::InternalServerError("Failed to access job storage".to_string()))?;
    
    match storage.remove(&job_id) {
        Some(_) => Ok(StatusCode::NO_CONTENT),
        None => Err(AppError::NotFound("Job not found".to_string())),
    }
}
