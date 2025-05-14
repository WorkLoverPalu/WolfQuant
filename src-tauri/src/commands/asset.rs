use crate::error::auth::ErrorResponse;
use crate::models::{
    AssetType, UserGroup, Asset, CreateGroupRequest, UpdateGroupRequest, DeleteGroupRequest,
    CreateAssetRequest, UpdateAssetRequest, DeleteAssetRequest, GetUserAssetsRequest,
    MessageResponse,
};
use crate::services::asset::{
    get_asset_types, create_user_group, update_user_group, delete_user_group, get_user_groups,
    create_asset, update_asset, delete_asset, get_user_assets,
};
use tauri::command;
use log::{info, error};

#[command]
pub async fn asset_get_asset_types_command() -> Result<Vec<AssetType>, ErrorResponse> {
    match get_asset_types() {
        Ok(types) => {
            info!("Retrieved {} asset types", types.len());
            Ok(types)
        },
        Err(err) => {
            error!("Failed to get asset types: {}", err);
            Err(err.into())
        },
    }
}

#[command]
pub async fn asset_create_group_command(request: CreateGroupRequest) -> Result<UserGroup, ErrorResponse> {
    info!("Create group request received for user: {}", request.user_id);
    
    match create_user_group(
        request.user_id,
        &request.name,
        request.asset_type_id,
        request.description.as_deref(),
    ) {
        Ok(group) => {
            info!("Group created successfully: {}", group.name);
            Ok(group)
        },
        Err(err) => {
            error!("Failed to create group: {}", err);
            Err(err.into())
        },
    }
}

#[command]
pub async fn asset_update_group_command(request: UpdateGroupRequest) -> Result<UserGroup, ErrorResponse> {
    info!("Update group request received for group: {}", request.id);
    
    match update_user_group(
        request.id,
        request.user_id,
        &request.name,
        request.description.as_deref(),
    ) {
        Ok(group) => {
            info!("Group updated successfully: {}", group.name);
            Ok(group)
        },
        Err(err) => {
            error!("Failed to update group: {}", err);
            Err(err.into())
        },
    }
}

#[command]
pub async fn asset_delete_group_command(request: DeleteGroupRequest) -> Result<MessageResponse, ErrorResponse> {
    info!("Delete group request received for group: {}", request.id);
    
    match delete_user_group(request.id, request.user_id) {
        Ok(_) => {
            info!("Group deleted successfully: {}", request.id);
            Ok(MessageResponse {
                message: "分组删除成功".to_string(),
            })
        },
        Err(err) => {
            error!("Failed to delete group: {}", err);
            Err(err.into())
        },
    }
}

#[command]
pub async fn asset_get_user_groups_command(
    user_id: i64,
    asset_type_id: Option<i64>,
) -> Result<Vec<UserGroup>, ErrorResponse> {
    info!("Get user groups request received for user: {}", user_id);
    
    match get_user_groups(user_id, asset_type_id) {
        Ok(groups) => {
            info!("Retrieved {} groups for user: {}", groups.len(), user_id);
            Ok(groups)
        },
        Err(err) => {
            error!("Failed to get user groups: {}", err);
            Err(err.into())
        },
    }
}

#[command]
pub async fn asset_create_asset_command(request: CreateAssetRequest) -> Result<Asset, ErrorResponse> {
    info!("Create asset request received for user: {}", request.user_id);
    
    match create_asset(
        request.user_id,
        request.group_id,
        request.asset_type_id,
        &request.code,
        &request.name,
        request.current_price,
    ) {
        Ok(asset) => {
            info!("Asset created successfully: {} ({})", asset.name, asset.code);
            Ok(asset)
        },
        Err(err) => {
            error!("Failed to create asset: {}", err);
            Err(err.into())
        },
    }
}

#[command]
pub async fn asset_update_asset_command(request: UpdateAssetRequest) -> Result<Asset, ErrorResponse> {
    info!("Update asset request received for asset: {}", request.id);
    
    match update_asset(
        request.id,
        request.user_id,
        request.group_id,
        &request.name,
        request.current_price,
    ) {
        Ok(asset) => {
            info!("Asset updated successfully: {}", asset.name);
            Ok(asset)
        },
        Err(err) => {
            error!("Failed to update asset: {}", err);
            Err(err.into())
        },
    }
}

#[command]
pub async fn asset_delete_asset_command(request: DeleteAssetRequest) -> Result<MessageResponse, ErrorResponse> {
    info!("Delete asset request received for asset: {}", request.id);
    
    match delete_asset(request.id, request.user_id) {
        Ok(_) => {
            info!("Asset deleted successfully: {}", request.id);
            Ok(MessageResponse {
                message: "资产删除成功".to_string(),
            })
        },
        Err(err) => {
            error!("Failed to delete asset: {}", err);
            Err(err.into())
        },
    }
}

#[command]
pub async fn asset_get_user_assets_command(request: GetUserAssetsRequest) -> Result<Vec<Asset>, ErrorResponse> {
    info!("Get user assets request received for user: {}", request.user_id);
    
    match get_user_assets(request.user_id, request.asset_type_id, request.group_id) {
        Ok(assets) => {
            info!("Retrieved {} assets for user: {}", assets.len(), request.user_id);
            Ok(assets)
        },
        Err(err) => {
            error!("Failed to get user assets: {}", err);
            Err(err.into())
        },
    }
}