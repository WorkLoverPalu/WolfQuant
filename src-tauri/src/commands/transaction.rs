/**
 * 交易
 */
use crate::error::auth::ErrorResponse;
use crate::models::{
    Transaction, CreateTransactionRequest, UpdateTransactionRequest, DeleteTransactionRequest,
    GetUserTransactionsRequest, MessageResponse,
};
use crate::services::transaction::{
    create_transaction, update_transaction, delete_transaction, get_user_transactions,
};
use tauri::command;
use log::{info, error};

#[command]
pub async fn create_transaction_command(request: CreateTransactionRequest) -> Result<Transaction, ErrorResponse> {
    info!("Create transaction request received for user: {}", request.user_id);
    
    match create_transaction(
        request.user_id,
        request.asset_id,
        &request.transaction_type,
        request.amount,
        request.price,
        request.transaction_date,
        request.notes.as_deref(),
    ) {
        Ok(transaction) => {
            info!("Transaction created successfully: {} {}", transaction.transaction_type, transaction.asset_name);
            Ok(transaction)
        },
        Err(err) => {
            error!("Failed to create transaction: {}", err);
            Err(err.into())
        },
    }
}

#[command]
pub async fn update_transaction_command(request: UpdateTransactionRequest) -> Result<Transaction, ErrorResponse> {
    info!("Update transaction request received for transaction: {}", request.id);
    
    match update_transaction(
        request.id,
        request.user_id,
        &request.transaction_type,
        request.amount,
        request.price,
        request.transaction_date,
        request.notes.as_deref(),
    ) {
        Ok(transaction) => {
            info!("Transaction updated successfully: {}", request.id);
            Ok(transaction)
        },
        Err(err) => {
            error!("Failed to update transaction: {}", err);
            Err(err.into())
        },
    }
}

#[command]
pub async fn delete_transaction_command(request: DeleteTransactionRequest) -> Result<MessageResponse, ErrorResponse> {
    info!("Delete transaction request received for transaction: {}", request.id);
    
    match delete_transaction(request.id, request.user_id) {
        Ok(_) => {
            info!("Transaction deleted successfully: {}", request.id);
            Ok(MessageResponse {
                message: "交易记录删除成功".to_string(),
            })
        },
        Err(err) => {
            error!("Failed to delete transaction: {}", err);
            Err(err.into())
        },
    }
}

#[command]
pub async fn get_user_transactions_command(request: GetUserTransactionsRequest) -> Result<Vec<Transaction>, ErrorResponse> {
    info!("Get user transactions request received for user: {}", request.user_id);
    
    match get_user_transactions(request.user_id, request.asset_id, request.start_date, request.end_date) {
        Ok(transactions) => {
            info!("Retrieved {} transactions for user: {}", transactions.len(), request.user_id);
            Ok(transactions)
        },
        Err(err) => {
            error!("Failed to get user transactions: {}", err);
            Err(err.into())
        },
    }
}