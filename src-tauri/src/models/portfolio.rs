use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetSummary {
    pub asset_type: String,
    pub total_value: f64,
    pub total_cost: f64,
    pub total_profit: f64,
    pub total_profit_percent: f64,
    pub daily_profit: f64,
    pub daily_profit_percent: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PortfolioSummary {
    pub total_value: f64,
    pub total_cost: f64,
    pub total_profit: f64,
    pub total_profit_percent: f64,
    pub daily_profit: f64,
    pub daily_profit_percent: f64,
    pub asset_summaries: Vec<AssetSummary>,
}
