use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InvestmentPlan {
    pub id: i64,
    pub user_id: i64,
    pub asset_id: i64,
    pub asset_name: String,
    pub asset_code: String,
    pub name: String,
    pub frequency: String,
    pub day_of_week: Option<i64>,
    pub day_of_month: Option<i64>,
    pub amount: f64,
    pub is_active: bool,
    pub last_executed: Option<i64>,
    pub next_execution: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveInvestmentPlanRequest {
    pub id: Option<i64>,
    pub user_id: i64,
    pub asset_id: i64,
    pub name: String,
    pub frequency: String,
    pub day_of_week: Option<i64>,
    pub day_of_month: Option<i64>,
    pub amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteInvestmentPlanRequest {
    pub id: i64,
    pub user_id: i64,
}
