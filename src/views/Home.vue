<template>
  <div class="home-page">
    <el-row :gutter="20">
      <!-- 第一步：获取格式要求 -->
      <el-col :span="24">
        <div class="format-card">
          <h3 class="section-title">第一步：获取格式要求</h3>
          
          <el-tabs v-model="sourceType" @tab-change="handleSourceChange">
            <el-tab-pane label="📁 导入招标文件" name="tender">
              <div 
                class="upload-area"
                @click="selectTenderFile"
                @dragover.prevent
                @drop.prevent="handleTenderDrop"
              >
                <el-icon :size="48" color="#409eff"><Upload /></el-icon>
                <p class="upload-text">拖拽招标文件到这里，或点击选择</p>
                <p class="upload-hint">支持: .docx .doc .pdf</p>
              </div>
              <div v-if="tenderDocument" class="file-info">
                <el-icon><Document /></el-icon>
                <span>{{ tenderDocument }}</span>
                <el-button type="danger" text @click="clearTender">
                  <el-icon><Close /></el-icon>
                </el-button>
              </div>
            </el-tab-pane>
            
            <el-tab-pane label="📋 选择已保存模板" name="template">
              <el-select 
                v-model="selectedTemplate" 
                placeholder="选择格式模板"
                style="width: 100%"
                size="large"
              >
                <el-option 
                  v-for="tpl in presetTemplates" 
                  :key="tpl.id"
                  :label="tpl.name"
                  :value="tpl.id"
                >
                  <div class="template-option">
                    <span>{{ tpl.name }}</span>
                    <span class="template-desc">{{ tpl.description }}</span>
                  </div>
                </el-option>
              </el-select>
              
              <div v-if="selectedTemplate" class="template-preview">
                <el-descriptions :column="2" border size="small">
                  <el-descriptions-item label="字体">{{ getSelectedTemplate?.format.bodyFont }}</el-descriptions-item>
                  <el-descriptions-item label="字号">{{ getSelectedTemplate?.format.bodyFontSize }}</el-descriptions-item>
                  <el-descriptions-item label="页边距">{{ getSelectedTemplate?.format.marginTop }}cm</el-descriptions-item>
                  <el-descriptions-item label="行距">{{ getSelectedTemplate?.format.bodyLineHeight }}</el-descriptions-item>
                </el-descriptions>
              </div>
            </el-tab-pane>
          </el-tabs>
        </div>
      </el-col>
      
      <!-- 第二步：导入待优化标书 -->
      <el-col :span="24">
        <div class="format-card">
          <h3 class="section-title">第二步：导入待优化标书</h3>
          
          <div 
            class="upload-area"
            @click="selectBidFile"
            @dragover.prevent
            @drop.prevent="handleBidDrop"
          >
            <el-icon :size="48" color="#67c23a"><Document /></el-icon>
            <p class="upload-text">拖拽标书文件到这里，或点击选择</p>
            <p class="upload-hint">支持: .docx .doc .wps</p>
          </div>
          
          <div v-if="currentDocument" class="file-info">
            <el-icon><Document /></el-icon>
            <span>{{ currentDocument }}</span>
            <el-button type="danger" text @click="clearDocument">
              <el-icon><Close /></el-icon>
            </el-button>
          </div>
        </div>
      </el-col>
      
      <!-- 操作按钮 -->
      <el-col :span="24">
        <div class="action-bar">
          <el-button 
            type="primary" 
            size="large"
            :disabled="!canProceed"
            @click="goToConfirm"
          >
            下一步：确认格式要求
            <el-icon class="el-icon--right"><ArrowRight /></el-icon>
          </el-button>
        </div>
      </el-col>
    </el-row>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage } from 'element-plus'
import { open } from '@tauri-apps/plugin-dialog'
import { useFormatStore, type FormatTemplate } from '../stores/format'

const router = useRouter()
const store = useFormatStore()

const sourceType = ref<'tender' | 'template'>('tender')
const tenderDocument = ref('')
const currentDocument = ref('')
const selectedTemplate = ref('')

// 预设模板
const presetTemplates = ref<FormatTemplate[]>([
  {
    id: 'gov-procurement',
    name: '政府采购通用模板',
    description: '字体: 宋体小四 | 页边距: 2.54/3.17cm | 行距: 1.5倍',
    format: {
      paperSize: 'A4',
      marginTop: 2.54,
      marginBottom: 2.54,
      marginLeft: 3.17,
      marginRight: 3.17,
      gutter: 0,
      gutterPosition: 'left',
      orientation: 'portrait',
      bodyFont: '宋体',
      bodyFontSize: '小四',
      bodyLineHeight: '1.5倍',
      bodyParagraphSpacing: { before: 0, after: 0 },
      heading1: { font: '黑体', fontSize: '三号', spacing: { before: 0.5, after: 0.5 } },
      heading2: { font: '黑体', fontSize: '四号', spacing: { before: 0.5, after: 0.5 } },
      heading3: { font: '黑体', fontSize: '小四', spacing: { before: 0, after: 0 } },
      heading4: { font: '黑体', fontSize: '小四', spacing: { before: 0, after: 0 } },
      header: { content: '项目名称', font: '宋体', fontSize: '小五' },
      footer: { position: '居中', format: '第X页 共X页' },
      tableFont: '宋体',
      tableFontSize: '小四',
      tableBorder: '单线',
      tableAlign: '居中'
    },
    isPreset: true,
    createdAt: '2026-01-01'
  },
  {
    id: 'enterprise-standard',
    name: '央企招标标准模板',
    description: '字体: 宋体四号 | 页边距: 2.5cm | 行距: 1.25倍',
    format: {
      paperSize: 'A4',
      marginTop: 2.5,
      marginBottom: 2.5,
      marginLeft: 2.5,
      marginRight: 2.5,
      gutter: 0,
      gutterPosition: 'left',
      orientation: 'portrait',
      bodyFont: '宋体',
      bodyFontSize: '四号',
      bodyLineHeight: '1.25倍',
      bodyParagraphSpacing: { before: 0, after: 0 },
      heading1: { font: '黑体', fontSize: '二号', spacing: { before: 1, after: 0.5 } },
      heading2: { font: '黑体', fontSize: '三号', spacing: { before: 0.5, after: 0.5 } },
      heading3: { font: '黑体', fontSize: '四号', spacing: { before: 0.5, after: 0 } },
      heading4: { font: '黑体', fontSize: '小四', spacing: { before: 0, after: 0 } },
      header: { content: '', font: '宋体', fontSize: '小五' },
      footer: { position: '居中', format: '第X页' },
      tableFont: '宋体',
      tableFontSize: '五号',
      tableBorder: '单线',
      tableAlign: '居中'
    },
    isPreset: true,
    createdAt: '2026-01-01'
  },
  {
    id: 'state-owned',
    name: '国企投标格式模板',
    description: '字体: 仿宋四号 | 页边距: 2.8cm | 行距: 1.5倍',
    format: {
      paperSize: 'A4',
      marginTop: 2.8,
      marginBottom: 2.8,
      marginLeft: 2.8,
      marginRight: 2.8,
      gutter: 0,
      gutterPosition: 'left',
      orientation: 'portrait',
      bodyFont: '仿宋',
      bodyFontSize: '四号',
      bodyLineHeight: '1.5倍',
      bodyParagraphSpacing: { before: 0, after: 0 },
      heading1: { font: '黑体', fontSize: '二号', spacing: { before: 1, after: 0.5 } },
      heading2: { font: '黑体', fontSize: '三号', spacing: { before: 0.5, after: 0.5 } },
      heading3: { font: '楷体', fontSize: '四号', spacing: { before: 0.5, after: 0 } },
      heading4: { font: '楷体', fontSize: '小四', spacing: { before: 0, after: 0 } },
      header: { content: '项目名称', font: '仿宋', fontSize: '小四' },
      footer: { position: '居中', format: '第X页 共X页' },
      tableFont: '仿宋',
      tableFontSize: '小四',
      tableBorder: '单线',
      tableAlign: '居中'
    },
    isPreset: true,
    createdAt: '2026-01-01'
  }
])

const getSelectedTemplate = computed(() => {
  return presetTemplates.value.find(t => t.id === selectedTemplate.value)
})

const canProceed = computed(() => {
  if (sourceType.value === 'tender') {
    return tenderDocument.value && currentDocument.value
  } else {
    return selectedTemplate.value && currentDocument.value
  }
})

function handleSourceChange() {
  tenderDocument.value = ''
  selectedTemplate.value = ''
}

async function selectTenderFile() {
  const selected = await open({
    multiple: false,
    filters: [{ name: '招标文件', extensions: ['docx', 'doc', 'pdf'] }]
  })
  if (selected) {
    tenderDocument.value = selected as string
    store.setTenderDocument(selected as string)
    ElMessage.success('已选择招标文件')
  }
}

function handleTenderDrop(e: DragEvent) {
  const files = e.dataTransfer?.files
  if (files && files.length > 0) {
    const file = files[0]
    if (['.docx', '.doc', '.pdf'].some(ext => file.name.endsWith(ext))) {
      tenderDocument.value = file.path || file.name
      ElMessage.success('已选择招标文件')
    } else {
      ElMessage.warning('请选择 .docx .doc .pdf 格式的文件')
    }
  }
}

async function selectBidFile() {
  const selected = await open({
    multiple: false,
    filters: [{ name: '投标文件', extensions: ['docx', 'doc', 'wps'] }]
  })
  if (selected) {
    currentDocument.value = selected as string
    store.setCurrentDocument(selected as string)
    ElMessage.success('已选择投标文件')
  }
}

function handleBidDrop(e: DragEvent) {
  const files = e.dataTransfer?.files
  if (files && files.length > 0) {
    const file = files[0]
    if (['.docx', '.doc', '.wps'].some(ext => file.name.endsWith(ext))) {
      currentDocument.value = file.path || file.name
      store.setCurrentDocument(file.path || file.name)
      ElMessage.success('已选择投标文件')
    } else {
      ElMessage.warning('请选择 .docx .doc .wps 格式的文件')
    }
  }
}

function clearTender() {
  tenderDocument.value = ''
  store.setTenderDocument('')
}

function clearDocument() {
  currentDocument.value = ''
  store.setCurrentDocument('')
}

async function goToConfirm() {
  // 如果是招标文件模式，需要调用后端提取格式
  if (sourceType.value === 'tender') {
    // TODO: 调用后端 API 提取格式要求
    ElMessage.info('正在提取格式要求...')
  } else {
    // 使用模板
    if (getSelectedTemplate.value) {
      store.setFormatRequirement(getSelectedTemplate.value.format)
    }
  }
  
  router.push('/format-confirm')
}
</script>

<style scoped>
.home-page {
  max-width: 800px;
  margin: 0 auto;
}

.section-title {
  font-size: 14px;
  font-weight: 600;
  color: #303133;
  margin-bottom: 12px;
}

.dark .section-title {
  color: #e5e7eb;
}

.upload-area {
  border: 2px dashed #dcdfe6;
  border-radius: 8px;
  padding: 32px;
  text-align: center;
  cursor: pointer;
  transition: all 0.3s;
}

.upload-area:hover {
  border-color: #409eff;
  background: #f5f7fa;
}

.dark .upload-area {
  border-color: #2d3748;
}

.dark .upload-area:hover {
  border-color: #409eff;
  background: #1e3a5f;
}

.upload-text {
  margin-top: 16px;
  font-size: 13px;
  color: #606266;
}

.dark .upload-text {
  color: #9ca3af;
}

.upload-hint {
  margin-top: 8px;
  font-size: 11px;
  color: #909399;
}

.file-info {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 16px;
  background: #f5f7fa;
  border-radius: 6px;
  margin-top: 12px;
}

.dark .file-info {
  background: #1e3a5f;
}

.template-option {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.template-desc {
  font-size: 12px;
  color: #909399;
}

.template-preview {
  margin-top: 16px;
}

.action-bar {
  text-align: center;
  margin-top: 16px;
}
</style>
