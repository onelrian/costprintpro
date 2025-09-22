use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrandingSettings {
    pub id: Uuid,
    #[serde(rename = "companyName")]
    pub company_name: String,
    #[serde(rename = "companyLogoUrl")]
    pub company_logo_url: Option<String>,
    #[serde(rename = "primaryColor")]
    pub primary_color: String,
    #[serde(rename = "secondaryColor")]
    pub secondary_color: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBrandingRequest {
    #[serde(rename = "companyName")]
    pub company_name: Option<String>,
    #[serde(rename = "companyLogoUrl")]
    pub company_logo_url: Option<String>,
    #[serde(rename = "primaryColor")]
    pub primary_color: Option<String>,
    #[serde(rename = "secondaryColor")]
    pub secondary_color: Option<String>,
}

impl Default for BrandingSettings {
    fn default() -> Self {
        BrandingSettings {
            id: Uuid::new_v4(),
            company_name: "CostPrint Pro".to_string(),
            company_logo_url: None,
            primary_color: "#3B82F6".to_string(),
            secondary_color: "#1F2937".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }
}
