/**
 * 定投计划
 */
use crate::error::auth::ErrorResponse;
use crate::models::{
    DeleteInvestmentPlanRequest, InvestmentPlan, MessageResponse, SaveInvestmentPlanRequest,
};
use crate::services::investment_plan::{
    delete_investment_plan, execute_due_investment_plans, get_user_investment_plans,
    save_investment_plan,
};
use log::{error, info};
use tauri::command;
use tauri::http::request;

/// 创建定投计划
#[command]
pub async fn plan_save_investment_plan(
    request: SaveInvestmentPlanRequest,
) -> Result<InvestmentPlan, ErrorResponse> {
    info!(
        "Create investment plan request received for user: {}",
        request.user_id
    );

    match save_investment_plan(
        request.id,
        request.user_id,
        request.asset_id,
        &request.name,
        &request.frequency,
        request.day_of_week,
        request.day_of_month,
        request.amount,
        true,
    ) {
        Ok(plan) => {
            info!("Investment plan created successfully: {}", plan.name);
            Ok(plan)
        }
        Err(err) => {
            error!("Failed to create investment plan: {}", err);
            Err(err.into())
        }
    }
}

/// 删除定投
#[command]
pub async fn plan_delete_investment_plan_command(
    request: DeleteInvestmentPlanRequest,
) -> Result<MessageResponse, ErrorResponse> {
    info!(
        "Delete investment plan request received for plan: {}",
        request.id
    );

    match delete_investment_plan(request.id, request.user_id) {
        Ok(_) => {
            info!("Investment plan deleted successfully: {}", request.id);
            Ok(MessageResponse {
                message: "定投计划删除成功".to_string(),
            })
        }
        Err(err) => {
            error!("Failed to delete investment plan: {}", err);
            Err(err.into())
        }
    }
}

/// 获取用户所有的定投
#[command]
pub async fn plan_get_user_investment_plans_command(
    user_id: i64,
    asset_id: Option<i64>,
) -> Result<Vec<InvestmentPlan>, ErrorResponse> {
    info!(
        "Get user investment plans request received for user: {}",
        user_id
    );

    match get_user_investment_plans(user_id, asset_id) {
        Ok(plans) => {
            info!(
                "Retrieved {} investment plans for user: {}",
                plans.len(),
                user_id
            );
            Ok(plans)
        }
        Err(err) => {
            error!("Failed to get user investment plans: {}", err);
            Err(err.into())
        }
    }
}

/// 执行定投
#[command]
pub async fn plan_execute_due_investment_plans_command() -> Result<MessageResponse, ErrorResponse> {
    info!("Execute due investment plans request received");

    match execute_due_investment_plans() {
        Ok(count) => {
            info!("Executed {} due investment plans", count);
            Ok(MessageResponse {
                message: format!("成功执行{}个到期定投计划", count),
            })
        }
        Err(err) => {
            error!("Failed to execute due investment plans: {}", err);
            Err(err.into())
        }
    }
}
