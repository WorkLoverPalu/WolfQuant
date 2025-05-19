
import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'
import { useAssetStore } from './assetStore'

// Define interfaces for the data structures
interface InvestmentPlan {
    id: number
    user_id: number
    asset_id: number
    asset_name: string
    asset_code: string
    name: string
    frequency: string
    day_of_week: number | null
    day_of_month: number | null
    amount: number
    is_active: boolean
    last_executed: number | null
    next_execution: number | null
    created_at: number
    updated_at: number
}

interface CreateInvestmentPlanRequest {
    user_id: number
    asset_id: number
    name: string
    frequency: string
    day_of_week: number | null
    day_of_month: number | null
    amount: number
}

interface UpdateInvestmentPlanRequest {
    id: number
    user_id: number
    name: string
    frequency: string
    day_of_week: number | null
    day_of_month: number | null
    amount: number
    is_active: boolean
}

interface DeleteInvestmentPlanRequest {
    id: number
    user_id: number
}

interface MessageResponse {
    message: string
}

// Define the store state interface
interface InvestmentPlanState {
    investmentPlans: InvestmentPlan[]
    loading: boolean
    error: Error | null
    message: string | null
}

export const useInvestmentPlanStore = defineStore('investmentPlan', {
    state: (): InvestmentPlanState => ({
        investmentPlans: [],
        loading: false,
        error: null,
        message: null
    }),

    getters: {
        // Get all investment plans
        getAllPlans: (state): InvestmentPlan[] => state.investmentPlans,

        // Get active plans only
        getActivePlans: (state): InvestmentPlan[] =>
            state.investmentPlans.filter(plan => plan.is_active),

        // Get plans by asset ID
        getPlansByAsset: (state) => (assetId: number): InvestmentPlan[] => {
            return state.investmentPlans.filter(plan => plan.asset_id === assetId)
        },

        // Get plans by asset code
        getPlansByAssetCode: (state) => (assetCode: string): InvestmentPlan[] => {
            return state.investmentPlans.filter(plan => plan.asset_code === assetCode)
        },

        // Check if loading
        isLoading: (state): boolean => state.loading
    },

    actions: {
        // Create a new investment plan
        async createInvestmentPlan(planData: {
            userId: number
            assetId: number
            name: string
            frequency: string
            dayOfWeek: number | null
            dayOfMonth: number | null
            amount: number
        }): Promise<InvestmentPlan> {
            this.loading = true
            this.error = null

            try {
                const request: CreateInvestmentPlanRequest = {
                    user_id: planData.userId,
                    asset_id: planData.assetId,
                    name: planData.name,
                    frequency: planData.frequency,
                    day_of_week: planData.dayOfWeek,
                    day_of_month: planData.dayOfMonth,
                    amount: planData.amount
                }

                const newPlan = await invoke<InvestmentPlan>('plan_create_investment_plan_command', { request })
                this.investmentPlans.push(newPlan)
                this.message = '定投计划创建成功'

                // Update asset store positions with investment plan info
                const assetStore = useAssetStore()
                const existingPosition = assetStore.positions[newPlan.asset_code] || { cost: 0, amount: 0 }

                assetStore.positions[newPlan.asset_code] = {
                    ...existingPosition,
                    investmentType: this.mapFrequencyToType(newPlan.frequency),
                    dayOfWeek: newPlan.day_of_week !== null ? newPlan.day_of_week : undefined,
                    dayOfMonth: newPlan.day_of_month !== null ? newPlan.day_of_month : undefined,
                    investmentAmount: newPlan.amount
                }

                return newPlan
            } catch (err) {
                this.error = err as Error
                console.error('Failed to create investment plan:', err)
                throw err
            } finally {
                this.loading = false
            }
        },

        // Update an existing investment plan
        async updateInvestmentPlan(planData: {
            id: number
            userId: number
            name: string
            frequency: string
            dayOfWeek: number | null
            dayOfMonth: number | null
            amount: number
            isActive: boolean
        }): Promise<InvestmentPlan> {
            this.loading = true
            this.error = null

            try {
                const request: UpdateInvestmentPlanRequest = {
                    id: planData.id,
                    user_id: planData.userId,
                    name: planData.name,
                    frequency: planData.frequency,
                    day_of_week: planData.dayOfWeek,
                    day_of_month: planData.dayOfMonth,
                    amount: planData.amount,
                    is_active: planData.isActive
                }

                const updatedPlan = await invoke<InvestmentPlan>('plan_update_investment_plan_command', { request })

                // Update the plan in the state
                const index = this.investmentPlans.findIndex(plan => plan.id === updatedPlan.id)
                if (index !== -1) {
                    this.investmentPlans[index] = updatedPlan
                }

                // Update asset store positions with investment plan info
                const assetStore = useAssetStore()
                const existingPosition = assetStore.positions[updatedPlan.asset_code] || { cost: 0, amount: 0 }

                assetStore.positions[updatedPlan.asset_code] = {
                    ...existingPosition,
                    investmentType: planData.isActive ? this.mapFrequencyToType(updatedPlan.frequency) : undefined,
                    dayOfWeek: planData.isActive && updatedPlan.day_of_week !== null ? updatedPlan.day_of_week : undefined,
                    dayOfMonth: planData.isActive && updatedPlan.day_of_month !== null ? updatedPlan.day_of_month : undefined,
                    investmentAmount: planData.isActive ? updatedPlan.amount : undefined
                }

                this.message = '定投计划更新成功'
                return updatedPlan
            } catch (err) {
                this.error = err as Error
                console.error('Failed to update investment plan:', err)
                throw err
            } finally {
                this.loading = false
            }
        },

        // Delete an investment plan
        async deleteInvestmentPlan(planId: number, userId: number): Promise<MessageResponse> {
            this.loading = true
            this.error = null

            try {
                // Find the plan before deleting to get the asset code
                const planToDelete = this.investmentPlans.find(plan => plan.id === planId)

                const request: DeleteInvestmentPlanRequest = {
                    id: planId,
                    user_id: userId
                }

                const response = await invoke<MessageResponse>('plan_delete_investment_plan_command', { request })

                // Remove the plan from the state
                this.investmentPlans = this.investmentPlans.filter(plan => plan.id !== planId)

                // Update asset store positions to remove investment plan info
                if (planToDelete) {
                    const assetStore = useAssetStore()
                    const existingPosition = assetStore.positions[planToDelete.asset_code]

                    if (existingPosition) {
                        assetStore.positions[planToDelete.asset_code] = {
                            cost: existingPosition.cost,
                            amount: existingPosition.amount
                            // Remove investment plan related fields
                        }
                    }
                }

                this.message = response.message
                return response
            } catch (err) {
                this.error = err as Error
                console.error('Failed to delete investment plan:', err)
                throw err
            } finally {
                this.loading = false
            }
        },

        // Get all investment plans for a user
        async getUserInvestmentPlans(assetId: number | null = null): Promise<InvestmentPlan[]> {
            this.loading = true
            this.error = null

            try {
                const plans = await invoke<InvestmentPlan[]>('plan_get_user_investment_plans_command', { userId: this.getUserId(), assetId })
                this.investmentPlans = plans

                // Update asset store positions with investment plan info
                const assetStore = useAssetStore()

                plans.forEach(plan => {
                    if (plan.is_active) {
                        const existingPosition = assetStore.positions[plan.asset_code] || { cost: 0, amount: 0 }

                        assetStore.positions[plan.asset_code] = {
                            ...existingPosition,
                            investmentType: this.mapFrequencyToType(plan.frequency),
                            dayOfWeek: plan.day_of_week !== null ? plan.day_of_week : undefined,
                            dayOfMonth: plan.day_of_month !== null ? plan.day_of_month : undefined,
                            investmentAmount: plan.amount
                        }
                    }
                })

                return plans
            } catch (err) {
                this.error = err as Error
                console.error('Failed to get user investment plans:', err)
                throw err
            } finally {
                this.loading = false
            }
        },

        // Clear any error or message
        clearNotifications(): void {
            this.error = null
            this.message = null
        },

        // Helper method to map backend frequency to frontend investment type
        mapFrequencyToType(frequency: string): string {
            const mapping: Record<string, string> = {
                'DAILY': 'daily',
                'WEEKLY': 'weekly',
                'BIWEEKLY': 'biweekly',
                'MONTHLY': 'monthly'
            }
            return mapping[frequency] || 'none'
        },

        // Helper method to map frontend investment type to backend frequency
        mapTypeToFrequency(type: string): string {
            const mapping: Record<string, string> = {
                'daily': 'DAILY',
                'weekly': 'WEEKLY',
                'biweekly': 'BIWEEKLY',
                'monthly': 'MONTHLY'
            }
            return mapping[type] || 'DAILY'
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
})