use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use crate::models::Currency;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobType {
    Book,
    Flyer,
    BusinessCard,
    Brochure,
    Poster,
    Banner,
    Sticker,
    Custom,
}

impl std::fmt::Display for JobType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JobType::Book => write!(f, "book"),
            JobType::Flyer => write!(f, "flyer"),
            JobType::BusinessCard => write!(f, "business_card"),
            JobType::Brochure => write!(f, "brochure"),
            JobType::Poster => write!(f, "poster"),
            JobType::Banner => write!(f, "banner"),
            JobType::Sticker => write!(f, "sticker"),
            JobType::Custom => write!(f, "custom"),
        }
    }
}

impl std::str::FromStr for JobType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "book" => Ok(JobType::Book),
            "flyer" => Ok(JobType::Flyer),
            "business_card" => Ok(JobType::BusinessCard),
            "brochure" => Ok(JobType::Brochure),
            "poster" => Ok(JobType::Poster),
            "banner" => Ok(JobType::Banner),
            "sticker" => Ok(JobType::Sticker),
            "custom" => Ok(JobType::Custom),
            _ => Err(format!("Invalid job type: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobStatus {
    Draft,
    Quoted,
    Approved,
    InProduction,
    Completed,
    Cancelled,
}

impl std::fmt::Display for JobStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JobStatus::Draft => write!(f, "draft"),
            JobStatus::Quoted => write!(f, "quoted"),
            JobStatus::Approved => write!(f, "approved"),
            JobStatus::InProduction => write!(f, "in_production"),
            JobStatus::Completed => write!(f, "completed"),
            JobStatus::Cancelled => write!(f, "cancelled"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobSpecifications {
    #[serde(rename = "paperType")]
    pub paper_type: String,
    #[serde(rename = "paperSize")]
    pub paper_size: String,
    #[serde(rename = "paperWeight")]
    pub paper_weight: Option<String>,
    pub colors: ColorSpecification,
    pub pages: Option<i32>,
    pub binding: Option<String>,
    pub lamination: Option<String>,
    pub finishing: Vec<String>,
    #[serde(rename = "specialRequirements")]
    pub special_requirements: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorSpecification {
    #[serde(rename = "frontColors")]
    pub front_colors: i32,
    #[serde(rename = "backColors")]
    pub back_colors: i32,
    #[serde(rename = "spotColors")]
    pub spot_colors: Vec<String>,
    #[serde(rename = "isFullColor")]
    pub is_full_color: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostBreakdown {
    #[serde(rename = "paperCost")]
    pub paper_cost: BigDecimal,
    #[serde(rename = "plateCost")]
    pub plate_cost: BigDecimal,
    #[serde(rename = "laborCost")]
    pub labor_cost: BigDecimal,
    #[serde(rename = "bindingCost")]
    pub binding_cost: BigDecimal,
    #[serde(rename = "finishingCost")]
    pub finishing_cost: BigDecimal,
    pub overhead: BigDecimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: Uuid,
    #[serde(rename = "userId")]
    pub user_id: Uuid,
    pub title: String,
    #[serde(rename = "jobType")]
    pub job_type: JobType,
    pub quantity: i32,
    pub specifications: JobSpecifications,
    #[serde(rename = "costBreakdown")]
    pub cost_breakdown: CostBreakdown,
    #[serde(rename = "totalCost")]
    pub total_cost: BigDecimal,
    #[serde(rename = "unitCost")]
    pub unit_cost: BigDecimal,
    pub status: JobStatus,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateJobRequest {
    pub title: String,
    #[serde(rename = "jobType")]
    pub job_type: JobType,
    pub quantity: i32,
    pub specifications: JobSpecifications,
}

#[derive(Debug, Deserialize)]
pub struct UpdateJobRequest {
    pub title: Option<String>,
    pub quantity: Option<i32>,
    pub specifications: Option<JobSpecifications>,
    pub status: Option<JobStatus>,
}

#[derive(Debug, Deserialize)]
pub struct JobListQuery {
    pub page: Option<u64>,
    pub limit: Option<u64>,
    pub job_type: Option<JobType>,
    pub status: Option<JobStatus>,
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct JobListResponse {
    pub jobs: Vec<Job>,
    pub total: u64,
    pub page: u64,
    pub limit: u64,
    pub total_pages: u64,
}

#[derive(Debug, Deserialize)]
pub struct CostCalculationRequest {
    #[serde(rename = "jobType")]
    pub job_type: JobType,
    pub quantity: i32,
    pub specifications: JobSpecifications,
    pub currency: Option<Currency>,
}

#[derive(Debug, Serialize)]
pub struct CostCalculationResponse {
    #[serde(rename = "costBreakdown")]
    pub cost_breakdown: CostBreakdown,
    #[serde(rename = "totalCost")]
    pub total_cost: BigDecimal,
    #[serde(rename = "unitCost")]
    pub unit_cost: BigDecimal,
    #[serde(rename = "estimatedDeliveryDays")]
    pub estimated_delivery_days: i32,
    pub currency: Option<Currency>,
    #[serde(rename = "exchangeRate")]
    pub exchange_rate: Option<f64>,
}
