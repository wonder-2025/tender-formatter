import { defineStore } from 'pinia'
import { ref, watch } from 'vue'

export interface ErrorReportConfig {
  enabled: boolean
  serverUrl: string
}

export const useSettingsStore = defineStore('settings', () => {
  // 错误日志上报配置
  // 安全修复：移除硬编码服务器地址，引导用户自行配置
  const errorReport = ref<ErrorReportConfig>({
    enabled: false,
    serverUrl: '' // 请在设置中配置您的错误日志服务器地址
  })

  // 测试服务器连接
  const testConnection = async (): Promise<{ success: boolean; message: string }> => {
    try {
      const response = await fetch(errorReport.value.serverUrl.replace('/error-log', '/ping'), {
        method: 'GET',
        headers: { 'Content-Type': 'application/json' }
      })
      if (response.ok) {
        const data = await response.json()
        return { success: true, message: data.message || '连接成功' }
      }
      return { success: false, message: `HTTP ${response.status}` }
    } catch (error) {
      return { success: false, message: error instanceof Error ? error.message : String(error) }
    }
  }

  // 提交错误日志
  const submitErrorLog = async (errorData: {
    error_type: string
    error_message: string
    stack_trace?: string
    user_description?: string
    additional_info?: Record<string, unknown>
  }): Promise<{ success: boolean; message: string }> => {
    if (!errorReport.value.enabled) {
      return { success: false, message: '错误日志上报已禁用' }
    }

    const payload = {
      app_name: 'tender-formatter',
      version: '1.0.0',
      os: navigator.platform,
      ...errorData,
      timestamp: new Date().toISOString()
    }

    try {
      const response = await fetch(errorReport.value.serverUrl, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload)
      })

      if (response.ok) {
        return { success: true, message: '错误日志已提交' }
      }
      return { success: false, message: `HTTP ${response.status}` }
    } catch (error) {
      return { success: false, message: error instanceof Error ? error.message : String(error) }
    }
  }

  // 从本地存储加载设置
  const loadSettings = (): void => {
    const saved = localStorage.getItem('tender-formatter-settings')
    if (saved) {
      try {
        const settings = JSON.parse(saved)
        if (settings.errorReport) {
          errorReport.value = settings.errorReport
        }
      } catch (e) {
        console.error('加载设置失败:', e)
      }
    }
  }

  // 保存设置到本地存储
  watch([errorReport], () => {
    localStorage.setItem('tender-formatter-settings', JSON.stringify({
      errorReport: errorReport.value
    }))
  }, { deep: true })

  return {
    errorReport,
    testConnection,
    submitErrorLog,
    loadSettings
  }
})
