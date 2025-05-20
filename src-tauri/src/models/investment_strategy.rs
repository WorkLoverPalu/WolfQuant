use serde::{Deserialize, Serialize};
use crate::models::transaction::Transaction; 

/// 投资策略结构体，表示一个投资策略的详细信息。
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InvestmentStrategy {
    /// 策略ID
    pub id: i64,
    /// 用户ID
    pub user_id: i64,
    /// 策略名称
    pub name: String,
    /// 策略描述（可选）
    pub description: Option<String>,
    /// 策略类型
    pub strategy_type: String,
    /// 策略参数（序列化为字符串）
    pub parameters: String,
    /// 创建时间（时间戳）
    pub created_at: i64,
    /// 更新时间（时间戳）
    pub updated_at: i64,
}

/// 策略应用结构体，表示某个策略被应用到某个资产的情况。
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StrategyApplication {
    /// 应用ID
    pub id: i64,
    /// 用户ID
    pub user_id: i64,
    /// 策略ID
    pub strategy_id: i64,
    /// 策略名称
    pub strategy_name: String,
    /// 资产ID
    pub asset_id: i64,
    /// 资产名称
    pub asset_name: String,
    /// 资产代码
    pub asset_code: String,
    /// 是否激活
    pub is_active: bool,
    /// 创建时间（时间戳）
    pub created_at: i64,
    /// 更新时间（时间戳）
    pub updated_at: i64,
}

/// 创建投资策略请求结构体。
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateInvestmentStrategyRequest {
    /// 用户ID
    pub user_id: i64,
    /// 策略名称
    pub name: String,
    /// 策略描述（可选）
    pub description: Option<String>,
    /// 策略类型
    pub strategy_type: String,
    /// 策略参数（序列化为字符串）
    pub parameters: String,
}

/// 更新投资策略请求结构体。
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateInvestmentStrategyRequest {
    /// 策略ID
    pub id: i64,
    /// 用户ID
    pub user_id: i64,
    /// 策略名称
    pub name: String,
    /// 策略描述（可选）
    pub description: Option<String>,
    /// 策略参数（序列化为字符串）
    pub parameters: String,
}

/// 删除投资策略请求结构体。
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteInvestmentStrategyRequest {
    /// 策略ID
    pub id: i64,
    /// 用户ID
    pub user_id: i64,
}

/// 应用策略请求结构体。
#[derive(Debug, Serialize, Deserialize)]
pub struct ApplyStrategyRequest {
    /// 用户ID
    pub user_id: i64,
    /// 策略ID
    pub strategy_id: i64,
    /// 资产ID
    pub asset_id: i64,
}

/// 移除策略应用请求结构体。
#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveStrategyApplicationRequest {
    /// 应用ID
    pub id: i64,
    /// 用户ID
    pub user_id: i64,
}

/// 策略回测请求结构体。
#[derive(Debug, Serialize, Deserialize)]
pub struct BacktestStrategyRequest {
    /// 用户ID
    pub user_id: i64,
    /// 策略ID
    pub strategy_id: i64,
    /// 资产ID
    pub asset_id: i64,
    /// 回测开始日期（时间戳）
    pub start_date: i64,
    /// 回测结束日期（时间戳）
    pub end_date: i64,
}

/// 回测结果结构体，包含回测的各项指标和交易明细。
#[derive(Debug, Serialize, Deserialize)]
pub struct BacktestResult {
    /// 初始投资金额
    pub initial_investment: f64,
    /// 最终资产价值
    pub final_value: f64,
    /// 总收益率
    pub total_return: f64,
    /// 年化收益率
    pub annualized_return: f64,
    /// 最大回撤
    pub max_drawdown: f64,
    /// 交易明细列表
    pub transactions: Vec<Transaction>,
    /// 绩效数据点列表
    pub performance_data: Vec<PerformancePoint>,
}

/// 绩效数据点结构体，记录某一时刻的资产表现。
#[derive(Debug, Serialize, Deserialize)]
pub struct PerformancePoint {
    /// 日期（时间戳）
    pub date: i64,
    /// 资产价值
    pub value: f64,
    /// 基准价值（可选）
    pub benchmark_value: Option<f64>,
}