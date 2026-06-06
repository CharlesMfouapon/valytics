use crate::models::company::*;
use crate::models::FinancialStatements;

pub fn compute_all_ratios(financials: &FinancialStatements, _prices: &[f64]) -> RatioAnalysis {
    let profitability = compute_profitability_ratios(financials);
    let liquidity = compute_liquidity_ratios(financials);
    let leverage = compute_leverage_ratios(financials);
    let efficiency = compute_efficiency_ratios(financials);
    let valuation_ratios = compute_valuation_ratios(financials, _prices);
    let trends = compute_trends(&profitability, &leverage, &efficiency);

    RatioAnalysis {
        profitability,
        liquidity,
        leverage,
        efficiency,
        valuation_ratios,
        trends,
    }
}

fn compute_profitability_ratios(financials: &FinancialStatements) -> ProfitabilityRatios {
    let mut gross_margin = Vec::new();
    let mut operating_margin = Vec::new();
    let mut net_margin = Vec::new();
    let mut roe = Vec::new();
    let mut roa = Vec::new();
    let mut roic = Vec::new();

    for (i, income) in financials.income_statement.iter().enumerate() {
        let year = income.fiscal_year;
        let revenue = income.revenue;
        
        if revenue > 0.0 {
            gross_margin.push(YearMetric { year, value: income.gross_profit / revenue });
            operating_margin.push(YearMetric { year, value: income.operating_income / revenue });
            net_margin.push(YearMetric { year, value: income.net_income / revenue });
        }

        if let Some(balance) = financials.balance_sheet.get(i) {
            if balance.total_equity > 0.0 {
                roe.push(YearMetric { year, value: income.net_income / balance.total_equity });
            }
            if balance.total_assets > 0.0 {
                roa.push(YearMetric { year, value: income.net_income / balance.total_assets });
            }
            let invested_capital = balance.total_equity + balance.long_term_debt;
            if invested_capital > 0.0 {
                let nopat = income.operating_income * (1.0 - (income.income_tax / income.income_before_tax.max(1.0)));
                roic.push(YearMetric { year, value: nopat / invested_capital });
            }
        }
    }

    let dupont = if let (Some(latest_income), Some(latest_balance)) = (
        financials.income_statement.last(),
        financials.balance_sheet.last(),
    ) {
        let net_margin_val = latest_income.net_income / latest_income.revenue.max(1.0);
        let asset_turnover = latest_income.revenue / latest_balance.total_assets.max(1.0);
        let equity_multiplier = latest_balance.total_assets / latest_balance.total_equity.max(1.0);
        Some(DuPontDecomposition {
            net_margin: net_margin_val,
            asset_turnover,
            equity_multiplier,
            roe: net_margin_val * asset_turnover * equity_multiplier,
        })
    } else {
        None
    };

    ProfitabilityRatios {
        gross_margin,
        operating_margin,
        net_margin,
        roe,
        roa,
        roic,
        dupont_decomposition: dupont,
    }
}

fn compute_liquidity_ratios(financials: &FinancialStatements) -> LiquidityRatios {
    let mut current_ratio = Vec::new();
    let mut quick_ratio = Vec::new();
    let mut cash_ratio = Vec::new();

    for balance in &financials.balance_sheet {
        let year = balance.fiscal_year;
        if balance.current_liabilities > 0.0 {
            current_ratio.push(YearMetric { year, value: balance.current_assets / balance.current_liabilities });
            cash_ratio.push(YearMetric { year, value: balance.cash_and_equivalents / balance.current_liabilities });
        }
    }

    LiquidityRatios {
        current_ratio,
        quick_ratio,
        cash_ratio,
    }
}

fn compute_leverage_ratios(financials: &FinancialStatements) -> LeverageRatios {
    let mut debt_to_equity = Vec::new();
    let mut debt_to_ebitda = Vec::new();
    let mut interest_coverage = Vec::new();

    for (i, balance) in financials.balance_sheet.iter().enumerate() {
        let year = balance.fiscal_year;
        if balance.total_equity > 0.0 {
            debt_to_equity.push(YearMetric { year, value: balance.long_term_debt / balance.total_equity });
        }
        if let Some(income) = financials.income_statement.get(i) {
            let ebitda = income.operating_income + (financials.cash_flow.get(i)
                .map(|cf| cf.capital_expenditure)
                .unwrap_or(0.0)).abs();
            if ebitda > 0.0 {
                debt_to_ebitda.push(YearMetric { year, value: balance.long_term_debt / ebitda });
            }
            if income.interest_expense > 0.0 {
                interest_coverage.push(YearMetric { year, value: income.operating_income / income.interest_expense });
            }
        }
    }

    LeverageRatios {
        debt_to_equity,
        debt_to_ebitda,
        interest_coverage,
    }
}

fn compute_efficiency_ratios(financials: &FinancialStatements) -> EfficiencyRatios {
    let mut asset_turnover = Vec::new();
    let inventory_turnover = Vec::new();
    let days_sales_outstanding = Vec::new();

    for (i, income) in financials.income_statement.iter().enumerate() {
        let year = income.fiscal_year;
        if let Some(balance) = financials.balance_sheet.get(i) {
            if balance.total_assets > 0.0 {
                asset_turnover.push(YearMetric { year, value: income.revenue / balance.total_assets });
            }
        }
    }

    EfficiencyRatios {
        asset_turnover,
        inventory_turnover,
        days_sales_outstanding,
    }
}

fn compute_valuation_ratios(financials: &FinancialStatements, prices: &[f64]) -> ValuationRatios {
    let latest_price = prices.last().copied().unwrap_or(0.0);
    let latest_income = financials.income_statement.last();
    let latest_balance = financials.balance_sheet.last();

    let pe_ratio = latest_income.and_then(|i| {
        if i.eps_diluted > 0.0 { Some(latest_price / i.eps_diluted) } else { None }
    });

    let pb_ratio = latest_balance.and_then(|b| {
        let bvps = b.total_equity / latest_income.map(|i| i.shares_outstanding).unwrap_or(1.0);
        if bvps > 0.0 { Some(latest_price / bvps) } else { None }
    });

    let ps_ratio = latest_income.and_then(|i| {
        let sps = i.revenue / i.shares_outstanding.max(1.0);
        if sps > 0.0 { Some(latest_price / sps) } else { None }
    });

    ValuationRatios {
        pe_ratio,
        pb_ratio,
        ps_ratio,
        ev_to_ebitda: None,
        peg_ratio: None,
        dividend_yield: None,
    }
}

fn compute_trends(
    profitability: &ProfitabilityRatios,
    leverage: &LeverageRatios,
    _efficiency: &EfficiencyRatios,
) -> Vec<RatioTrend> {
    let mut trends = Vec::new();

    if profitability.net_margin.len() >= 2 {
        let values = profitability.net_margin.clone();
        let first = values.first().unwrap().value;
        let last = values.last().unwrap().value;
        let change = (last - first) / first.abs().max(0.001);
        trends.push(RatioTrend {
            name: "Net Margin".into(),
            values,
            trend_direction: if change > 0.05 { "improving".into() } else if change < -0.05 { "declining".into() } else { "stable".into() },
            trend_pct: change,
        });
    }

    if profitability.roe.len() >= 2 {
        let values = profitability.roe.clone();
        let first = values.first().unwrap().value;
        let last = values.last().unwrap().value;
        let change = (last - first) / first.abs().max(0.001);
        trends.push(RatioTrend {
            name: "ROE".into(),
            values,
            trend_direction: if change > 0.05 { "improving".into() } else if change < -0.05 { "declining".into() } else { "stable".into() },
            trend_pct: change,
        });
    }

    if leverage.debt_to_equity.len() >= 2 {
        let values = leverage.debt_to_equity.clone();
        let first = values.first().unwrap().value;
        let last = values.last().unwrap().value;
        let change = (last - first) / first.abs().max(0.001);
        trends.push(RatioTrend {
            name: "Debt/Equity".into(),
            values,
            trend_direction: if change > 0.10 { "declining".into() } else if change < -0.10 { "improving".into() } else { "stable".into() },
            trend_pct: change,
        });
    }

    trends
}
