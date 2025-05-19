use serde::{Deserialize, Serialize};
use crate::models::transaction::Transaction; 

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InvestmentStrategy {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub strategy_type: String,
    pub parameters: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StrategyApplication {
    pub id: i64,
    pub user_id: i64,
    pub strategy_id: i64,
    pub strategy_name: String,
    pub asset_id: i64,
    pub asset_name: String,
    pub asset_code: String,
    pub is_active: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateInvestmentStrategyRequest {
    pub user_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub strategy_type: String,
    pub parameters: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateInvestmentStrategyRequest {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub parameters: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteInvestmentStrategyRequest {
    pub id: i64,
    pub user_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplyStrategyRequest {
    pub user_id: i64,
    pub strategy_id: i64,
    pub asset_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveStrategyApplicationRequest {
    pub id: i64,
    pub user_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BacktestStrategyRequest {
    pub user_id: i64,
    pub strategy_id: i64,
    pub asset_id: i64,
    pub start_date: i64,
    pub end_date: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BacktestResult {
    pub initial_investment: f64,
    pub final_value: f64,
    pub total_return: f64,
    pub annualized_return: f64,
    pub max_drawdown: f64,
    pub transactions: Vec<Transaction>,
    pub performance_data: Vec<PerformancePoint>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PerformancePoint {
    pub date: i64,
    pub value: f64,
    pub benchmark_value: Option<f64>,
}
