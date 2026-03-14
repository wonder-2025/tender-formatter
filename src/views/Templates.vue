<template>
  <div class="templates-page">
    <div class="format-card">
      <div class="card-header">
        <h3>格式模板管理</h3>
        <el-button type="primary" @click="createTemplate">
          <el-icon><Plus /></el-icon>
          新建模板
        </el-button>
      </div>
      
      <!-- 预设模板 -->
      <div class="template-section">
        <h4 class="section-title">
          <el-icon><Collection /></el-icon>
          预设模板
        </h4>
        <div class="template-list">
          <div 
            v-for="tpl in presetTemplates" 
            :key="tpl.id" 
            class="template-card preset"
          >
            <div class="template-info">
              <div class="template-name">
                <el-icon><Document /></el-icon>
                {{ tpl.name }}
              </div>
              <div class="template-desc">{{ tpl.description }}</div>
              <div class="template-tags">
                <el-tag size="small" v-for="tag in tpl.tags" :key="tag">{{ tag }}</el-tag>
              </div>
            </div>
            <div class="template-actions">
              <el-button size="small" @click="viewTemplate(tpl)">查看</el-button>
              <el-button size="small" type="primary" @click="useTemplate(tpl)">使用</el-button>
            </div>
          </div>
        </div>
      </div>
      
      <!-- 自定义模板 -->
      <div class="template-section">
        <h4 class="section-title">
          <el-icon><FolderOpened /></el-icon>
          自定义模板
        </h4>
        <div v-if="customTemplates.length === 0" class="empty-tip">
          <el-empty description="暂无自定义模板，可以从格式确认页保存" :image-size="80" />
        </div>
        <div v-else class="template-list">
          <div 
            v-for="tpl in customTemplates" 
            :key="tpl.id" 
            class="template-card custom"
          >
            <div class="template-info">
              <div class="template-name">
                <el-icon><EditPen /></el-icon>
                {{ tpl.name }}
              </div>
              <div class="template-desc">{{ tpl.description }}</div>
              <div class="template-meta">
                创建: {{ tpl.createdAt }}
                <span v-if="tpl.lastUsedAt"> | 最后使用: {{ tpl.lastUsedAt }}</span>
              </div>
            </div>
            <div class="template-actions">
              <el-button size="small" @click="editTemplate(tpl)">编辑</el-button>
              <el-button size="small" @click="duplicateTemplate(tpl)">复制</el-button>
              <el-button size="small" type="danger" @click="deleteTemplate(tpl)">删除</el-button>
            </div>
          </div>
        </div>
      </div>
      
      <!-- 导入导出 -->
      <div class="import-export">
        <el-button @click="importTemplate">
          <el-icon><Download /></el-icon>
          导入模板
        </el-button>
        <el-button @click="exportTemplates" :disabled="customTemplates.length === 0">
          <el-icon><Upload /></el-icon>
          导出模板
        </el-button>
      </div>
    </div>
    
    <!-- 模板详情对话框 -->
    <el-dialog v-model="detailDialogVisible" :title="currentTemplate?.name" width="600px">
      <el-descriptions :column="2" border v-if="currentTemplate">
        <el-descriptions-item label="纸张大小">{{ currentTemplate.format?.paperSize || 'A4' }}</el-descriptions-item>
        <el-descriptions-item label="页方向">{{ currentTemplate.format?.orientation === 'portrait' ? '纵向' : '横向' }}</el-descriptions-item>
        <el-descriptions-item label="页边距-上">{{ currentTemplate.format?.marginTop }} cm</el-descriptions-item>
        <el-descriptions-item label="页边距-下">{{ currentTemplate.format?.marginBottom }} cm</el-descriptions-item>
        <el-descriptions-item label="页边距-左">{{ currentTemplate.format?.marginLeft }} cm</el-descriptions-item>
        <el-descriptions-item label="页边距-右">{{ currentTemplate.format?.marginRight }} cm</el-descriptions-item>
        <el-descriptions-item label="正文字体">{{ currentTemplate.format?.bodyFont }}</el-descriptions-item>
        <el-descriptions-item label="正文字号">{{ currentTemplate.format?.bodyFontSize }}</el-descriptions-item>
        <el-descriptions-item label="行间距">{{ currentTemplate.format?.bodyLineHeight }}</el-descriptions-item>
        <el-descriptions-item label="一级标题">{{ currentTemplate.format?.heading1?.font }} {{ currentTemplate.format?.heading1?.fontSize }}</el-descriptions-item>
      </el-descriptions>
      <template #footer>
        <el-button @click="detailDialogVisible = false">关闭</el-button>
        <el-button type="primary" @click="useCurrentTemplate">使用此模板</el-button>
      </template>
    </el-dialog>
    
    <!-- 新建/编辑模板对话框 -->
    <el-dialog v-model="editDialogVisible" :title="editingTemplate ? '编辑模板' : '新建模板'" width="500px">
      <el-form :model="templateForm" label-width="80px">
        <el-form-item label="模板名称" required>
          <el-input v-model="templateForm.name" placeholder="输入模板名称" />
        </el-form-item>
        <el-form-item label="描述">
          <el-input v-model="templateForm.description" type="textarea" rows="2" placeholder="简要描述模板特点" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="editDialogVisible = false">取消</el-button>
        <el-button type="primary" @click="saveTemplateForm">保存</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'
import { useFormatStore, type FormatTemplate } from '../stores/format'
import { invoke } from '@tauri-apps/api/core'

const router = useRouter()
const store = useFormatStore()

const detailDialogVisible = ref(false)
const editDialogVisible = ref(false)
const currentTemplate = ref<FormatTemplate | null>(null)
const editingTemplate = ref<FormatTemplate | null>(null)

const templateForm = ref({
  name: '',
  description: ''
})

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
    createdAt: '2026-01-01',
    tags: ['政府采购', '通用']
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
      heading1: { font: '黑体', fontSize: '三号', spacing: { before: 0.5, after: 0.5 } },
      heading2: { font: '黑体', fontSize: '四号', spacing: { before: 0.5, after: 0.5 } },
      heading3: { font: '黑体', fontSize: '小四', spacing: { before: 0, after: 0 } },
      heading4: { font: '黑体', fontSize: '小四', spacing: { before: 0, after: 0 } },
      header: { content: '', font: '宋体', fontSize: '小五' },
      footer: { position: '居中', format: '第X页 共X页' },
      tableFont: '宋体',
      tableFontSize: '小四',
      tableBorder: '单线',
      tableAlign: '居中'
    },
    isPreset: true,
    createdAt: '2026-01-01',
    tags: ['央企', '标准']
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
      heading1: { font: '黑体', fontSize: '三号', spacing: { before: 0.5, after: 0.5 } },
      heading2: { font: '黑体', fontSize: '四号', spacing: { before: 0.5, after: 0.5 } },
      heading3: { font: '黑体', fontSize: '小四', spacing: { before: 0, after: 0 } },
      heading4: { font: '黑体', fontSize: '小四', spacing: { before: 0, after: 0 } },
      header: { content: '', font: '仿宋', fontSize: '小五' },
      footer: { position: '居中', format: '第X页' },
      tableFont: '仿宋',
      tableFontSize: '小四',
      tableBorder: '单线',
      tableAlign: '居中'
    },
    isPreset: true,
    createdAt: '2026-01-01',
    tags: ['国企', '投标']
  },
  {
    id: 'construction',
    name: '建设工程招标模板',
    description: '字体: 宋体四号 | 页边距: 2.5cm | 装订线: 0.5cm',
    format: {
      paperSize: 'A4',
      marginTop: 2.5,
      marginBottom: 2.5,
      marginLeft: 2.5,
      marginRight: 2.0,
      gutter: 0.5,
      gutterPosition: 'left',
      orientation: 'portrait',
      bodyFont: '宋体',
      bodyFontSize: '四号',
      bodyLineHeight: '1.5倍',
      bodyParagraphSpacing: { before: 0, after: 0 },
      heading1: { font: '黑体', fontSize: '三号', spacing: { before: 0.5, after: 0.5 } },
      heading2: { font: '黑体', fontSize: '四号', spacing: { before: 0.5, after: 0.5 } },
      heading3: { font: '楷体', fontSize: '四号', spacing: { before: 0, after: 0 } },
      heading4: { font: '楷体', fontSize: '小四', spacing: { before: 0, after: 0 } },
      header: { content: '', font: '宋体', fontSize: '小五' },
      footer: { position: '居中', format: '第X页 共X页' },
      tableFont: '宋体',
      tableFontSize: '小四',
      tableBorder: '单线',
      tableAlign: '居中'
    },
    isPreset: true,
    createdAt: '2026-01-01',
    tags: ['建设工程', '招标']
  }
])

const customTemplates = ref<FormatTemplate[]>([])

// 加载模板列表
async function loadTemplates() {
  try {
    const templates = await invoke<FormatTemplate[]>('get_templates')
    // 分离预设和自定义模板
    customTemplates.value = templates.filter(t => !t.isPreset)
  } catch (error) {
    console.error('加载模板失败:', error)
  }
}

function viewTemplate(tpl: FormatTemplate) {
  currentTemplate.value = tpl
  detailDialogVisible.value = true
}

function useTemplate(tpl: FormatTemplate) {
  store.setFormatRequirement(tpl.format)
  ElMessage.success(`已选择模板: ${tpl.name}`)
  router.push('/')
}

function useCurrentTemplate() {
  if (currentTemplate.value) {
    useTemplate(currentTemplate.value)
  }
  detailDialogVisible.value = false
}

function createTemplate() {
  editingTemplate.value = null
  templateForm.value = {
    name: '',
    description: ''
  }
  editDialogVisible.value = true
}

function editTemplate(tpl: FormatTemplate) {
  editingTemplate.value = tpl
  templateForm.value = {
    name: tpl.name,
    description: tpl.description
  }
  editDialogVisible.value = true
}

async function saveTemplateForm() {
  if (!templateForm.value.name) {
    ElMessage.warning('请输入模板名称')
    return
  }
  
  try {
    if (editingTemplate.value) {
      // 更新模板
      const index = customTemplates.value.findIndex(t => t.id === editingTemplate.value!.id)
      if (index !== -1) {
        customTemplates.value[index] = {
          ...editingTemplate.value,
          ...templateForm.value
        }
      }
    } else {
      // 创建新模板
      const template = await invoke<FormatTemplate>('save_template', {
        name: templateForm.value.name,
        description: templateForm.value.description,
        format: store.currentRequirement || presetTemplates.value[0].format
      })
      customTemplates.value.push(template)
    }
    
    ElMessage.success(editingTemplate.value ? '模板已更新' : '模板已创建')
    editDialogVisible.value = false
  } catch (error) {
    ElMessage.error('保存失败: ' + error)
  }
}

function duplicateTemplate(tpl: FormatTemplate) {
  editingTemplate.value = null
  templateForm.value = {
    name: `${tpl.name} (副本)`,
    description: tpl.description
  }
  editDialogVisible.value = true
}

function deleteTemplate(tpl: FormatTemplate) {
  ElMessageBox.confirm(`确定要删除模板 "${tpl.name}" 吗？`, '提示', {
    confirmButtonText: '确定',
    cancelButtonText: '取消',
    type: 'warning'
  }).then(async () => {
    try {
      await invoke('delete_template', { id: tpl.id })
      customTemplates.value = customTemplates.value.filter(t => t.id !== tpl.id)
      ElMessage.success('删除成功')
    } catch (error) {
      ElMessage.error('删除失败: ' + error)
    }
  }).catch(() => {})
}

function importTemplate() {
  ElMessage.info('导入功能开发中...')
}

function exportTemplates() {
  ElMessage.info('导出功能开发中...')
}

onMounted(() => {
  loadTemplates()
})
</script>

<style scoped>
.templates-page {
  max-width: 800px;
  margin: 0 auto;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
}

.card-header h3 {
  font-size: 18px;
  font-weight: 600;
}

.template-section {
  margin-bottom: 32px;
}

.section-title {
  font-size: 14px;
  font-weight: 600;
  color: #606266;
  margin-bottom: 16px;
  display: flex;
  align-items: center;
  gap: 8px;
}

.dark .section-title {
  color: #a0aec0;
}

.template-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.template-card {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px;
  background: #fafafa;
  border-radius: 8px;
  border: 1px solid #ebeef5;
  transition: all 0.3s;
}

.template-card:hover {
  border-color: #409eff;
  box-shadow: 0 2px 12px rgba(64, 158, 255, 0.1);
}

.dark .template-card {
  background: #1e3a5f;
  border-color: #2d3748;
}

.template-card.preset {
  background: linear-gradient(135deg, #f5f7fa 0%, #e4e7ed 100%);
}

.dark .template-card.preset {
  background: linear-gradient(135deg, #1e3a5f 0%, #2d4a6f 100%);
}

.template-info {
  flex: 1;
}

.template-name {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 15px;
  font-weight: 500;
  color: #303133;
}

.dark .template-name {
  color: #e5e7eb;
}

.template-desc {
  font-size: 12px;
  color: #909399;
  margin-top: 4px;
}

.template-tags {
  margin-top: 8px;
  display: flex;
  gap: 4px;
}

.template-meta {
  font-size: 12px;
  color: #c0c4cc;
  margin-top: 4px;
}

.template-actions {
  display: flex;
  gap: 8px;
}

.empty-tip {
  padding: 40px;
  background: #fafafa;
  border-radius: 8px;
}

.dark .empty-tip {
  background: #1e3a5f;
}

.import-export {
  display: flex;
  gap: 12px;
  justify-content: center;
  padding-top: 16px;
  border-top: 1px solid #ebeef5;
}
</style>
