use crate::models::*;
use crate::models::company::*;
use chrono::Datelike;
use rust_decimal::Decimal;
use serde::Deserialize;

const YAHOO_BASE: &str = "https://query1.finance.yahoo.com/v8/finance/chart";
const YAHOO_QUOTE: &str = "https://query1.finance.yahoo.com/v7/finance/quote";
const YAHOO_SEARCH: &str = "https://query1.finance.yahoo.com/v1/finance/search";

pub async fn get_company(ticker: &str) -> anyhow::Result<Company> {
    let url = format!("{}?symbols={}", YAHOO_QUOTE, ticker);
    let client = reqwest::Client::new();
    let resp = client.get(&url)
        .header("User-Agent", "Valytics/1.0")
        .send()
        .await?;
    
    let data: QuoteResponse = resp.json().await?;
    let quote = data.quote_response.result.first()
        .ok_or_else(|| anyhow::anyhow!("No data for {}", ticker))?;

    Ok(Company {
        ticker: quote.symbol.clone(),
        name: quote.long_name.clone().unwrap_or_else(|| quote.short_name.clone().unwrap_or_default()),
        exchange: quote.exchange.clone().unwrap_or_default(),
        sector: quote.sector.clone().unwrap_or_default(),
        industry: quote.industry.clone().unwrap_or_default(),
        market_cap: quote.market_cap,
        employees: quote.full_time_employees,
        description: quote.long_business_summary.clone().unwrap_or_default(),
    })
}

pub async fn get_historical_prices(ticker: &str, years: u32) -> anyhow::Result<Vec<f64>> {
    let now = chrono::Utc::now();
    let start = now.timestamp() - (years as i64 * 365 * 24 * 60 * 60);
    let end = now.timestamp();

    let url = format!(
        "{}/{}?period1={}&period2={}&interval=1d",
        YAHOO_BASE, ticker, start, end
    );

    let client = reqwest::Client::new();
    let resp = client.get(&url)
        .header("User-Agent", "Valytics/1.0")
        .send()
        .await?;

    let data: ChartResponse = resp.json().await?;
    let result = data.chart.result.first()
        .ok_or_else(|| anyhow::anyhow!("No price data for {}", ticker))?;

    let quotes = &result.indicators.quote.first()
        .ok_or_else(|| anyhow::anyhow!("No quote data"))?;

    let closes: Vec<f64> = quotes.close.iter()
        .filter_map(|c| c.clone())
        .collect();

    Ok(closes)
}

pub async fn search_ticker(query: &str) -> anyhow::Result<Vec<Company>> {
    let url = format!("{}?q={}", YAHOO_SEARCH, query);
    let client = reqwest::Client::new();
    let resp = client.get(&url)
        .header("User-Agent", "Valytics/1.0")
        .send()
        .await?;

    let data: SearchResponse = resp.json().await?;
    
    let results = data.quotes.iter()
        .filter(|q| q.quote_type == "EQUITY")
        .map(|q| Company {
            ticker: q.symbol.clone(),
            name: q.longname.clone().unwrap_or_else(|| q.shortname.clone().unwrap_or_default()),
            exchange: q.exchange.clone(),
            sector: String::new(),
            industry: String::new(),
            market_cap: None,
            employees: None,
            description: String::new(),
        })
        .collect();

    Ok(results)
}

#[derive(Debug, Deserialize)]
struct QuoteResponse {
    #[serde(rename = "quoteResponse")]
    quote_response: QuoteResult,
}

#[derive(Debug, Deserialize)]
struct QuoteResult {
    result: Vec<QuoteData>,
}

#[derive(Debug, Deserialize)]
struct QuoteData {
    symbol: String,
    #[serde(rename = "longName")]
    long_name: Option<String>,
    #[serde(rename = "shortName")]
    short_name: Option<String>,
    exchange: Option<String>,
    sector: Option<String>,
    industry: Option<String>,
    #[serde(rename = "marketCap")]
    market_cap: Option<f64>,
    #[serde(rename = "fullTimeEmployees")]
    full_time_employees: Option<i64>,
    #[serde(rename = "longBusinessSummary")]
    long_business_summary: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ChartResponse {
    chart: ChartData,
}

#[derive(Debug, Deserialize)]
struct ChartData {
    result: Vec<ChartResult>,
}

#[derive(Debug, Deserialize)]
struct ChartResult {
    indicators: Indicators,
}

#[derive(Debug, Deserialize)]
struct Indicators {
    quote: Vec<QuoteIndicators>,
}

#[derive(Debug, Deserialize)]
struct QuoteIndicators {
    close: Vec<Option<f64>>,
}

#[derive(Debug, Deserialize)]
struct SearchResponse {
    quotes: Vec<SearchQuote>,
}

#[derive(Debug, Deserialize)]
struct SearchQuote {
    symbol: String,
    longname: Option<String>,
    shortname: Option<String>,
    exchange: String,
    #[serde(rename = "quoteType")]
    quote_type: String,
}
