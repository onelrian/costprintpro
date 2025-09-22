use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Currency {
    USD,
    FCFA,
    EUR,
    GBP,
    CAD,
}

impl Currency {
    pub fn code(&self) -> &'static str {
        match self {
            Currency::USD => "USD",
            Currency::FCFA => "XAF", // Central African CFA franc
            Currency::EUR => "EUR",
            Currency::GBP => "GBP",
            Currency::CAD => "CAD",
        }
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            Currency::USD => "$",
            Currency::FCFA => "FCFA",
            Currency::EUR => "€",
            Currency::GBP => "£",
            Currency::CAD => "C$",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Currency::USD => "US Dollar",
            Currency::FCFA => "Central African CFA Franc",
            Currency::EUR => "Euro",
            Currency::GBP => "British Pound",
            Currency::CAD => "Canadian Dollar",
        }
    }
}

impl Default for Currency {
    fn default() -> Self {
        Currency::USD
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeRates {
    pub base: Currency,
    pub rates: HashMap<String, f64>,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyConversionRequest {
    pub amount: f64,
    #[serde(rename = "fromCurrency")]
    pub from_currency: Currency,
    #[serde(rename = "toCurrency")]
    pub to_currency: Currency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyConversionResponse {
    #[serde(rename = "originalAmount")]
    pub original_amount: f64,
    #[serde(rename = "convertedAmount")]
    pub converted_amount: f64,
    #[serde(rename = "fromCurrency")]
    pub from_currency: Currency,
    #[serde(rename = "toCurrency")]
    pub to_currency: Currency,
    #[serde(rename = "exchangeRate")]
    pub exchange_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencySettings {
    #[serde(rename = "defaultCurrency")]
    pub default_currency: Currency,
    #[serde(rename = "supportedCurrencies")]
    pub supported_currencies: Vec<Currency>,
}
