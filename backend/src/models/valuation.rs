use serde::{Deserialize, Serialize};

/// DCF model assumptions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DCFAssumptions {
    pub projection_years: u32,
    pub revenue_growth_rates: Vec<f64>,
    pub ebitda_margins: Vec<f64>,
    pub capex_pct_revenue: f64,
    pub depreciation_pct_revenue: f64,
    pub working_capital_pct_revenue: f64,
    pub tax_rate: f64,
    pub terminal_growth_rate: f64,
    pub risk_free_rate: f64,
    pub equity_risk_premium: f64,
    pub beta: f64,
    pub cost_of_debt: f64,
    pub debt_weight: f64,
    pub equity_weight: f64,
    pub shares_outstanding: f64,
    pub current_price: f64,
}

impl Default for DCFAssumptions {
    fn default() -> Self {
        Self {
            projection_years: 5,
            revenue_growth_rates: vec![0.10, 0.08, 0.06, 0.05, 0.04],
            ebitda_margins: vec![0.30, 0.31, 0.32, 0.32, 0.33],
            capex_pct_revenue: 0.05,
            depreciation_pct_revenue: 0.03,
            working_capital_pct_revenue: 0.02,
            tax_rate: 0.21,
            terminal_growth_rate: 0.025,
            risk_free_rate: 0.04,
            equity_risk_premium: 0.055,
            beta: 1.0,
            cost_of_debt: 0.04,
            debt_weight: 0.30,
            equity_weight: 0.70,
            shares_outstanding: 1_000_000_000.0,
            current_price: 100.0,
        }
    }
}

/// Monte Carlo simulation parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonteCarloParams {
    pub iterations: u32,
    pub revenue_growth_mean: f64,
    pub revenue_growth_std: f64,
    pub ebitda_margin_mean: f64,
    pub ebitda_margin_std: f64,
    pub wacc_mean: f64,
    pub wacc_std: f64,
}
