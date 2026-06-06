pub mod yahoo;

use crate::models::FinancialStatements;
use crate::models::Company;

/// Trait for financial data providers.
/// Implementations: Yahoo Finance, Financial Modeling Prep, SEC EDGAR.
pub trait DataProvider {
    async fn get_company(&self, ticker: &str) -> anyhow::Result<Company>;
    async fn get_financials(&self, ticker: &str) -> anyhow::Result<FinancialStatements>;
    async fn get_historical_prices(&self, ticker: &str, years: u32) -> anyhow::Result<Vec<f64>>;
    async fn search(&self, query: &str) -> anyhow::Result<Vec<Company>>;
}
