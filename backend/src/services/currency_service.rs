use std::collections::HashMap;
use reqwest;
use serde_json::Value;
use crate::models::currency::{Currency, ExchangeRates, CurrencyConversionRequest, CurrencyConversionResponse};
use crate::utils::errors::AppError;

pub struct CurrencyService {
    client: reqwest::Client,
    // Using a free exchange rate API
    api_url: String,
}

impl CurrencyService {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            // Using exchangerate-api.com free tier
            api_url: "https://api.exchangerate-api.com/v4/latest/USD".to_string(),
        }
    }

    pub async fn get_exchange_rates(&self) -> Result<ExchangeRates, AppError> {
        // For demo purposes, return hardcoded rates
        // In production, you would fetch from a real API
        let mut rates = HashMap::new();
        
        // Sample exchange rates (USD base)
        rates.insert("USD".to_string(), 1.0);
        rates.insert("XAF".to_string(), 620.0); // 1 USD = 620 FCFA (approximate)
        rates.insert("EUR".to_string(), 0.85);  // 1 USD = 0.85 EUR (approximate)
        rates.insert("GBP".to_string(), 0.73);  // 1 USD = 0.73 GBP (approximate)
        rates.insert("CAD".to_string(), 1.35);  // 1 USD = 1.35 CAD (approximate)

        Ok(ExchangeRates {
            base: Currency::USD,
            rates,
            last_updated: chrono::Utc::now(),
        })
    }

    pub async fn convert_currency(&self, request: CurrencyConversionRequest) -> Result<CurrencyConversionResponse, AppError> {
        let rates = self.get_exchange_rates().await?;
        
        let from_rate = rates.rates.get(request.from_currency.code())
            .copied()
            .unwrap_or(1.0);
        
        let to_rate = rates.rates.get(request.to_currency.code())
            .copied()
            .unwrap_or(1.0);
        
        // Convert from source currency to USD, then to target currency
        let usd_amount = request.amount / from_rate;
        let converted_amount = usd_amount * to_rate;
        let exchange_rate = to_rate / from_rate;

        Ok(CurrencyConversionResponse {
            original_amount: request.amount,
            converted_amount,
            from_currency: request.from_currency,
            to_currency: request.to_currency,
            exchange_rate,
        })
    }

    pub fn get_supported_currencies() -> Vec<Currency> {
        vec![
            Currency::USD,
            Currency::FCFA,
            Currency::EUR,
            Currency::GBP,
            Currency::CAD,
        ]
    }

    // Helper method to convert a single amount
    pub async fn convert_amount(&self, amount: f64, from: Currency, to: Currency) -> Result<f64, AppError> {
        if from.code() == to.code() {
            return Ok(amount);
        }

        let request = CurrencyConversionRequest {
            amount,
            from_currency: from,
            to_currency: to,
        };

        let response = self.convert_currency(request).await?;
        Ok(response.converted_amount)
    }
}
