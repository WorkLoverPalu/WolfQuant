
import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'

// Define interfaces for the data structures
interface InvestmentPlan {
    id: number
    user_id: number
    asset_id: number
    name: string
    frequency: 'daily' | 'weekly' | 'monthly'
    day_of_week: number | null
    day_of_month: number | null
    amount: number
    is_active: boolean
    created_at: string
    updated_at: string
}

interface CreateInvestmentPlanRequest {
    user_id: number
    asset_id: number
    name: string
    frequency: 'daily' | 'weekly' | 'monthly'
    day_of_week: number | null
    day_of_month: number | null
    amount: number
}

interface UpdateInvestmentPlanRequest {
    id: number
    user_id: number
    name: string
    frequency: 'daily' | 'weekly' | 'monthly'
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

        // Check if loading
        isLoading: (state): boolean => state.loading
    },

    actions: {
        // Create a new investment plan
        async createInvestmentPlan(planData: {
            userId: number
            assetId: number
            name: string
            frequency: 'daily' | 'weekly' | 'monthly'
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
            frequency: 'daily' | 'weekly' | 'monthly'
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
                const request: DeleteInvestmentPlanRequest = {
                    id: planId,
                    user_id: userId
                }

                const response = await invoke<MessageResponse>('plan_delete_investment_plan_command', { request })

                // Remove the plan from the state
                this.investmentPlans = this.investmentPlans.filter(plan => plan.id !== planId)

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
        async getUserInvestmentPlans(userId: number, assetId: number | null = null): Promise<InvestmentPlan[]> {
            this.loading = true
            this.error = null

            try {
                const plans = await invoke<InvestmentPlan[]>('plan_get_user_investment_plans_command', { userId, assetId })

                this.investmentPlans = plans
                return plans
            } catch (err) {
                this.error = err as Error
                console.error('Failed to get user investment plans:', err)
                throw err
            } finally {
                this.loading = false
            }
        },

        // Execute due investment plans
        // async executeDueInvestmentPlans(): Promise<MessageResponse> {
        //     this.loading = true
        //     this.error = null

        //     try {
        //         const response = await invoke<MessageResponse>('plan_execute_due_investment_plans_command')
        //         this.message = response.message

        //         // Refresh the plans after execution
        //         if (this.investmentPlans.length > 0 && this.investmentPlans[0].user_id) {
        //             await this.getUserInvestmentPlans(this.investmentPlans[0].user_id)
        //         }

        //         return response
        //     } catch (err) {
        //         this.error = err as Error
        //         console.error('Failed to execute due investment plans:', err)
        //         throw err
        //     } finally {
        //         this.loading = false
        //     }
        // },

        // Clear any error or message
        clearNotifications(): void {
            this.error = null
            this.message = null
        }
    }
})