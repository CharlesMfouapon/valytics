use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub fmp_api_key: Option<String>,
    pub redis_url: Option<String>,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            fmp_api_key: env::var("FMP_API_KEY").ok(),
            redis_url: env::var("REDIS_URL").ok(),
            port: env::var("PORT")
                .unwrap_or_else(|_| "3001".into())
                .parse()
                .unwrap_or(3001),
        }
    }
}
