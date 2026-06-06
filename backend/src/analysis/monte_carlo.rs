use crate::models::company::*;
use crate::models::valuation::*;
use crate::models::FinancialStatements;
use rand::Rng;
use rand_distr::{Normal, Distribution};

pub fn run_dcf_simulation(
    assumptions: &DCFAssumptions,
    _financials: &FinancialStatements,
) -> MonteCarloResult {
    let iterations = 10_000;
    let mut rng = rand::thread_rng();
    let mut fair_values = Vec::with_capacity(iterations);
    
    let revenue_growth_dist = Normal::new(0.08, 0.04).unwrap();
    let ebitda_margin_dist = Normal::new(0.30, 0.05).unwrap();
    let wacc_dist = Normal::new(0.09, 0.015).unwrap();
    
    for _ in 0..iterations {
        let sim_growth = revenue_growth_dist.sample(&mut rng).clamp(-0.10, 0.30);
        let sim_margin = ebitda_margin_dist.sample(&mut rng).clamp(0.05, 0.60);
        let sim_wacc = wacc_dist.sample(&mut rng).clamp(0.04, 0.20);
        
        let latest_revenue = 100_000_000_000.0;
        let mut proj_revenue = latest_revenue;
        let mut total_pv = 0.0;
        
        for year in 0..5 {
            let y_growth = sim_growth * (0.95_f64).powi(year as i32);
            proj_revenue *= 1.0 + y_growth;
            
            let ebitda = proj_revenue * sim_margin;
            let depreciation = proj_revenue * 0.03;
            let ebit = ebitda - depreciation;
            let taxes = ebit * assumptions.tax_rate;
            let nopat = ebit - taxes;
            let capex = proj_revenue * assumptions.capex_pct_revenue;
            let fcf = nopat + depreciation - capex;
            
            total_pv += fcf.max(0.0) / (1.0 + sim_wacc).powi(year as i32 + 1);
        }
        
        let terminal_fcf = proj_revenue * sim_margin * (1.0 - assumptions.tax_rate);
        let terminal_value = terminal_fcf * (1.0 + assumptions.terminal_growth_rate) 
            / (sim_wacc - assumptions.terminal_growth_rate).max(0.001);
        let terminal_pv = terminal_value / (1.0 + sim_wacc).powi(5);
        
        let enterprise_value = total_pv + terminal_pv;
        let equity_value = enterprise_value - 50_000_000_000.0;
        let fvps = equity_value / assumptions.shares_outstanding.max(1.0);
        
        fair_values.push(fvps.max(0.0));
    }
    
    fair_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    let n = fair_values.len();
    let mean = fair_values.iter().sum::<f64>() / n as f64;
    let variance = fair_values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / n as f64;
    let std_dev = variance.sqrt();
    
    let median = fair_values[n / 2];
    let p10 = fair_values[(n as f64 * 0.10) as usize];
    let p90 = fair_values[(n as f64 * 0.90) as usize];
    
    MonteCarloResult {
        simulations: iterations as u32,
        mean_fair_value: mean,
        median_fair_value: median,
        std_dev,
        percentile_10: p10,
        percentile_90: p90,
    }
}
