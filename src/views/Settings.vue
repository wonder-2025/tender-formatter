<template>
  <div class="settings-page">
    <div class="format-card">
      <h3>设置</h3>
      
      <!-- API 配置 -->
      <div class="settings-section">
        <h4 class="section-title">🔑 API 配置</h4>
        <el-form label-width="100px">
          <el-form-item label="提供商">
            <el-select v-model="apiConfig.provider" @change="handleProviderChange">
              <el-option label="百度千帆" value="baidu" />
              <el-option label="阿里通义" value="aliyun" />
              <el-option label="OpenAI" value="openai" />
              <el-option label="DeepSeek" value="deepseek" />
              <el-option label="自定义" value="custom" />
            </el-select>
          </el-form-item>
          <el-form-item label="API Key">
            <el-input 
              v-model="apiConfig.apiKey" 
              :type="showApiKey ? 'text' : 'password'" 
              placeholder="请输入 API Key"
            >
              <template #append>
                <el-button @click="showApiKey = !showApiKey">
                  {{ showApiKey ? '隐藏' : '显示' }}
                </el-button>
              </template>
            </el-input>
            <el-button type="primary" text @click="testConnection" :loading="testing">
              测试连接
            </el-button>
          </el-form-item>
          <el-form-item label="模型">
            <el-select v-model="apiConfig.model">
              <el-option 
                v-for="model in availableModels" 
                :key="model.value" 
                :label="model.label" 
                :value="model.value" 
              />
            </el-select>
          </el-form-item>
          
          <!-- 备用模型配置 -->
          <el-divider content-position="left">
            <el-checkbox v-model="apiConfig.enableBackup">启用备用模型</el-checkbox>
          </el-divider>
          
          <template v-if="apiConfig.enableBackup">
            <el-form-item label="备用提供商">
              <el-select v-model="apiConfig.backupProvider" @change="handleBackupProviderChange">
                <el-option label="百度千帆" value="baidu" />
                <el-option label="阿里通义" value="aliyun" />
                <el-option label="OpenAI" value="openai" />
                <el-option label="DeepSeek" value="deepseek" />
              </el-select>
            </el-form-item>
            <el-form-item label="备用API Key">
              <el-input 
                v-model="apiConfig.backupApiKey" 
                type="password" 
                show-password
                placeholder="请输入备用 API Key"
              />
            </el-form-item>
            <el-form-item label="备用模型">
              <el-select v-model="apiConfig.backupModel">
                <el-option 
                  v-for="model in backupModels" 
                  :key="model.value" 
                  :label="model.label" 
                  :value="model.value" 
                />
              </el-select>
            </el-form-item>
            <el-alert type="info" :closable="false" style="margin-bottom: 16px">
              主模型请求失败时，将自动切换到备用模型重试
            </el-alert>
          </template>
        </el-form>
      </div>
      
      <!-- 输出设置 -->
      <div class="settings-section">
        <h4 class="section-title">📄 输出设置</h4>
        <el-form label-width="120px">
          <el-form-item label="默认输出方式">
            <el-radio-group v-model="outputSettings.defaultMode">
              <el-radio value="modify">直接修改</el-radio>
              <el-radio value="new">生成新文档</el-radio>
              <el-radio value="copy">生成副本</el-radio>
            </el-radio-group>
          </el-form-item>
          <el-form-item label="新文档命名">
            <el-input v-model="outputSettings.newFileName" placeholder="原文件名_优化" />
          </el-form-item>
          <el-form-item label="自动备份原文档">
            <el-switch v-model="outputSettings.autoBackup" />
          </el-form-item>
        </el-form>
      </div>
      
      <!-- 安全设置 -->
      <div class="settings-section">
        <h4 class="section-title">🔒 安全设置</h4>
        <el-form label-width="120px">
          <el-form-item label="敏感信息脱敏">
            <el-switch v-model="securitySettings.desensitize" />
            <div class="setting-hint">发送给AI前自动脱敏手机、身份证、金额等信息</div>
          </el-form-item>
          <el-form-item label="审计日志">
            <el-switch v-model="securitySettings.auditLog" />
            <div class="setting-hint">记录所有操作便于追溯</div>
          </el-form-item>
        </el-form>
      </div>
      
      <!-- Debug 设置 -->
      <div class="settings-section">
        <h4 class="section-title">🐛 调试设置</h4>
        <el-form label-width="120px">
          <el-form-item label="启用Debug模式">
            <el-switch v-model="debugSettings.enabled" />
            <div class="setting-hint">开启后将记录详细日志，便于问题排查</div>
          </el-form-item>
          
          <template v-if="debugSettings.enabled">
            <el-form-item label="日志级别">
              <el-select v-model="debugSettings.logLevel">
                <el-option label="INFO - 基本信息" value="info" />
                <el-option label="DEBUG - 调试信息" value="debug" />
                <el-option label="TRACE - 详细追踪" value="trace" />
              </el-select>
            </el-form-item>
            
            <el-form-item label="记录内容">
              <el-checkbox-group v-model="debugSettings.logItems">
                <el-checkbox value="desensitize">脱敏过程</el-checkbox>
                <el-checkbox value="apiRequest">API请求</el-checkbox>
                <el-checkbox value="apiResponse">API响应</el-checkbox>
                <el-checkbox value="fileOperation">文件操作</el-checkbox>
                <el-checkbox value="formatChange">格式修改</el-checkbox>
              </el-checkbox-group>
            </el-form-item>
            
            <el-form-item label="日志文件路径">
              <el-input v-model="debugSettings.logFilePath" disabled>
                <template #append>
                  <el-button @click="openLogDirectory">打开目录</el-button>
                </template>
              </el-input>
            </el-form-item>
            
            <el-form-item label="日志统计">
              <div class="log-stats" v-if="logStats">
                <span>文件数: {{ logStats.file_count }}</span>
                <span>总大小: {{ formatSize(logStats.total_size) }}</span>
                <span v-if="logStats.earliest_date">
                  日期范围: {{ logStats.earliest_date }} ~ {{ logStats.latest_date }}
                </span>
              </div>
            </el-form-item>
            
            <el-form-item>
              <el-button @click="viewLogFile" :disabled="!debugSettings.enabled">
                <el-icon><View /></el-icon>
                查看日志
              </el-button>
              <el-button @click="clearLogFile" :disabled="!debugSettings.enabled">
                <el-icon><Delete /></el-icon>
                清空日志
              </el-button>
            </el-form-item>
          </template>
        </el-form>
      </div>
      
      <!-- 界面设置 -->
      <div class="settings-section">
        <h4 class="section-title">🎨 界面设置</h4>
        <el-form label-width="100px">
          <el-form-item label="主题">
            <el-radio-group v-model="uiSettings.theme">
              <el-radio value="light">浅色</el-radio>
              <el-radio value="dark">深色</el-radio>
              <el-radio value="system">跟随系统</el-radio>
            </el-radio-group>
          </el-form-item>
          <el-form-item label="语言">
            <el-select v-model="uiSettings.language">
              <el-option label="简体中文" value="zh-CN" />
              <el-option label="English" value="en-US" disabled />
            </el-select>
          </el-form-item>
        </el-form>
      </div>
      
      <!-- 错误日志上报 -->
      <div class="settings-section">
        <h4 class="section-title">📊 错误日志上报</h4>
        <el-alert
          type="info"
          :closable="false"
          style="margin-bottom: 16px"
        >
          启用后，应用遇到错误时会自动将日志发送到服务器，帮助开发者快速定位和修复问题。
        </el-alert>
        <el-form label-width="120px">
          <el-form-item label="启用上报">
            <el-switch v-model="settingsStore.errorReport.enabled" />
            <div class="setting-hint">启用后，遇到错误时自动发送日志</div>
          </el-form-item>
          
          <el-form-item label="服务器地址" v-if="settingsStore.errorReport.enabled">
            <el-input 
              v-model="settingsStore.errorReport.serverUrl" 
              placeholder="错误日志服务器地址"
            />
            <div class="setting-hint">接收错误日志的服务器地址</div>
          </el-form-item>
          
          <el-form-item v-if="settingsStore.errorReport.enabled">
            <el-button type="primary" @click="testErrorReportConnection" :loading="testingErrorReport">
              测试连接
            </el-button>
            <span v-if="errorReportStatus" :style="{ color: errorReportStatus === '成功' ? '#67C23A' : '#F56C6C', marginLeft: '10px' }">
              {{ errorReportStatus }}
            </span>
          </el-form-item>
        </el-form>
      </div>
      
      <!-- 操作按钮 -->
      <div class="action-bar">
        <el-button @click="resetSettings">重置默认</el-button>
        <el-button type="primary" @click="saveSettings">保存设置</el-button>
      </div>
    </div>
    
    <!-- 日志查看对话框 -->
    <el-dialog 
      v-model="logDialogVisible" 
      title="日志内容" 
      width="80%" 
      top="5vh"
    >
      <div class="log-content">
        <pre>{{ logContent }}</pre>
      </div>
      <template #footer>
        <el-button @click="logDialogVisible = false">关闭</el-button>
        <el-button @click="refreshLogFile">刷新</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, watch } from 'vue'
import { ElMessage } from 'element-plus'
import { invoke } from '@tauri-apps/api/core'
import { useSettingsStore } from '@/stores/settings'

const settingsStore = useSettingsStore()

const showApiKey = ref(false)
const testing = ref(false)
const logDialogVisible = ref(false)
const logContent = ref('')
const testingErrorReport = ref(false)
const errorReportStatus = ref('')

const apiConfig = reactive({
  provider: 'baidu',
  apiKey: '',
  model: 'ERNIE-3.5-8K',
  enableBackup: false,
  backupProvider: 'deepseek',
  backupApiKey: '',
  backupModel: 'deepseek-chat'
})

const outputSettings = reactive({
  defaultMode: 'new' as 'modify' | 'new' | 'copy',
  newFileName: '原文件名_优化',
  autoBackup: true
})

const securitySettings = reactive({
  desensitize: true,
  auditLog: true
})

const debugSettings = reactive({
  enabled: false,
  logLevel: 'info',
  logItems: ['apiRequest', 'apiResponse', 'fileOperation'],
  logFilePath: ''
})

const uiSettings = reactive({
  theme: 'light' as 'light' | 'dark' | 'system',
  language: 'zh-CN'
})

const logStats = ref<{
  total_size: number
  file_count: number
  earliest_date?: string
  latest_date?: string
} | null>(null)

const availableModels = computed(() => {
  const models: Record<string, { label: string; value: string }[]> = {
    baidu: [
      { label: 'ERNIE-4.0-8K', value: 'ERNIE-4.0-8K' },
      { label: 'ERNIE-3.5-8K', value: 'ERNIE-3.5-8K' },
      { label: 'ERNIE-3.5-128K', value: 'ERNIE-3.5-128K' }
    ],
    aliyun: [
      { label: 'qwen-max', value: 'qwen-max' },
      { label: 'qwen-plus', value: 'qwen-plus' },
      { label: 'qwen-turbo', value: 'qwen-turbo' }
    ],
    openai: [
      { label: 'GPT-4', value: 'gpt-4' },
      { label: 'GPT-3.5-turbo', value: 'gpt-3.5-turbo' }
    ],
    deepseek: [
      { label: 'deepseek-chat', value: 'deepseek-chat' },
      { label: 'deepseek-coder', value: 'deepseek-coder' }
    ],
    custom: []
  }
  return models[apiConfig.provider] || []
})

const backupModels = computed(() => {
  const models: Record<string, { label: string; value: string }[]> = {
    baidu: [
      { label: 'ERNIE-4.0-8K', value: 'ERNIE-4.0-8K' },
      { label: 'ERNIE-3.5-8K', value: 'ERNIE-3.5-8K' }
    ],
    aliyun: [
      { label: 'qwen-max', value: 'qwen-max' },
      { label: 'qwen-plus', value: 'qwen-plus' }
    ],
    openai: [
      { label: 'GPT-4', value: 'gpt-4' },
      { label: 'GPT-3.5-turbo', value: 'gpt-3.5-turbo' }
    ],
    deepseek: [
      { label: 'deepseek-chat', value: 'deepseek-chat' },
      { label: 'deepseek-coder', value: 'deepseek-coder' }
    ]
  }
  return models[apiConfig.backupProvider] || []
})

function handleProviderChange() {
  apiConfig.model = availableModels.value[0]?.value || ''
}

function handleBackupProviderChange() {
  apiConfig.backupModel = backupModels.value[0]?.value || ''
}

async function testConnection() {
  if (!apiConfig.apiKey) {
    ElMessage.warning('请先输入 API Key')
    return
  }
  
  testing.value = true
  try {
    await invoke('test_api_connection', {
      provider: apiConfig.provider,
      apiKey: apiConfig.apiKey,
      model: apiConfig.model
    })
    ElMessage.success('连接成功')
  } catch (error) {
    ElMessage.error('连接失败: ' + error)
  } finally {
    testing.value = false
  }
}

// 测试错误日志服务器连接
async function testErrorReportConnection() {
  testingErrorReport.value = true
  errorReportStatus.value = ''
  
  try {
    const result = await settingsStore.testConnection()
    if (result.success) {
      errorReportStatus.value = '成功'
      ElMessage.success('连接测试成功')
    } else {
      errorReportStatus.value = '失败'
      ElMessage.error('连接失败: ' + result.message)
    }
  } catch (error) {
    errorReportStatus.value = '失败'
    ElMessage.error('连接失败: ' + error)
  } finally {
    testingErrorReport.value = false
  }
}

// 加载 Debug 配置
async function loadDebugConfig() {
  try {
    const config = await invoke<{
      enabled: boolean
      log_level: string
      log_items: string[]
      log_file_path: string
    }>('load_debug_config')
    
    debugSettings.enabled = config.enabled
    debugSettings.logLevel = config.log_level
    debugSettings.logItems = config.log_items
    debugSettings.logFilePath = config.log_file_path
    
    // 加载日志统计
    await loadLogStats()
  } catch (error) {
    console.error('加载 Debug 配置失败:', error)
  }
}

// 保存 Debug 配置
async function saveDebugConfig() {
  try {
    await invoke('save_debug_config', {
      config: {
        enabled: debugSettings.enabled,
        log_level: debugSettings.logLevel,
        log_items: debugSettings.logItems,
        log_file_path: debugSettings.logFilePath
      }
    })
  } catch (error) {
    console.error('保存 Debug 配置失败:', error)
  }
}

// 加载日志统计
async function loadLogStats() {
  try {
    logStats.value = await invoke('get_log_stats')
    if (logStats.value) {
      debugSettings.logFilePath = logStats.value.log_directory
    }
  } catch (error) {
    console.error('加载日志统计失败:', error)
  }
}

// 打开日志目录
async function openLogDirectory() {
  try {
    await invoke('open_log_directory')
  } catch (error) {
    ElMessage.error('打开目录失败: ' + error)
  }
}

// 查看日志
async function viewLogFile() {
  try {
    logContent.value = await invoke('read_log_file', { lines: 500 })
    logDialogVisible.value = true
  } catch (error) {
    ElMessage.error('读取日志失败: ' + error)
  }
}

// 刷新日志
async function refreshLogFile() {
  try {
    logContent.value = await invoke('read_log_file', { lines: 500 })
    ElMessage.success('日志已刷新')
  } catch (error) {
    ElMessage.error('刷新失败: ' + error)
  }
}

// 清空日志
async function clearLogFile() {
  try {
    await invoke('clear_log_file')
    ElMessage.success('日志已清空')
    await loadLogStats()
  } catch (error) {
    ElMessage.error('清空失败: ' + error)
  }
}

// 格式化文件大小
function formatSize(bytes: number): string {
  if (bytes < 1024) return bytes + ' B'
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(2) + ' KB'
  return (bytes / (1024 * 1024)).toFixed(2) + ' MB'
}

function resetSettings() {
  apiConfig.provider = 'baidu'
  apiConfig.apiKey = ''
  apiConfig.model = 'ERNIE-3.5-8K'
  outputSettings.defaultMode = 'new'
  outputSettings.newFileName = '原文件名_优化'
  outputSettings.autoBackup = true
  securitySettings.desensitize = true
  securitySettings.auditLog = true
  debugSettings.enabled = false
  debugSettings.logLevel = 'info'
  debugSettings.logItems = ['apiRequest', 'apiResponse', 'fileOperation']
  uiSettings.theme = 'light'
  uiSettings.language = 'zh-CN'
  ElMessage.success('已重置为默认设置')
}

async function saveSettings() {
  try {
    // 保存应用配置
    await invoke('save_config', {
      config: {
        api_provider: apiConfig.provider,
        api_key: apiConfig.apiKey,
        api_model: apiConfig.model,
        output_mode: outputSettings.defaultMode,
        auto_backup: outputSettings.autoBackup
      }
    })
    
    // 保存 Debug 配置
    await saveDebugConfig()
    
    ElMessage.success('设置已保存')
  } catch (error) {
    ElMessage.error('保存失败: ' + error)
  }
}

// 监听 debug 开关变化
watch(() => debugSettings.enabled, async (newVal) => {
  if (newVal) {
    await loadLogStats()
  }
})

onMounted(async () => {
  await loadDebugConfig()
})
</script>

<style scoped>
.settings-page {
  max-width: 600px;
  margin: 0 auto;
}

.settings-section {
  margin-bottom: 24px;
  padding-bottom: 24px;
  border-bottom: 1px solid #ebeef5;
}

.settings-section:last-of-type {
  border-bottom: none;
}

.section-title {
  font-size: 14px;
  font-weight: 600;
  color: #303133;
  margin-bottom: 16px;
}

.dark .section-title {
  color: #e5e7eb;
}

.setting-hint {
  font-size: 12px;
  color: #909399;
  margin-top: 4px;
}

.log-stats {
  display: flex;
  flex-wrap: wrap;
  gap: 16px;
  font-size: 13px;
  color: #606266;
}

.dark .log-stats {
  color: #a0aec0;
}

.log-content {
  max-height: 60vh;
  overflow: auto;
  background: #f5f7fa;
  border-radius: 6px;
  padding: 12px;
}

.dark .log-content {
  background: #1e3a5f;
}

.log-content pre {
  margin: 0;
  font-family: 'Consolas', 'Monaco', monospace;
  font-size: 12px;
  line-height: 1.5;
  white-space: pre-wrap;
  word-break: break-all;
}

.action-bar {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  margin-top: 24px;
}
</style>
