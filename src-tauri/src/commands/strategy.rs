use crate::error::ErrorResponse;
use crate::models::{
    InvestmentStrategy, StrategyApplication, BacktestResult, CreateInvestmentStrategyRequest,
    UpdateInvestmentStrategyRequest, DeleteInvestmentStrategyRequest, ApplyStrategyRequest,
    RemoveStrategyApplicationRequest, BacktestStrategyRequest, MessageResponse,
};
use crate::services::strategy::{
    create_investment_strategy, update_investment_strategy, delete_investment_strategy,
    get_user_investment_strategies, apply_strategy, remove_strategy_application,
    get_user_strategy_applications, backtest_strategy,
};
use tauri::command;
use log::{info, error};

#[command]
pub async fn create_investment_strategy_command(request: CreateInvestmentStrategyRequest) -> Result<InvestmentStrategy, ErrorResponse> {
    info!("Create investment strategy request received for user: {}", request.user_id);
    
    match create_investment_strategy(
        &request.user_id,
        &request.name,
        request.description.as_deref(),
        &request.strategy_type,
        &request.parameters,
    ) {
        Ok(strategy) => {
            info!("Investment strategy created successfully: {}", strategy.name);
            Ok(strategy)
        },
        Err(err) => {
            error!("Failed to create investment strategy: {}", err);
            Err(err.into())
        },
    }
}

#[command]
pub async fn update_investment_strategy_command(request: UpdateInvestmentStrategyRequest) -> Result<InvestmentStrategy, ErrorResponse> {
    info!("Update investment strategy request received for strategy: {}", request.id);
    
    match update_investment_strategy(
        request.id,
        &request.user_id,
        &request.name,
        request.description.as_deref(),
        &request.parameters,
    ) {
        Ok(strategy) => {
            info!("Investment strategy updated successfully: {}", strategy.name);
            Ok(strategy)
        },
        Err(err) => {
            error!("Failed to update investment strategy: {}", err);
            Err(err.into())
        },
    }
}

#[command]
pub async fn delete_investment_strategy_command(request: DeleteInvestmentStrategyRequest) -> Result<MessageResponse, ErrorResponse> {
    info!("Delete investment strategy request received for strategy: {}", request.id);
    
    match delete_investment_strategy(request.id, &request.user_id) {
        Ok(_) => {
            info!("Investment strategy deleted successfully: {}", request.id);
            Ok(MessageResponse {
                message: "投资策略删除成功".to_string(),
            })
        },
        Err(err) => {
            error!("Failed to delete investment strategy: {}", err);
            Err(err.into())
        },
    }
}

#[command]
pub async fn get_user_investment_strategies_command(user_id: String) -> Result<Vec<InvestmentStrategy>, ErrorResponse> {
    info!("Get user investment strategies request received for user: {}", user_id);
    
    match get_user_investment_strategies(&user_id) {
        Ok(strategies) => {
            info!("Retrieved {} investment strategies for user: {}", strategies.len(), user_id);
            Ok(strategies)
        },
        Err(err) => {
            error!("Failed to get user investment strategies: {}", err);
            Err(err.into())
        },
    }
}

#[command]
pub async fn apply_strategy_command(request: ApplyStrategyRequest) -> Result<StrategyApplication, ErrorResponse> {
    info!("Apply strategy request received for user: {}", request.user_id);
    
    match apply_strategy(
        &request.user_id,
        request.strategy_id,
        request.asset_id,
    ) {
        Ok(application) => {
            info!("Strategy applied successfully: {} to {}", application.strategy_name, application.asset_name);
            Ok(application)
        },
        Err(err) => {
            error!("Failed to apply strategy: {}", err);
            Err(err.into())
        },
    }
}

#[command]
pub async fn remove_strategy_application_command(request: RemoveStrategyApplicationRequest) -> Result<MessageResponse, ErrorResponse> {
    info!("Remove strategy application request received for application: {}", request.id);
    
    match remove_strategy_application(request.id, &request.user_id) {
        Ok(_) => {
            info!("Strategy application removed successfully: {}", request.id);
            Ok(MessageResponse {
                message: "策略应用移除成功".to_string(),
            })
        },
        Err(err) => {
            error!("Failed to remove strategy application: {}", err);
            Err(err.into())
        },
    }
}

#[command]
pub async fn get_user_strategy_applications_command(
    user_id: String,
    asset_id: Option<i64>,
) -> Result<Vec<StrategyApplication>, ErrorResponse> {
    info!("Get user strategy applications request received for user: {}", user_id);
    
    match get_user_strategy_applications(&user_id, asset_id) {
        Ok(applications) => {
            info!("Retrieved {} strategy applications for user: {}", applications.len(), user_id);
            Ok(applications)
        },
        Err(err) => {
            error!("Failed to get user strategy applications: {}", err);
            Err(err.into())
        },
    }
}

#[command]
pub async fn backtest_strategy_command(request: BacktestStrategyRequest) -> Result<BacktestResult, ErrorResponse> {
    info!("Backtest strategy request received for user: {}", request.user_id);
    
    match backtest_strategy(
        &request.user_id,
        request.strategy_id,
        request.asset_id,
        request.start_date,
        request.end_date,
    ) {
        Ok(result) => {
            info!("Strategy backtest completed successfully");
            Ok(result)
        },
        Err(err) => {
            error!("Failed to backtest strategy: {}", err);
            Err(err.into())
        },
    }
}