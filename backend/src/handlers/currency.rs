use axum::{
    extract::Query,
    http::StatusCode,
    response::Json,
    Extension,
};
use serde::Deserialize;

use crate::{
    models::currency::{Currency, CurrencyConversionRequest, CurrencyConversionResponse, CurrencySettings, ExchangeRates},
    services::currency_service::CurrencyService,
    utils::errors::AppError,
};

#[derive(Deserialize)]
pub struct ConversionQuery {
    amount: f64,
    from: String,
    to: String,
}

pub async fn get_supported_currencies() -> Result<Json<Vec<Currency>>, AppError> {
    let currencies = CurrencyService::get_supported_currencies();
    Ok(Json(currencies))
}

pub async fn get_exchange_rates() -> Result<Json<ExchangeRates>, AppError> {
    let service = CurrencyService::new();
    let rates = service.get_exchange_rates().await?;
    Ok(Json(rates))
}

pub async fn convert_currency(
    Query(query): Query<ConversionQuery>,
) -> Result<Json<CurrencyConversionResponse>, AppError> {
    let service = CurrencyService::new();
    
    // Parse currency codes
    let from_currency = parse_currency(&query.from)?;
    let to_currency = parse_currency(&query.to)?;
    
    let request = CurrencyConversionRequest {
        amount: query.amount,
        from_currency,
        to_currency,
    };
    
    let response = service.convert_currency(request).await?;
    Ok(Json(response))
}

pub async fn get_currency_settings() -> Result<Json<CurrencySettings>, AppError> {
    // Return default currency settings
    Ok(Json(CurrencySettings {
        default_currency: Currency::USD,
        supported_currencies: CurrencyService::get_supported_currencies(),
    }))
}

fn parse_currency(code: &str) -> Result<Currency, AppError> {
    match code.to_uppercase().as_str() {
        "USD" => Ok(Currency::USD),
        "XAF" | "FCFA" => Ok(Currency::FCFA),
        "EUR" => Ok(Currency::EUR),
        "GBP" => Ok(Currency::GBP),
        "CAD" => Ok(Currency::CAD),
        _ => Err(AppError::BadRequest(format!("Unsupported currency: {}", code))),
    }
}
