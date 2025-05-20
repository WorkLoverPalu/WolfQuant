import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'

// Define interfaces for the data structures
interface ImportTask {
  id: string
  status: 'Pending' | 'Running' | 'Completed' | 'Failed'
  asset_type?: string
  symbol?: string
  source?: string
  start_time?: string
  end_time?: string
  interval?: string
  // Add any other task properties you need
}

interface AvailableData {
  asset_type: string
  // Add other properties of available data
}

interface MessageResponse {
  message: string
}

// Define the store state interface
interface ImportState {
  importTasks: ImportTask[]
  currentTask: ImportTask | null
  availableData: AvailableData[]
  loading: boolean
  error: Error | null
}

export const useImportStore = defineStore('import', {
  state: (): ImportState => ({
    importTasks: [],
    currentTask: null,
    availableData: [],
    loading: false,
    error: null
  }),

  getters: {
    // Get all pending or running tasks
    pendingTasks: (state): ImportTask[] =>
      state.importTasks.filter(task => task.status === 'Pending' || task.status === 'Running'),

    // Get all completed tasks
    completedTasks: (state): ImportTask[] =>
      state.importTasks.filter(task => task.status === 'Completed'),

    // Get all failed tasks
    failedTasks: (state): ImportTask[] =>
      state.importTasks.filter(task => task.status === 'Failed'),

    // Get available data filtered by type
    availableDataByType: (state) => (type: string): AvailableData[] =>
      state.availableData.filter(data => data.asset_type === type),

    // Get task by ID
    getTaskById: (state) => (id: string): ImportTask | undefined =>
      state.importTasks.find(task => task.id === id),

    // Check if loading
    isLoading: (state): boolean => state.loading
  },

  actions: {
    // Fetch all import tasks
    async fetchImportTasks(): Promise<ImportTask[]> {
      this.loading = true
      this.error = null

      try {
        const tasks = await invoke < ImportTask[] > ('get_import_tasks')
        this.importTasks = tasks
        return tasks
      } catch (err) {
        this.error = err as Error
        console.error('Failed to fetch import tasks:', err)
        throw err
      } finally {
        this.loading = false
      }
    },

    // Fetch a single import task
    async fetchImportTask(id: string): Promise<ImportTask | null> {
      this.loading = true
      this.error = null

      try {
        const task = await invoke < ImportTask > ('get_import_task', { id })
        if (task) {
          this.updateImportTask(task)
        }
        return task
      } catch (err) {
        this.error = err as Error
        console.error(`Failed to fetch import task ${id}:`, err)
        throw err
      } finally {
        this.loading = false
      }
    },

    // Start a new import
    async startImport(params: {
      assetType: string
      symbol: string
      source: string
      startTime: string
      endTime: string
      interval: string
    }): Promise<ImportTask> {
      this.loading = true
      this.error = null

      try {
        const task = await invoke < ImportTask > ('start_import', {
          assetType: params.assetType,
          symbol: params.symbol,
          source: params.source,
          startTime: params.startTime,
          endTime: params.endTime,
          interval: params.interval
        })
        this.addImportTask(task)
        return task
      } catch (err) {
        this.error = err as Error
        console.error('Failed to start import:', err)
        throw err
      } finally {
        this.loading = false
      }
    },

    // Fetch available data
    async fetchAvailableData(): Promise<AvailableData[]> {
      this.loading = true
      this.error = null

      try {
        const data = await invoke < AvailableData[] > ('get_available_data')
        this.availableData = data
        return data
      } catch (err) {
        this.error = err as Error
        console.error('Failed to fetch available data:', err)
        throw err
      } finally {
        this.loading = false
      }
    },

    // Poll task status
    async pollTaskStatus(id: string): Promise<void> {
      const task = this.getTaskById(id)
      if (!task || task.status === 'Completed' || task.status === 'Failed') {
        return
      }

      await this.fetchImportTask(id)

      // Continue polling
      setTimeout(() => {
        this.pollTaskStatus(id)
      }, 2000) // Poll every 2 seconds
    },

    // Helper methods (equivalent to mutations in Vuex)
    addImportTask(task: ImportTask): void {
      const index = this.importTasks.findIndex(t => t.id === task.id)
      if (index === -1) {
        this.importTasks.unshift(task)
      } else {
        this.importTasks.splice(index, 1, task)
      }
      this.currentTask = task
    },

    updateImportTask(task: ImportTask): void {
      const index = this.importTasks.findIndex(t => t.id === task.id)
      if (index !== -1) {
        this.importTasks.splice(index, 1, task)
      }
      if (this.currentTask && this.currentTask.id === task.id) {
        this.currentTask = task
      }
    },

    // Clear error
    clearError(): void {
      this.error = null
    }
  }
})