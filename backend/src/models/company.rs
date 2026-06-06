use serde::{Deserialize, Serialize};

/// Complete company analysis response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyAnalysis {
    pub company: super::Company,
    pub financials: super::FinancialStatements,
    pub ratios: RatioAnalysis,
    pub dcf_valuation: Option<DCFValuation>,
    pub comparable_analysis: Option<ComparableAnalysis>,
    pub insights: Vec<Insight>,
}

/// All computed financial ratios.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatioAnalysis {
    pub profitability: ProfitabilityRatios,
    pub liquidity: LiquidityRatios,
    pub leverage: LeverageRatios,
    pub efficiency: EfficiencyRatios,
    pub valuation_ratios: ValuationRatios,
    pub trends: Vec<RatioTrend>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfitabilityRatios {
    pub gross_margin: Vec<YearMetric>,
    pub operating_margin: Vec<YearMetric>,
    pub net_margin: Vec<YearMetric>,
    pub roe: Vec<YearMetric>,
    pub roa: Vec<YearMetric>,
    pub roic: Vec<YearMetric>,
    pub dupont_decomposition: Option<DuPontDecomposition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuPontDecomposition {
    pub net_margin: f64,
    pub asset_turnover: f64,
    pub equity_multiplier: f64,
    pub roe: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiquidityRatios {
    pub current_ratio: Vec<YearMetric>,
    pub quick_ratio: Vec<YearMetric>,
    pub cash_ratio: Vec<YearMetric>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeverageRatios {
    pub debt_to_equity: Vec<YearMetric>,
    pub debt_to_ebitda: Vec<YearMetric>,
    pub interest_coverage: Vec<YearMetric>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EfficiencyRatios {
    pub asset_turnover: Vec<YearMetric>,
    pub inventory_turnover: Vec<YearMetric>,
    pub days_sales_outstanding: Vec<YearMetric>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuationRatios {
    pub pe_ratio: Option<f64>,
    pub pb_ratio: Option<f64>,
    pub ps_ratio: Option<f64>,
    pub ev_to_ebitda: Option<f64>,
    pub peg_ratio: Option<f64>,
    pub dividend_yield: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YearMetric {
    pub year: i32,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatioTrend {
    pub name: String,
    pub values: Vec<YearMetric>,
    pub trend_direction: String, // "improving", "declining", "stable"
    pub trend_pct: f64,
}

/// DCF valuation result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DCFValuation {
    pub fair_value_per_share: f64,
    pub current_price: f64,
    pub upside_pct: f64,
    pub recommendation: String, // "undervalued", "overvalued", "fairly_valued"
    pub projected_free_cash_flows: Vec<YearMetric>,
    pub terminal_value: f64,
    pub wacc: f64,
    pub terminal_growth_rate: f64,
    pub sensitivity_table: Vec<Vec<f64>>,
    pub monte_carlo: Option<MonteCarloResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonteCarloResult {
    pub simulations: u32,
    pub mean_fair_value: f64,
    pub median_fair_value: f64,
    pub std_dev: f64,
    pub percentile_10: f64,
    pub percentile_90: f64,
}

/// Comparable company analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparableAnalysis {
    pub peer_group: Vec<PeerCompany>,
    pub valuation_multiples: Vec<MultipleComparison>,
    pub implied_value_per_share: f64,
    pub premium_discount_pct: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerCompany {
    pub ticker: String,
    pub name: String,
    pub market_cap: f64,
    pub ev_to_ebitda: Option<f64>,
    pub pe_ratio: Option<f64>,
    pub revenue_growth: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultipleComparison {
    pub multiple_name: String,
    pub company_value: f64,
    pub peer_median: f64,
    pub peer_mean: f64,
    pub premium_discount_pct: f64,
}

/// Key insight or risk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Insight {
    pub category: String, // "strength", "weakness", "risk", "opportunity"
    pub message: String,
    pub severity: String, // "low", "medium", "high"
}
