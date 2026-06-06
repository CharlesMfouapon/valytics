use crate::models::company::*;
use crate::models::valuation::*;
use crate::models::FinancialStatements;
use crate::models::Company;
use crate::analysis::monte_carlo;

pub fn compute_dcf(
    financials: &FinancialStatements,
    prices: &[f64],
    company: &Company,
) -> DCFValuation {
    let assumptions = build_assumptions(financials, company);
    let current_price = prices.last().copied().unwrap_or(100.0);
    
    let wacc = compute_wacc(&assumptions);
    let projected_fcf = project_free_cash_flows(financials, &assumptions);
    let terminal_value = compute_terminal_value(
        projected_fcf.last().copied().unwrap_or(0.0),
        wacc,
        assumptions.terminal_growth_rate,
    );
    
    let enterprise_value = compute_enterprise_value(&projected_fcf, terminal_value, wacc);
    let net_debt = compute_net_debt(financials);
    let equity_value = enterprise_value - net_debt;
    let fair_value_per_share = equity_value / assumptions.shares_outstanding.max(1.0);
    let upside_pct = (fair_value_per_share / current_price) - 1.0;
    
    let recommendation = if upside_pct > 0.15 {
        "undervalued".to_string()
    } else if upside_pct < -0.15 {
        "overvalued".to_string()
    } else {
        "fairly_valued".to_string()
    };

    let projected_values: Vec<YearMetric> = projected_fcf.iter().enumerate().map(|(i, fcf)| {
        YearMetric {
            year: chrono::Utc::now().year() + i as i32 + 1,
            value: *fcf,
        }
    }).collect();

    let sensitivity_table = build_sensitivity_table(
        enterprise_value / assumptions.shares_outstanding.max(1.0),
        wacc,
        assumptions.terminal_growth_rate,
    );

    let monte_carlo_result = monte_carlo::run_dcf_simulation(&assumptions, financials);

    DCFValuation {
        fair_value_per_share,
        current_price,
        upside_pct,
        recommendation,
        projected_free_cash_flows: projected_values,
        terminal_value,
        wacc,
        terminal_growth_rate: assumptions.terminal_growth_rate,
        sensitivity_table,
        monte_carlo: Some(monte_carlo_result),
    }
}

fn build_assumptions(financials: &FinancialStatements, company: &Company) -> DCFAssumptions {
    let mut assumptions = DCFAssumptions::default();
    
    if let (Some(latest_income), Some(latest_balance)) = (
        financials.income_statement.last(),
        financials.balance_sheet.last(),
    ) {
        // Use actual shares outstanding
        assumptions.shares_outstanding = latest_income.shares_outstanding.max(1.0);
        
        // Use actual tax rate
        if latest_income.income_before_tax > 0.0 {
            assumptions.tax_rate = (latest_income.income_tax / latest_income.income_before_tax).clamp(0.0, 0.40);
        }
        
        // Use actual capital structure
        let total_capital = latest_balance.total_equity + latest_balance.long_term_debt;
        if total_capital > 0.0 {
            assumptions.equity_weight = latest_balance.total_equity / total_capital;
            assumptions.debt_weight = latest_balance.long_term_debt / total_capital;
        }
    }
    
    // Adjust beta based on sector
    let sector_betas: Vec<(&str, f64)> = vec![
        ("Technology", 1.2),
        ("Financial Services", 1.1),
        ("Healthcare", 0.9),
        ("Consumer Cyclical", 1.15),
        ("Industrials", 1.1),
        ("Energy", 1.3),
        ("Utilities", 0.7),
        ("Real Estate", 0.8),
    ];
    
    for (sector, beta) in &sector_betas {
        if company.sector.contains(sector) {
            assumptions.beta = *beta;
            break;
        }
    }
    
    assumptions
}

fn compute_wacc(assumptions: &DCFAssumptions) -> f64 {
    let cost_of_equity = assumptions.risk_free_rate 
        + assumptions.beta * assumptions.equity_risk_premium;
    
    let after_tax_cost_of_debt = assumptions.cost_of_debt * (1.0 - assumptions.tax_rate);
    
    assumptions.equity_weight * cost_of_equity 
        + assumptions.debt_weight * after_tax_cost_of_debt
}

fn project_free_cash_flows(
    financials: &FinancialStatements,
    assumptions: &DCFAssumptions,
) -> Vec<f64> {
    let latest_revenue = financials.income_statement.last()
        .map(|i| i.revenue)
        .unwrap_or(100_000_000_000.0);
    
    let mut fcf_projections = Vec::new();
    let mut current_revenue = latest_revenue;
    
    for year in 0..assumptions.projection_years as usize {
        let growth = assumptions.revenue_growth_rates.get(year)
            .copied()
            .unwrap_or(assumptions.revenue_growth_rates.last().copied().unwrap_or(0.03));
        
        current_revenue *= 1.0 + growth;
        
        let ebitda_margin = assumptions.ebitda_margins.get(year)
            .copied()
            .unwrap_or(assumptions.ebitda_margins.last().copied().unwrap_or(0.30));
        
        let ebitda = current_revenue * ebitda_margin;
        let depreciation = current_revenue * assumptions.depreciation_pct_revenue;
        let ebit = ebitda - depreciation;
        let taxes = ebit * assumptions.tax_rate;
        let nopat = ebit - taxes;
        let capex = current_revenue * assumptions.capex_pct_revenue;
        let working_capital_change = current_revenue * assumptions.working_capital_pct_revenue * growth;
        let fcf = nopat + depreciation - capex - working_capital_change;
        
        fcf_projections.push(fcf.max(0.0));
    }
    
    fcf_projections
}

fn compute_terminal_value(
    final_fcf: f64,
    wacc: f64,
    terminal_growth_rate: f64,
) -> f64 {
    if wacc <= terminal_growth_rate {
        return final_fcf * 20.0;
    }
    final_fcf * (1.0 + terminal_growth_rate) / (wacc - terminal_growth_rate)
}

fn compute_enterprise_value(
    projected_fcf: &[f64],
    terminal_value: f64,
    wacc: f64,
) -> f64 {
    let mut pv_sum = 0.0;
    
    for (i, fcf) in projected_fcf.iter().enumerate() {
        let discount_factor = (1.0 + wacc).powi(i as i32 + 1);
        pv_sum += fcf / discount_factor;
    }
    
    let terminal_pv = terminal_value / (1.0 + wacc).powi(projected_fcf.len() as i32);
    
    pv_sum + terminal_pv
}

fn compute_net_debt(financials: &FinancialStatements) -> f64 {
    if let Some(balance) = financials.balance_sheet.last() {
        balance.long_term_debt + balance.current_liabilities - balance.cash_and_equivalents
    } else {
        0.0
    }
}

fn build_sensitivity_table(
    base_value: f64,
    wacc: f64,
    terminal_growth: f64,
) -> Vec<Vec<f64>> {
    let wacc_range: Vec<f64> = vec![
        wacc - 0.02, wacc - 0.01, wacc, wacc + 0.01, wacc + 0.02
    ];
    let growth_range: Vec<f64> = vec![
        terminal_growth - 0.01, terminal_growth - 0.005, terminal_growth,
        terminal_growth + 0.005, terminal_growth + 0.01,
    ];
    
    let mut table = Vec::new();
    for &w in &wacc_range {
        let mut row = Vec::new();
        for &g in &growth_range {
            let adjusted_value = base_value * (wacc / w.max(0.001)) * ((1.0 + g) / (1.0 + terminal_growth.max(0.001)));
            row.push((adjusted_value * 100.0).round() / 100.0);
        }
        table.push(row);
    }
    
    table
}
