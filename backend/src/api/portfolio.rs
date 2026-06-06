use axum::response::Json;
use serde_json::{json, Value};

use crate::analysis::portfolio_risk;
use crate::analysis::efficient_frontier;
use crate::models::portfolio::*;

pub async fn analyze_portfolio(
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Json<Value> {
    let tickers_str = params.get("tickers").cloned().unwrap_or_default();
    let weights_str = params.get("weights").cloned().unwrap_or_default();
    let benchmark = params.get("benchmark").cloned();
    let risk_free_rate = params.get("risk_free_rate")
        .and_then(|r| r.parse().ok())
        .unwrap_or(0.04);

    let tickers: Vec<String> = tickers_str.split(',')
        .map(|s| s.trim().to_uppercase())
        .filter(|s| !s.is_empty())
        .collect();

    let weights: Vec<f64> = weights_str.split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    if tickers.is_empty() || tickers.len() != weights.len() {
        return Json(json!({ "error": "Invalid tickers or weights. Provide comma-separated lists of equal length." }));
    }

    let positions: Vec<Position> = tickers.iter().zip(weights.iter())
        .map(|(t, w)| Position {
            ticker: t.clone(),
            weight: *w,
            cost_basis: None,
        })
        .collect();

    let portfolio_input = PortfolioInput {
        positions,
        benchmark,
        risk_free_rate: Some(risk_free_rate),
    };

    let performance = portfolio_risk::compute_performance(&portfolio_input).await;
    let risk = portfolio_risk::compute_risk_metrics(&portfolio_input).await;
    let factor_decomposition = portfolio_risk::compute_factor_decomposition(&portfolio_input).await;
    let correlation_matrix = portfolio_risk::compute_correlation_matrix(&portfolio_input).await;
    let efficient_frontier_result = efficient_frontier::compute_efficient_frontier(&portfolio_input).await;
    let scenario_analysis = portfolio_risk::run_scenario_analysis(&portfolio_input).await;
    let holdings = portfolio_risk::analyze_holdings(&portfolio_input).await;

    let analysis = PortfolioAnalysis {
        performance,
        risk,
        factor_decomposition,
        correlation_matrix,
        efficient_frontier: efficient_frontier_result,
        scenario_analysis,
        holdings,
    };

    Json(serde_json::to_value(analysis).unwrap_or(json!({ "error": "Analysis failed" })))
}
