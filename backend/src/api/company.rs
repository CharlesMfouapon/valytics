use axum::extract::Path;
use axum::response::Json;
use serde_json::{json, Value};

use crate::analysis::ratios;
use crate::analysis::dcf;
use crate::analysis::comparable;
use crate::data::yahoo;
use crate::models::company::*;

pub async fn get_company_analysis(
    Path(ticker): Path<String>,
) -> Json<Value> {
    let ticker = ticker.to_uppercase();
    
    let company = match yahoo::get_company(&ticker).await {
        Ok(c) => c,
        Err(e) => return Json(json!({ "error": format!("Company not found: {}", e) })),
    };

    let prices = yahoo::get_historical_prices(&ticker, 5).await.unwrap_or_default();
    let financials = yahoo::get_financials(&ticker).await.unwrap_or_else(|_| {
        crate::models::FinancialStatements {
            ticker: ticker.clone(),
            income_statement: vec![],
            balance_sheet: vec![],
            cash_flow: vec![],
        }
    });

    let ratio_analysis = ratios::compute_all_ratios(&financials, &prices);
    let dcf_valuation = dcf::compute_dcf(&financials, &prices, &company);
    let comparable_analysis = comparable::compute_comparable(&ticker, &company).await;
    let insights = generate_insights(&ratio_analysis, &dcf_valuation, &company);

    let analysis = CompanyAnalysis {
        company,
        financials,
        ratios: ratio_analysis,
        dcf_valuation: Some(dcf_valuation),
        comparable_analysis: Some(comparable_analysis),
        insights,
    };

    Json(serde_json::to_value(analysis).unwrap_or(json!({ "error": "Serialization failed" })))
}

pub async fn get_financials(
    Path(ticker): Path<String>,
) -> Json<Value> {
    let ticker = ticker.to_uppercase();
    match yahoo::get_financials(&ticker).await {
        Ok(f) => Json(serde_json::to_value(f).unwrap_or(json!({}))),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

pub async fn get_ratios(
    Path(ticker): Path<String>,
) -> Json<Value> {
    let ticker = ticker.to_uppercase();
    let prices = yahoo::get_historical_prices(&ticker, 5).await.unwrap_or_default();
    let financials = yahoo::get_financials(&ticker).await.unwrap_or_else(|_| {
        crate::models::FinancialStatements {
            ticker: ticker.clone(),
            income_statement: vec![],
            balance_sheet: vec![],
            cash_flow: vec![],
        }
    });
    let ratios = ratios::compute_all_ratios(&financials, &prices);
    Json(serde_json::to_value(ratios).unwrap_or(json!({})))
}

pub async fn get_dcf_valuation(
    Path(ticker): Path<String>,
) -> Json<Value> {
    let ticker = ticker.to_uppercase();
    let prices = yahoo::get_historical_prices(&ticker, 5).await.unwrap_or_default();
    let financials = yahoo::get_financials(&ticker).await.unwrap_or_else(|_| {
        crate::models::FinancialStatements {
            ticker: ticker.clone(),
            income_statement: vec![],
            balance_sheet: vec![],
            cash_flow: vec![],
        }
    });
    let company = yahoo::get_company(&ticker).await.unwrap_or(Company {
        ticker: ticker.clone(),
        name: String::new(),
        exchange: String::new(),
        sector: String::new(),
        industry: String::new(),
        market_cap: None,
        employees: None,
        description: String::new(),
    });
    let dcf = dcf::compute_dcf(&financials, &prices, &company);
    Json(serde_json::to_value(dcf).unwrap_or(json!({})))
}

pub async fn get_comparable_analysis(
    Path(ticker): Path<String>,
) -> Json<Value> {
    let ticker = ticker.to_uppercase();
    let company = yahoo::get_company(&ticker).await.unwrap_or(Company {
        ticker: ticker.clone(),
        name: String::new(),
        exchange: String::new(),
        sector: String::new(),
        industry: String::new(),
        market_cap: None,
        employees: None,
        description: String::new(),
    });
    let comparable = comparable::compute_comparable(&ticker, &company).await;
    Json(serde_json::to_value(comparable).unwrap_or(json!({})))
}

fn generate_insights(ratios: &RatioAnalysis, dcf: &DCFValuation, company: &crate::models::Company) -> Vec<Insight> {
    let mut insights = Vec::new();

    // Profitability insights
    if let Some(latest_roe) = ratios.profitability.roe.last() {
        if latest_roe.value > 0.20 {
            insights.push(Insight {
                category: "strength".into(),
                message: format!("Strong ROE of {:.1}% indicates efficient use of shareholder capital", latest_roe.value * 100.0),
                severity: "low".into(),
            });
        } else if latest_roe.value < 0.05 {
            insights.push(Insight {
                category: "weakness".into(),
                message: format!("Weak ROE of {:.1}% suggests inefficient capital allocation", latest_roe.value * 100.0),
                severity: "medium".into(),
            });
        }
    }

    // Leverage insights
    if let Some(latest_de) = ratios.leverage.debt_to_equity.last() {
        if latest_de.value > 2.0 {
            insights.push(Insight {
                category: "risk".into(),
                message: format!("High debt-to-equity ratio of {:.1}x — monitor leverage and interest coverage", latest_de.value),
                severity: "high".into(),
            });
        }
    }

    // Valuation insights
    if dcf.upside_pct > 0.15 {
        insights.push(Insight {
            category: "opportunity".into(),
            message: format!("DCF suggests {:.1}% upside — stock may be undervalued relative to intrinsic value", dcf.upside_pct * 100.0),
            severity: "low".into(),
        });
    } else if dcf.upside_pct < -0.15 {
        insights.push(Insight {
            category: "risk".into(),
            message: format!("DCF suggests {:.1}% downside — stock may be overvalued", dcf.upside_pct.abs() * 100.0),
            severity: "medium".into(),
        });
    }

    // Market cap insight
    if let Some(mcap) = company.market_cap {
        if mcap > 100_000_000_000.0 {
            insights.push(Insight {
                category: "strength".into(),
                message: format!("Large-cap stock (${:.0}B market cap) — generally more stable and liquid", mcap / 1_000_000_000.0),
                severity: "low".into(),
            });
        }
    }

    insights
}
