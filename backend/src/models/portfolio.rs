use serde::{Deserialize, Serialize};

/// Portfolio analysis input.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioInput {
    pub positions: Vec<Position>,
    pub benchmark: Option<String>,
    pub risk_free_rate: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub ticker: String,
    pub weight: f64,
    pub cost_basis: Option<f64>,
}

/// Complete portfolio analysis response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioAnalysis {
    pub performance: PerformanceMetrics,
    pub risk: RiskMetrics,
    pub factor_decomposition: Option<FactorDecomposition>,
    pub correlation_matrix: Vec<Vec<f64>>,
    pub efficient_frontier: EfficientFrontier,
    pub scenario_analysis: ScenarioAnalysis,
    pub holdings: Vec<HoldingAnalysis>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub total_return_pct: f64,
    pub annualized_return_pct: f64,
    pub cumulative_return_chart: Vec<ReturnPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnPoint {
    pub date: String,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskMetrics {
    pub volatility_annualized: f64,
    pub beta: f64,
    pub sharpe_ratio: f64,
    pub sortino_ratio: f64,
    pub max_drawdown_pct: f64,
    pub max_drawdown_duration_days: i32,
    pub var_95: f64,
    pub cvar_95: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactorDecomposition {
    pub market_beta: f64,
    pub size_exposure: f64,
    pub value_exposure: f64,
    pub momentum_exposure: f64,
    pub quality_exposure: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EfficientFrontier {
    pub points: Vec<FrontierPoint>,
    pub optimal_portfolio: OptimalPortfolio,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrontierPoint {
    pub volatility: f64,
    pub return_pct: f64,
    pub sharpe_ratio: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimalPortfolio {
    pub weights: Vec<AssetWeight>,
    pub expected_return: f64,
    pub volatility: f64,
    pub sharpe_ratio: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetWeight {
    pub ticker: String,
    pub weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioAnalysis {
    pub scenarios: Vec<Scenario>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scenario {
    pub name: String,
    pub description: String,
    pub portfolio_return_pct: f64,
    pub benchmark_return_pct: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoldingAnalysis {
    pub ticker: String,
    pub name: String,
    pub weight: f64,
    pub return_pct: f64,
    pub risk_contribution_pct: f64,
    pub sharpe_ratio: f64,
}
