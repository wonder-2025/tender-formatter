<template>
  <div class="home-page">
    <!-- 标题区 -->
    <div class="header">
      <div class="title-area">
        <el-icon :size="28" color="#409EFF"><Document /></el-icon>
        <div>
          <h1>标书格式优化工具</h1>
          <p>自动提取格式要求 · 一键优化标书</p>
        </div>
      </div>
    </div>

    <!-- 两步操作区 -->
    <div class="steps-container">
      <!-- 第一步 -->
      <div class="step-card">
        <div class="step-header">
          <span class="step-badge">1</span>
          <span class="step-title">获取格式要求</span>
        </div>
        <el-tabs v-model="sourceType" size="small">
          <el-tab-pane label="导入招标文件" name="tender">
            <div class="upload-box" @click="selectTenderFile" @dragover.prevent @drop.prevent="handleTenderDrop">
              <el-icon :size="32" color="#409EFF"><Upload /></el-icon>
              <p>拖拽或点击上传</p>
              <span>支持: .docx .doc .pdf</span>
            </div>
            <div v-if="tenderDocument" class="file-tag">
              <el-icon><Document /></el-icon>
              <span>{{ getFileName(tenderDocument) }}</span>
              <el-button type="danger" text size="small" @click.stop="clearTender">×</el-button>
            </div>
          </el-tab-pane>
          <el-tab-pane label="选择模板" name="template">
            <el-select v-model="selectedTemplate" placeholder="选择预设模板" style="width:100%" size="default">
              <el-option v-for="t in presetTemplates" :key="t.id" :label="t.name" :value="t.id">
                <span>{{ t.name }}</span>
                <span style="color:#909399;font-size:11px;margin-left:8px">{{ t.description }}</span>
              </el-option>
            </el-select>
          </el-tab-pane>
        </el-tabs>
      </div>

      <!-- 箭头 -->
      <el-icon :size="24" color="#C0C4CC"><Right /></el-icon>

      <!-- 第二步 -->
      <div class="step-card">
        <div class="step-header">
          <span class="step-badge">2</span>
          <span class="step-title">导入待优化标书</span>
        </div>
        <div class="upload-box" @click="selectBidFile" @dragover.prevent @drop.prevent="handleBidDrop">
          <el-icon :size="32" color="#67C23A"><Document /></el-icon>
          <p>拖拽或点击上传</p>
          <span>支持: .docx .doc .wps</span>
        </div>
        <div v-if="currentDocument" class="file-tag">
          <el-icon><Document /></el-icon>
          <span>{{ getFileName(currentDocument) }}</span>
          <el-button type="danger" text size="small" @click.stop="clearDocument">×</el-button>
        </div>
      </div>
    </div>

    <!-- 操作按钮 -->
    <div class="action-bar">
      <el-button type="primary" size="large" :disabled="!canProceed" @click="goToConfirm">
        <el-icon><ArrowRight /></el-icon>下一步：确认格式要求
      </el-button>
    </div>
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

const presetTemplates = ref<FormatTemplate[]>([
  {
    id: 'gov-procurement',
    name: '政府采购通用模板',
    description: '宋体小四 | 页边距2.54cm | 行距1.5倍',
    format: {
      paperSize: 'A4', marginTop: 2.54, marginBottom: 2.54, marginLeft: 3.17, marginRight: 3.17,
      gutter: 0, gutterPosition: 'left', orientation: 'portrait',
      bodyFont: '宋体', bodyFontSize: '小四', bodyLineHeight: '1.5倍',
      bodyParagraphSpacing: { before: 0, after: 0 },
      heading1: { font: '黑体', fontSize: '三号', spacing: { before: 0.5, after: 0.5 } },
      heading2: { font: '黑体', fontSize: '四号', spacing: { before: 0.5, after: 0.5 } },
      heading3: { font: '黑体', fontSize: '小四', spacing: { before: 0, after: 0 } },
      heading4: { font: '黑体', fontSize: '小四', spacing: { before: 0, after: 0 } },
      header: { content: '项目名称', font: '宋体', fontSize: '小五' },
      footer: { position: '居中', format: '第X页 共X页' },
      tableFont: '宋体', tableFontSize: '小四', tableBorder: '单线', tableAlign: '居中'
    },
    isPreset: true, createdAt: '2026-01-01'
  },
  {
    id: 'enterprise-standard',
    name: '央企招标标准模板',
    description: '宋体四号 | 页边距2.5cm | 行距1.25倍',
    format: {
      paperSize: 'A4', marginTop: 2.5, marginBottom: 2.5, marginLeft: 2.5, marginRight: 2.5,
      gutter: 0, gutterPosition: 'left', orientation: 'portrait',
      bodyFont: '宋体', bodyFontSize: '四号', bodyLineHeight: '1.25倍',
      bodyParagraphSpacing: { before: 0, after: 0 },
      heading1: { font: '黑体', fontSize: '二号', spacing: { before: 1, after: 0.5 } },
      heading2: { font: '黑体', fontSize: '三号', spacing: { before: 0.5, after: 0.5 } },
      heading3: { font: '黑体', fontSize: '四号', spacing: { before: 0.5, after: 0 } },
      heading4: { font: '黑体', fontSize: '小四', spacing: { before: 0, after: 0 } },
      header: { content: '', font: '宋体', fontSize: '小五' },
      footer: { position: '居中', format: '第X页' },
      tableFont: '宋体', tableFontSize: '五号', tableBorder: '单线', tableAlign: '居中'
    },
    isPreset: true, createdAt: '2026-01-01'
  },
  {
    id: 'state-owned',
    name: '国企投标格式模板',
    description: '仿宋四号 | 页边距2.8cm | 行距1.5倍',
    format: {
      paperSize: 'A4', marginTop: 2.8, marginBottom: 2.8, marginLeft: 2.8, marginRight: 2.8,
      gutter: 0, gutterPosition: 'left', orientation: 'portrait',
      bodyFont: '仿宋', bodyFontSize: '四号', bodyLineHeight: '1.5倍',
      bodyParagraphSpacing: { before: 0, after: 0 },
      heading1: { font: '黑体', fontSize: '二号', spacing: { before: 1, after: 0.5 } },
      heading2: { font: '黑体', fontSize: '三号', spacing: { before: 0.5, after: 0.5 } },
      heading3: { font: '楷体', fontSize: '四号', spacing: { before: 0.5, after: 0 } },
      heading4: { font: '楷体', fontSize: '小四', spacing: { before: 0, after: 0 } },
      header: { content: '项目名称', font: '仿宋', fontSize: '小四' },
      footer: { position: '居中', format: '第X页 共X页' },
      tableFont: '仿宋', tableFontSize: '小四', tableBorder: '单线', tableAlign: '居中'
    },
    isPreset: true, createdAt: '2026-01-01'
  }
])

const getSelectedTemplate = computed(() => presetTemplates.value.find(t => t.id === selectedTemplate.value))

const canProceed = computed(() => {
  if (sourceType.value === 'tender') return tenderDocument.value && currentDocument.value
  return selectedTemplate.value && currentDocument.value
})

function getFileName(path: string) {
  return path.split(/[/\\]/).pop() || path
}

async function selectTenderFile() {
  const selected = await open({ multiple: false, filters: [{ name: '招标文件', extensions: ['docx', 'doc', 'pdf'] }] })
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
      tenderDocument.value = (file as any).path || file.name
      ElMessage.success('已选择招标文件')
    } else {
      ElMessage.warning('请选择 .docx .doc .pdf 格式')
    }
  }
}

async function selectBidFile() {
  const selected = await open({ multiple: false, filters: [{ name: '投标文件', extensions: ['docx', 'doc', 'wps'] }] })
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
      currentDocument.value = (file as any).path || file.name
      store.setCurrentDocument((file as any).path || file.name)
      ElMessage.success('已选择投标文件')
    } else {
      ElMessage.warning('请选择 .docx .doc .wps 格式')
    }
  }
}

function clearTender() { tenderDocument.value = ''; store.setTenderDocument('') }
function clearDocument() { currentDocument.value = ''; store.setCurrentDocument('') }

async function goToConfirm() {
  if (sourceType.value === 'tender') {
    ElMessage.info('正在提取格式要求...')
  } else if (getSelectedTemplate.value) {
    store.setFormatRequirement(getSelectedTemplate.value.format)
  }
  router.push('/format-confirm')
}
</script>

<style scoped>
.home-page {
  height: 100%;
  display: flex;
  flex-direction: column;
  padding: 12px;
  overflow: hidden;
}

.header {
  margin-bottom: 12px;
}

.title-area {
  display: flex;
  align-items: center;
  gap: 12px;
}

.title-area h1 {
  font-size: 18px;
  font-weight: 700;
  margin: 0;
}

.title-area p {
  font-size: 11px;
  color: #909399;
  margin: 2px 0 0;
}

.steps-container {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 12px;
  min-height: 0;
}

.step-card {
  flex: 1;
  background: #fff;
  border-radius: 8px;
  padding: 12px;
  box-shadow: 0 1px 4px rgba(0,0,0,0.05);
  display: flex;
  flex-direction: column;
}

.step-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 10px;
}

.step-badge {
  width: 22px;
  height: 22px;
  background: linear-gradient(135deg, #409EFF, #36D1DC);
  color: #fff;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 12px;
  font-weight: 600;
}

.step-title {
  font-size: 13px;
  font-weight: 600;
  color: #303133;
}

.upload-box {
  border: 2px dashed #dcdfe6;
  border-radius: 6px;
  padding: 20px;
  text-align: center;
  cursor: pointer;
  transition: all 0.2s;
}

.upload-box:hover {
  border-color: #409EFF;
  background: #f5f7fa;
}

.upload-box p {
  margin: 8px 0 4px;
  font-size: 12px;
  color: #606266;
}

.upload-box span {
  font-size: 10px;
  color: #909399;
}

.file-tag {
  display: flex;
  align-items: center;
  gap: 6px;
  margin-top: 8px;
  padding: 6px 10px;
  background: #f0f9eb;
  border-radius: 4px;
  font-size: 11px;
}

.action-bar {
  text-align: center;
  margin-top: 12px;
}
</style>
