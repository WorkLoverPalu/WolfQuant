
import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'

// Define interfaces for the data structures
export interface Asset {
  id: number
  user_id: number
  group_id: number | null
  group_name: string | null
  asset_type_id: number
  asset_type_name: string
  code: string
  name: string
  current_price: number | null
  position_amount: number | null
  position_cost: number | null
  last_updated: number | null
  created_at: number
  updated_at: number
  total_amount: number | null
  total_cost: number | null
  daily_change: number | null
  daily_change_percent: number | null
  total_profit: number | null
  total_profit_percent: number | null
}

export interface AssetType {
  id: number
  name: string
  description: string | null
}

export interface UserGroup {
  id: number
  user_id: number
  name: string
  asset_type_id: number
  asset_type_name: string
  description: string | null
  created_at: number
  updated_at: number
}

export interface WatchlistItem {
  symbol: string
  name: string
  price: string
  unit: string
  change: string
  changePercent: string
  volume: string
  turnover: string
}

export interface UpdateAssetRequest {
  id: number
  user_id: number
  group_id: number | null
  name: string
  current_price: number | null
  position_amount: number | null
  position_cost: number | null
}

export const useAssetStore = defineStore('asset', {
  state: () => ({
    assetTypes: [] as AssetType[],
    userGroups: [] as UserGroup[],
    userAssets: [] as Asset[],
    activeCategory: 'all',
    selectedSymbol: null as WatchlistItem | null,
    positions: {} as Record<string, {
      cost: number,
      amount: number,
      investmentType?: string,
      dayOfWeek?: number,
      dayOfMonth?: number,
      investmentAmount?: number
    }>,
    loading: false,
    error: null as Error | null
  }),

  actions: {
    // Initialize asset data
    async initAssetData() {
      this.loading = true;
      try {
        await this.fetchAssetTypes();
        await this.fetchUserGroups();
        await this.fetchUserAssets();
        this.loading = false;
      } catch (err) {
        this.error = err as Error;
        this.loading = false;
        throw err;
      }
    },

    // Fetch asset types
    async fetchAssetTypes() {
      try {
        const types = await invoke<AssetType[]>('asset_get_asset_types_command');
        this.assetTypes = types;
        return types;
      } catch (err) {
        this.error = err as Error;
        console.error('Failed to fetch asset types:', err);
        throw err;
      }
    },

    // Fetch user groups
    async fetchUserGroups(assetTypeId?: number) {
      try {
        const userId = this.getUserId();
        const groups = await invoke<UserGroup[]>('asset_get_user_groups_command', {
          userId,
          assetTypeId
        });
        this.userGroups = groups;
        return groups;
      } catch (err) {
        this.error = err as Error;
        console.error('Failed to fetch user groups:', err);
        throw err;
      }
    },

    // Fetch user assets
    async fetchUserAssets(assetTypeId?: number, groupId?: number) {
      try {
        const userId = this.getUserId();
        const assets = await invoke<Asset[]>('asset_get_user_assets_command', {
          request: {
            user_id: userId,
            asset_type_id: assetTypeId,
            group_id: groupId
          }
        });
        this.userAssets = assets;

        // Update positions from assets
        assets.forEach(asset => {
          if (asset.position_amount && asset.position_cost) {
            this.positions[asset.code] = {
              cost: asset.position_cost,
              amount: asset.position_amount,
              // We'll need to fetch investment plans separately
            };
          }
        });

        return assets;
      } catch (err) {
        this.error = err as Error;
        console.error('Failed to fetch user assets:', err);
        throw err;
      }
    },

    // Create user group
    async createUserGroup(name: string, assetTypeId: number, description?: string) {
      try {
        const userId = this.getUserId();
        const group = await invoke<UserGroup>('asset_create_group_command', {
          request: {
            user_id: userId,
            name,
            asset_type_id: assetTypeId,
            description
          }
        });
        this.userGroups.push(group);
        return group;
      } catch (err) {
        this.error = err as Error;
        console.error('Failed to create user group:', err);
        throw err;
      }
    },

    // Update user group
    async updateUserGroup(id: number, name: string, description?: string) {
      try {
        const userId = this.getUserId();
        const group = await invoke<UserGroup>('asset_update_group_command', {
          request: {
            id,
            user_id: userId,
            name,
            description
          }
        });

        const index = this.userGroups.findIndex(g => g.id === id);
        if (index !== -1) {
          this.userGroups[index] = group;
        }

        return group;
      } catch (err) {
        this.error = err as Error;
        console.error('Failed to update user group:', err);
        throw err;
      }
    },

    // Delete user group
    async deleteUserGroup(id: number) {
      try {
        const userId = this.getUserId();
        await invoke('asset_delete_group_command', {
          request: {
            id,
            user_id: userId
          }
        });

        this.userGroups = this.userGroups.filter(g => g.id !== id);
        return true;
      } catch (err) {
        this.error = err as Error;
        console.error('Failed to delete user group:', err);
        throw err;
      }
    },

    // Create asset
    async createAsset(groupId: number | null, assetTypeId: number, code: string, name: string, currentPrice?: number) {
      try {
        const userId = this.getUserId();
        const asset = await invoke<Asset>('asset_create_asset_command', {
          request: {
            user_id: userId,
            group_id: groupId,
            asset_type_id: assetTypeId,
            code,
            name,
            current_price: currentPrice
          }
        });

        this.userAssets.push(asset);
        return asset;
      } catch (err) {
        this.error = err as Error;
        console.error('Failed to create asset:', err);
        throw err;
      }
    },

    // Update asset
    async updateAsset(id: number, name: string, groupId: number | null, currentPrice: number | null, positionAmount: number | null, positionCost: number | null) {
      try {
        const userId = this.getUserId();
        const request: UpdateAssetRequest = {
          id,
          user_id: userId,
          group_id: groupId,
          name,
          current_price: currentPrice,
          position_amount: positionAmount,
          position_cost: positionCost
        };

        const asset = await invoke<Asset>('asset_update_asset_command', { request });

        const index = this.userAssets.findIndex(a => a.id === id);
        if (index !== -1) {
          this.userAssets[index] = asset;

          // Update position data
          if (asset.position_amount !== null && asset.position_cost !== null) {
            this.positions[asset.code] = {
              ...this.positions[asset.code],
              cost: asset.position_cost,
              amount: asset.position_amount
            };
          }
        }

        return asset;
      } catch (err) {
        this.error = err as Error;
        console.error('Failed to update asset:', err);
        throw err;
      }
    },

    // Delete asset
    async deleteAsset(id: number) {
      try {
        const userId = this.getUserId();
        await invoke('asset_delete_asset_command', {
          request: {
            id,
            user_id: userId
          }
        });

        const asset = this.userAssets.find(a => a.id === id);
        if (asset) {
          delete this.positions[asset.code];
        }

        this.userAssets = this.userAssets.filter(a => a.id !== id);
        return true;
      } catch (err) {
        this.error = err as Error;
        console.error('Failed to delete asset:', err);
        throw err;
      }
    },

    // Set active category
    setActiveCategory(category: string) {
      this.activeCategory = category;
    },

    // Select symbol
    selectSymbol(symbol: WatchlistItem) {
      this.selectedSymbol = symbol;
    },

    // Update position
    updatePosition(symbol: string, position: {
      cost: number,
      amount: number,
      investmentType?: string,
      dayOfWeek?: number,
      dayOfMonth?: number,
      investmentAmount?: number
    }) {
      this.positions[symbol] = position;

      // Find the corresponding asset and update it
      const asset = this.userAssets.find(a => a.code === symbol);
      if (asset) {
        this.updateAsset(
          asset.id,
          asset.name,
          asset.group_id,
          asset.current_price || undefined,
          position.amount,
          position.cost
        );
      }
    },

    // Helper method to get user ID
    getUserId(): number {
      const userJson = localStorage.getItem('user');
      if (!userJson) {
        throw new Error('User not found');
      }

      const user = JSON.parse(userJson);
      return user.id;
    }
  }
});