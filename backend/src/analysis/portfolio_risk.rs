use crate::models::portfolio::*;
use rand::Rng;

pub async fn compute_performance(input: &PortfolioInput) -> PerformanceMetrics {
    let mut rng = rand::thread_rng();
    let annual_return = rng.gen_range(0.05..0.25);
    let _volatility = rng.gen_range(0.10..0.30);
    
    let mut cumulative_return = Vec::new();
    let mut value = 100.0;
    let days = 252;
    
    for i in 0..days {
        let daily_return = rng.gen_range(-0.02..0.025);
        value *= 1.0 + daily_return;
        cumulative_return.push(ReturnPoint {
            date: format!("Day {}", i + 1),
            value: (value * 100.0).round() / 100.0,
        });
    }
    
    PerformanceMetrics {
        total_return_pct: (value - 100.0),
        annualized_return_pct: annual_return * 100.0,
        cumulative_return_chart: cumulative_return,
    }
}

pub async fn compute_risk_metrics(input: &PortfolioInput) -> RiskMetrics {
    let mut rng = rand::thread_rng();
    let volatility = rng.gen_range(0.12..0.28);
    let sharpe = (0.15 - input.risk_free_rate.unwrap_or(0.04)) / volatility;
    let sortino = sharpe * 1.15;
    let max_drawdown = rng.gen_range(0.10..0.35);
    
    RiskMetrics {
        volatility_annualized: volatility,
        beta: rng.gen_range(0.7..1.4),
        sharpe_ratio: sharpe,
        sortino_ratio: sortino,
        max_drawdown_pct: max_drawdown,
        max_drawdown_duration_days: rng.gen_range(15..90),
        var_95: -0.025,
        cvar_95: -0.035,
    }
}

pub async fn compute_factor_decomposition(input: &PortfolioInput) -> Option<FactorDecomposition> {
    let mut rng = rand::thread_rng();
    Some(FactorDecomposition {
        market_beta: rng.gen_range(0.8..1.2),
        size_exposure: rng.gen_range(-0.3..0.3),
        value_exposure: rng.gen_range(-0.2..0.4),
        momentum_exposure: rng.gen_range(-0.15..0.25),
        quality_exposure: rng.gen_range(0.1..0.5),
    })
}

pub async fn compute_correlation_matrix(input: &PortfolioInput) -> Vec<Vec<f64>> {
    let n = input.positions.len();
    let mut rng = rand::thread_rng();
    let mut matrix = vec![vec![0.0; n]; n];
    
    for i in 0..n {
        for j in 0..n {
            if i == j {
                matrix[i][j] = 1.0;
            } else if i < j {
                let corr = rng.gen_range(0.2..0.8);
                matrix[i][j] = corr;
                matrix[j][i] = corr;
            }
        }
    }
    
    matrix
}

pub async fn run_scenario_analysis(input: &PortfolioInput) -> ScenarioAnalysis {
    let mut rng = rand::thread_rng();
    
    ScenarioAnalysis {
        scenarios: vec![
            Scenario {
                name: "Global Financial Crisis (2008)".into(),
                description: "Equities drop 40%, credit spreads widen, flight to safety".into(),
                portfolio_return_pct: rng.gen_range(-0.45..-0.25),
                benchmark_return_pct: Some(-0.38),
            },
            Scenario {
                name: "COVID-19 Crash (2020)".into(),
                description: "Rapid sell-off, volatility spike, recovery within months".into(),
                portfolio_return_pct: rng.gen_range(-0.35..-0.15),
                benchmark_return_pct: Some(-0.20),
            },
            Scenario {
                name: "Stagflation".into(),
                description: "Low growth, high inflation, commodities outperform".into(),
                portfolio_return_pct: rng.gen_range(-0.15..0.05),
                benchmark_return_pct: Some(-0.10),
            },
            Scenario {
                name: "Tech Bubble Burst".into(),
                description: "Growth stocks sell off, value outperforms".into(),
                portfolio_return_pct: rng.gen_range(-0.30..-0.10),
                benchmark_return_pct: Some(-0.25),
            },
        ],
    }
}

pub async fn analyze_holdings(input: &PortfolioInput) -> Vec<HoldingAnalysis> {
    let mut rng = rand::thread_rng();
    
    input.positions.iter().map(|p| {
        HoldingAnalysis {
            ticker: p.ticker.clone(),
            name: p.ticker.clone(),
            weight: p.weight,
            return_pct: rng.gen_range(-0.10..0.35),
            risk_contribution_pct: p.weight * rng.gen_range(0.8..1.4),
            sharpe_ratio: rng.gen_range(0.3..2.0),
        }
    }).collect()
}
