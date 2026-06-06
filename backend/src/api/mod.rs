pub mod company;
pub mod portfolio;

use axum::response::Json;
use serde_json::{json, Value};

pub async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "version": "0.1.0",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

#[derive(serde::Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

pub async fn search(
    axum::extract::Query(query): axum::extract::Query<SearchQuery>,
) -> Json<Value> {
    let results = crate::data::yahoo::search_ticker(&query.q).await;
    
    match results {
        Ok(companies) => Json(json!({ "results": companies })),
        Err(_) => Json(json!({ "results": [], "error": "Search unavailable" })),
    }
}
