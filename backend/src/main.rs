mod api;
mod analysis;
mod config;
mod data;
mod models;

use axum::{Router, routing::get};
use tower_http::cors::{CorsLayer, Any};
use std::net::SocketAddr;
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/health", get(api::health_check))
        .route("/api/company/{ticker}", get(api::company::get_company_analysis))
        .route("/api/company/{ticker}/financials", get(api::company::get_financials))
        .route("/api/company/{ticker}/ratios", get(api::company::get_ratios))
        .route("/api/company/{ticker}/dcf", get(api::company::get_dcf_valuation))
        .route("/api/company/{ticker}/comparable", get(api::company::get_comparable_analysis))
        .route("/api/portfolio/analyze", get(api::portfolio::analyze_portfolio))
        .route("/api/search", get(api::search))
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    tracing::info!("Valytics API running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
