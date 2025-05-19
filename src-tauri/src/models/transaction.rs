use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub id: i64,
    pub user_id: i64,
    pub asset_id: i64,
    pub asset_name: String,
    pub asset_code: String,
    pub transaction_type: String,
    pub amount: f64,
    pub price: f64,
    pub total_cost: f64,
    pub transaction_date: i64,
    pub notes: Option<String>,
    pub created_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTransactionRequest {
    pub user_id: i64,
    pub asset_id: i64,
    pub transaction_type: String,
    pub amount: f64,
    pub price: f64,
    pub transaction_date: i64,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTransactionRequest {
    pub id: i64,
    pub user_id: i64,
    pub transaction_type: String,
    pub amount: f64,
    pub price: f64,
    pub transaction_date: i64,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteTransactionRequest {
    pub id: i64,
    pub user_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserTransactionsRequest {
    pub user_id: i64,
    pub asset_id: Option<i64>,
    pub start_date: Option<i64>,
    pub end_date: Option<i64>,
}
