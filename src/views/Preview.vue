<template>
  <div class="preview-page">
    <div class="format-card">
      <div class="card-header">
        <h3>格式优化预览</h3>
        <div class="doc-info">
          <span>当前文档: {{ currentDocument }}</span>
        </div>
      </div>
      
      <!-- 差异列表 -->
      <div class="diff-section">
        <h4 class="section-title">
          格式差异列表 
          <el-tag v-if="diffs.length > 0" type="warning" size="small">
            共 {{ diffs.length }} 项需要调整
          </el-tag>
          <el-tag v-else type="success" size="small">格式已符合要求</el-tag>
        </h4>
        
        <div v-if="diffs.length > 0" class="diff-list">
          <div v-for="(diff, index) in diffs" :key="index" class="diff-item">
            <el-tag :type="getCategoryTagType(diff.category)" size="small">
              {{ diff.category }}
            </el-tag>
            <span class="diff-name">{{ diff.display_name || diff.name }}:</span>
            <span class="diff-current">{{ diff.current_value || diff.current }}</span>
            <el-icon class="diff-arrow"><Right /></el-icon>
            <span class="diff-target">{{ diff.target_value || diff.target }}</span>
          </div>
        </div>
        
        <el-empty v-else description="文档格式已符合要求，无需调整" :image-size="100" />
      </div>
      
      <!-- 预览对比 -->
      <div class="preview-section">
        <h4 class="section-title">预览对比</h4>
        
        <div class="preview-container">
          <div class="preview-pane">
            <div class="preview-header">原始文档</div>
            <div class="preview-content">
              <div class="preview-placeholder">
                <el-icon :size="48"><Document /></el-icon>
                <p>文档预览区域</p>
                <p class="hint">{{ currentDocument || '未选择文档' }}</p>
              </div>
            </div>
          </div>
          
          <div class="preview-pane">
            <div class="preview-header">优化后效果</div>
            <div class="preview-content">
              <div class="preview-placeholder optimized">
                <el-icon :size="48"><Document /></el-icon>
                <p>优化后预览</p>
                <p class="hint">显示应用格式后的效果</p>
              </div>
            </div>
          </div>
        </div>
      </div>
      
      <!-- 输出选项 -->
      <div class="output-section">
        <h4 class="section-title">输出方式</h4>
        <el-radio-group v-model="outputMode">
          <el-radio value="modify">
            <span>直接修改原文档</span>
            <el-tag size="small" type="danger" class="mode-tag">风险操作</el-tag>
          </el-radio>
          <el-radio value="new">
            <span>生成新文档</span>
            <el-tag size="small" type="success" class="mode-tag">推荐</el-tag>
          </el-radio>
          <el-radio value="copy">
            <span>生成副本</span>
          </el-radio>
        </el-radio-group>
        
        <div class="output-hint">
          <el-icon><InfoFilled /></el-icon>
          <span v-if="outputMode === 'modify'">将在原文档上直接修改，建议先备份</span>
          <span v-else-if="outputMode === 'new'">生成新文档: {{ newFileName }}</span>
          <span v-else>在原目录生成副本: {{ copyFileName }}</span>
        </div>
      </div>
      
      <!-- 操作按钮 -->
      <div class="action-bar">
        <el-button @click="router.back()">返回修改</el-button>
        <el-button type="danger" @click="cancel">
          <el-icon><Close /></el-icon>
          取消
        </el-button>
        <el-button 
          type="primary" 
          @click="executeOptimize" 
          :loading="optimizing"
          :disabled="diffs.length === 0"
        >
          <el-icon><Check /></el-icon>
          执行优化
        </el-button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'
import { useFormatStore } from '../stores/format'
import { invoke } from '@tauri-apps/api/core'

const router = useRouter()
const store = useFormatStore()

const outputMode = ref<'modify' | 'new' | 'copy'>('new')
const optimizing = ref(false)
const diffs = ref<any[]>([])

const currentDocument = computed(() => {
  const path = store.currentDocument
  return path ? path.split('/').pop() || path : '未选择'
})

const newFileName = computed(() => {
  const name = currentDocument.value
  const ext = name.split('.').pop()
  const base = name.replace(`.${ext}`, '')
  return `${base}_优化.${ext}`
})

const copyFileName = computed(() => {
  const name = currentDocument.value
  const ext = name.split('.').pop()
  const base = name.replace(`.${ext}`, '')
  return `${base}_副本.${ext}`
})

function getCategoryTagType(category: string): '' | 'success' | 'warning' | 'info' | 'danger' {
  const types: Record<string, '' | 'success' | 'warning' | 'info' | 'danger'> = {
    '页面设置': 'primary',
    '字体格式': 'warning',
    '标题格式': 'success',
    '页眉页脚': 'info',
    '表格格式': ''
  }
  return types[category] || ''
}

function cancel() {
  ElMessageBox.confirm('确定要取消优化吗？', '提示', {
    confirmButtonText: '确定',
    cancelButtonText: '继续编辑',
    type: 'warning'
  }).then(() => {
    store.reset()
    router.push('/')
  }).catch(() => {})
}

// 分析文档格式差异
async function analyzeDocument() {
  if (!store.currentDocument) {
    return
  }
  
  try {
    // 分析当前文档格式
    const currentFormat = await invoke<any>('analyze_document_format', {
      filePath: store.currentDocument
    })
    
    // 获取目标格式
    const targetFormat = store.currentRequirement
    
    if (targetFormat) {
      // 比较格式差异
      const diffList = await invoke<any[]>('compare_format_diff', {
        current: currentFormat,
        target: targetFormat
      })
      
      diffs.value = diffList
      store.setFormatDiffs(diffList)
    }
  } catch (error) {
    console.error('分析文档失败:', error)
    ElMessage.warning('文档分析失败，使用模拟数据')
    
    // 使用模拟数据
    diffs.value = store.formatDiffs.length > 0 ? store.formatDiffs : [
      { category: '页面设置', name: '页边距-上', current: '2cm', target: '2.54cm', display_name: '页边距-上', current_value: '2cm', target_value: '2.54cm' },
      { category: '页面设置', name: '页边距-下', current: '2cm', target: '2.54cm', display_name: '页边距-下', current_value: '2cm', target_value: '2.54cm' },
      { category: '页面设置', name: '页边距-左', current: '2.5cm', target: '3.17cm', display_name: '页边距-左', current_value: '2.5cm', target_value: '3.17cm' },
      { category: '字体格式', name: '正文字号', current: '五号', target: '小四', display_name: '正文字号', current_value: '五号', target_value: '小四' },
      { category: '字体格式', name: '行间距', current: '1.0倍', target: '1.5倍', display_name: '行间距', current_value: '1.0倍', target_value: '1.5倍' }
    ]
  }
}

async function executeOptimize() {
  if (!store.currentDocument || !store.currentRequirement) {
    ElMessage.warning('请先选择文档和格式要求')
    return
  }
  
  if (diffs.value.length === 0) {
    ElMessage.info('文档格式已符合要求，无需优化')
    return
  }
  
  // 确认操作
  if (outputMode.value === 'modify') {
    try {
      await ElMessageBox.confirm(
        '直接修改原文档可能导致数据丢失，确定要继续吗？',
        '风险提示',
        {
          confirmButtonText: '确定修改',
          cancelButtonText: '取消',
          type: 'warning'
        }
      )
    } catch {
      return
    }
  }
  
  optimizing.value = true
  
  try {
    const result = await invoke<string>('apply_format', {
      filePath: store.currentDocument,
      format: store.currentRequirement,
      outputMode: outputMode.value,
      backup: true
    })
    
    ElMessage.success(`格式优化完成！输出文件: ${result}`)
    store.reset()
    router.push('/')
  } catch (error) {
    ElMessage.error('优化失败: ' + error)
  } finally {
    optimizing.value = false
  }
}

onMounted(() => {
  analyzeDocument()
})
</script>

<style scoped>
.preview-page {
  max-width: 1000px;
  margin: 0 auto;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.card-header h3 {
  font-size: 18px;
  font-weight: 600;
}

.doc-info {
  font-size: 12px;
  color: #909399;
}

.section-title {
  font-size: 14px;
  font-weight: 600;
  color: #606266;
  margin-bottom: 12px;
  padding-bottom: 8px;
  border-bottom: 1px solid #ebeef5;
  display: flex;
  align-items: center;
  gap: 8px;
}

.dark .section-title {
  color: #e5e7eb;
}

.diff-list {
  max-height: 250px;
  overflow-y: auto;
  background: #fafafa;
  border-radius: 6px;
  padding: 12px;
}

.dark .diff-list {
  background: #1e3a5f;
}

.diff-item {
  display: flex;
  align-items: center;
  padding: 8px 0;
  border-bottom: 1px solid #ebeef5;
  font-size: 14px;
  gap: 8px;
}

.diff-item:last-child {
  border-bottom: none;
}

.diff-name {
  color: #303133;
  min-width: 80px;
}

.dark .diff-name {
  color: #e5e7eb;
}

.diff-current {
  color: #f56c6c;
}

.diff-arrow {
  color: #409eff;
}

.diff-target {
  color: #67c23a;
  font-weight: 500;
}

.preview-section {
  margin-top: 24px;
}

.preview-container {
  display: flex;
  gap: 20px;
  height: 300px;
}

.preview-pane {
  flex: 1;
  border: 1px solid #dcdfe6;
  border-radius: 8px;
  overflow: hidden;
}

.dark .preview-pane {
  border-color: #2d3748;
}

.preview-header {
  background: #f5f7fa;
  padding: 12px 16px;
  font-weight: 600;
  text-align: center;
  border-bottom: 1px solid #dcdfe6;
}

.dark .preview-header {
  background: #1e3a5f;
  border-bottom-color: #2d3748;
}

.preview-content {
  height: calc(100% - 44px);
  overflow: auto;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #fff;
}

.dark .preview-content {
  background: #16213e;
}

.preview-placeholder {
  text-align: center;
  color: #c0c4cc;
}

.preview-placeholder.optimized {
  color: #67c23a;
}

.preview-placeholder .hint {
  font-size: 12px;
  margin-top: 8px;
}

.output-section {
  margin-top: 24px;
}

.output-section :deep(.el-radio) {
  display: flex;
  align-items: center;
  margin-bottom: 8px;
}

.mode-tag {
  margin-left: 8px;
}

.output-hint {
  display: flex;
  align-items: center;
  gap: 4px;
  margin-top: 12px;
  font-size: 12px;
  color: #909399;
}

.action-bar {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  margin-top: 24px;
}
</style>
