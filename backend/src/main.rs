use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing::{info, Level};
use tracing_subscriber;

mod config;
mod db;
mod entities;
mod handlers;
mod models;
mod services;
mod utils;

use config::Config;
use db::Database;

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub config: Config,
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    message: String,
}

async fn health_check() -> Result<Json<HealthResponse>, StatusCode> {
    Ok(Json(HealthResponse {
        status: "ok".to_string(),
        message: "Costprint API is running".to_string(),
    }))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("Starting Costprint Backend API");

    // Load configuration
    let config = Config::from_env()?;
    info!("Configuration loaded");

    // Initialize database
    let db = Database::new(&config.database_url).await?;
    info!("Database connection established");

    // Skip migrations for now - will be handled separately
    // db.migrate().await.map_err(|e| {
    //     tracing::error!("Failed to run database migrations: {}", e);
    //     e
    // })?;
    let app_state = AppState { db, config };

    // Build our application with routes
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/auth/login", post(handlers::auth::login))
        .route("/api/auth/logout", post(handlers::auth::logout))
        .route("/api/auth/me", get(handlers::auth::me))
        .route("/api/jobs", get(handlers::jobs::list_jobs))
        .route("/api/jobs", post(handlers::jobs::create_job))
        .route("/api/jobs/:id", get(handlers::jobs::get_job))
        .route("/api/jobs/:id", put(handlers::jobs::update_job))
        .route("/api/jobs/:id", delete(handlers::jobs::delete_job))
        .route("/api/cost/calculate", post(handlers::costing::calculate_cost))
        .route("/api/cost/preview", post(handlers::costing::preview_cost))
        .route("/api/cost/quick", post(handlers::costing::quick_calculate))
        .route("/api/currency/supported", get(handlers::currency::get_supported_currencies))
        .route("/api/currency/rates", get(handlers::currency::get_exchange_rates))
        .route("/api/currency/convert", get(handlers::currency::convert_currency))
        .route("/api/currency/settings", get(handlers::currency::get_currency_settings))
        .route("/api/settings/cost-parameters", get(handlers::settings::get_cost_parameters))
        .route("/api/settings/cost-parameters", put(handlers::settings::update_cost_parameters))
        .route("/api/settings/branding", get(handlers::settings::get_branding))
        .route("/api/settings/branding", put(handlers::settings::update_branding))
        .route("/api/export/pdf/:job_id", post(handlers::export::export_pdf))
        .route("/api/export/excel/:job_id", post(handlers::export::export_excel))
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    let port = std::env::var("PORT").unwrap_or_else(|_| "3001".to_string());
    let addr = format!("0.0.0.0:{}", port);
    
    info!("Server starting on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
