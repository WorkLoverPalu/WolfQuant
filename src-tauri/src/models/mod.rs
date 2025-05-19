use serde::{Deserialize, Serialize};

// 已有的用户模型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub email_verified: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

// 已有的认证请求/响应模型
#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub verification_code: String,
}
// 登陆
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username_or_email: String,
    pub password: String,
}

// 退出登陆
#[derive(Debug, Serialize, Deserialize)]
pub struct LogoutRequest {
    pub user_id: i64,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionRequest {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendVerificationCodeRequest {
    pub email: String,
    pub purpose: String, // "register" 或 "reset_password"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForgotPasswordRequest {
    pub email: String,
    pub verification_code: String,
    pub new_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResetPasswordRequest {
    pub token: String,
    pub new_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub user: User,
    pub message: String,
    pub token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageResponse {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordResetToken {
    pub user_id: i64,
    pub token: String,
    pub expires_at: i64,
    pub created_at: i64,
}

// 新增的资产类型模型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AssetType {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
}

// 新增的用户分组模型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserGroup {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub asset_type_id: i64,
    pub asset_type_name: String,
    pub description: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

// 新增的资产模型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Asset {
    pub id: i64,
    pub user_id: i64,
    pub group_id: Option<i64>,
    pub group_name: Option<String>,
    pub asset_type_id: i64,
    pub asset_type_name: String,
    pub code: String,
    pub name: String,
    pub current_price: Option<f64>,//当前价格
    pub position_amount: Option<f64>,//持仓数量
    pub position_cost: Option<f64>,//持仓成本
    pub last_updated: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
    // 计算字段
    pub total_profit: Option<f64>,//总利润
    pub total_profit_percent: Option<f64>,//总利润百分比
}

// 新增的交易记录模型
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

// 新增的定投计划模型
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

// 新增的投资策略模型
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

// 新增的策略应用模型
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

// 新增的历史价格模型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PriceHistory {
    pub id: i64,
    pub asset_id: i64,
    pub date: i64,
    pub open_price: Option<f64>,
    pub close_price: f64,
    pub high_price: Option<f64>,
    pub low_price: Option<f64>,
    pub volume: Option<f64>,
    pub created_at: i64,
}

// 新增的交易提醒模型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TradeAlert {
    pub id: i64,
    pub user_id: i64,
    pub asset_id: i64,
    pub asset_name: String,
    pub asset_code: String,
    pub strategy_id: Option<i64>,
    pub strategy_name: Option<String>,
    pub alert_type: String,
    pub message: String,
    pub is_read: bool,
    pub created_at: i64,
}

// 请求模型
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGroupRequest {
    pub user_id: i64,
    pub name: String,
    pub asset_type_id: i64,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateGroupRequest {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteGroupRequest {
    pub id: i64,
    pub user_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAssetRequest {
    pub user_id: i64,
    pub group_id: Option<i64>,
    pub asset_type_id: i64,
    pub code: String,
    pub name: String,
    pub current_price: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAssetRequest {
    pub id: i64,
    pub user_id: i64,
    pub group_id: Option<i64>,
    pub name: String,
    pub current_price: Option<f64>,
    pub position_amount: Option<f64>,
    pub position_cost: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteAssetRequest {
    pub id: i64,
    pub user_id: i64,
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
pub struct CreateInvestmentPlanRequest {
    pub user_id: i64,
    pub asset_id: i64,
    pub name: String,
    pub frequency: String,
    pub day_of_week: Option<i64>,
    pub day_of_month: Option<i64>,
    pub amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateInvestmentPlanRequest {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub frequency: String,
    pub day_of_week: Option<i64>,
    pub day_of_month: Option<i64>,
    pub amount: f64,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteInvestmentPlanRequest {
    pub id: i64,
    pub user_id: i64,
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
pub struct MarkAlertReadRequest {
    pub id: i64,
    pub user_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserAssetsRequest {
    pub user_id: i64,
    pub asset_type_id: Option<i64>,
    pub group_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserTransactionsRequest {
    pub user_id: i64,
    pub asset_id: Option<i64>,
    pub start_date: Option<i64>,
    pub end_date: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAssetPriceHistoryRequest {
    pub asset_id: i64,
    pub start_date: Option<i64>,
    pub end_date: Option<i64>,
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