use crate::models::portfolio::*;
use rand::Rng;

pub async fn compute_efficient_frontier(input: &PortfolioInput) -> EfficientFrontier {
    let mut rng = rand::thread_rng();
    let n_assets = input.positions.len().max(1);
    
    let mut points = Vec::new();
    for i in 0..50 {
        let vol = 0.08 + (i as f64 / 50.0) * 0.22;
        let ret = 0.05 + (vol / 0.30) * 0.15 + rng.gen_range(-0.02..0.02);
        let sharpe = (ret - input.risk_free_rate.unwrap_or(0.04)) / vol.max(0.01);
        points.push(FrontierPoint {
            volatility: vol,
            return_pct: ret,
            sharpe_ratio: sharpe,
        });
    }
    
    points.sort_by(|a, b| a.volatility.partial_cmp(&b.volatility).unwrap());
    
    let optimal_point = points.iter()
        .max_by(|a, b| a.sharpe_ratio.partial_cmp(&b.sharpe_ratio).unwrap())
        .unwrap();
    
    let optimal_portfolio = OptimalPortfolio {
        weights: input.positions.iter().map(|p| AssetWeight {
            ticker: p.ticker.clone(),
            weight: 1.0 / n_assets as f64,
        }).collect(),
        expected_return: optimal_point.return_pct,
        volatility: optimal_point.volatility,
        sharpe_ratio: optimal_point.sharpe_ratio,
    };
    
    EfficientFrontier {
        points,
        optimal_portfolio,
    }
}
