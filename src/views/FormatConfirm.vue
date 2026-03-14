<template>
  <div class="format-confirm-page">
    <div class="format-card">
      <div class="card-header">
        <h3>格式要求确认</h3>
        <span class="source-info">来源: {{ sourceLabel }}</span>
      </div>
      
      <!-- 页面设置 -->
      <el-collapse v-model="activeCollapse">
        <el-collapse-item title="📄 页面设置" name="page">
          <el-form label-width="100px">
            <el-row :gutter="20">
              <el-col :span="12">
                <el-form-item label="纸张大小">
                  <el-select v-model="format.paperSize">
                    <el-option label="A4 (210×297mm)" value="A4" />
                    <el-option label="A3 (297×420mm)" value="A3" />
                    <el-option label="B5 (176×250mm)" value="B5" />
                    <el-option label="16K (184×260mm)" value="16K" />
                  </el-select>
                </el-form-item>
              </el-col>
              <el-col :span="12">
                <el-form-item label="页方向">
                  <el-select v-model="format.orientation">
                    <el-option label="纵向" value="portrait" />
                    <el-option label="横向" value="landscape" />
                  </el-select>
                </el-form-item>
              </el-col>
            </el-row>
            
            <el-divider content-position="left">页边距 (单位: 厘米)</el-divider>
            
            <el-row :gutter="20">
              <el-col :span="6">
                <el-form-item label="上">
                  <el-input-number v-model="format.marginTop" :min="0" :max="5" :step="0.1" :precision="2" />
                </el-form-item>
              </el-col>
              <el-col :span="6">
                <el-form-item label="下">
                  <el-input-number v-model="format.marginBottom" :min="0" :max="5" :step="0.1" :precision="2" />
                </el-form-item>
              </el-col>
              <el-col :span="6">
                <el-form-item label="左">
                  <el-input-number v-model="format.marginLeft" :min="0" :max="5" :step="0.1" :precision="2" />
                </el-form-item>
              </el-col>
              <el-col :span="6">
                <el-form-item label="右">
                  <el-input-number v-model="format.marginRight" :min="0" :max="5" :step="0.1" :precision="2" />
                </el-form-item>
              </el-col>
            </el-row>
            
            <el-row :gutter="20">
              <el-col :span="12">
                <el-form-item label="装订线">
                  <el-input-number v-model="format.gutter" :min="0" :max="2" :step="0.1" :precision="2" />
                </el-form-item>
              </el-col>
              <el-col :span="12">
                <el-form-item label="装订位置">
                  <el-select v-model="format.gutterPosition">
                    <el-option label="左" value="left" />
                    <el-option label="上" value="top" />
                  </el-select>
                </el-form-item>
              </el-col>
            </el-row>
          </el-form>
        </el-collapse-item>
        
        <!-- 字体格式 -->
        <el-collapse-item title="🔤 字体格式" name="font">
          <h4 class="sub-title">正文</h4>
          <el-form label-width="100px">
            <el-row :gutter="20">
              <el-col :span="8">
                <el-form-item label="字体">
                  <el-select v-model="format.bodyFont">
                    <el-option label="宋体" value="宋体" />
                    <el-option label="仿宋" value="仿宋" />
                    <el-option label="黑体" value="黑体" />
                    <el-option label="楷体" value="楷体" />
                    <el-option label="微软雅黑" value="微软雅黑" />
                  </el-select>
                </el-form-item>
              </el-col>
              <el-col :span="8">
                <el-form-item label="字号">
                  <el-select v-model="format.bodyFontSize">
                    <el-option label="二号" value="二号" />
                    <el-option label="三号" value="三号" />
                    <el-option label="四号" value="四号" />
                    <el-option label="小四" value="小四" />
                    <el-option label="五号" value="五号" />
                    <el-option label="小五" value="小五" />
                  </el-select>
                </el-form-item>
              </el-col>
              <el-col :span="8">
                <el-form-item label="行间距">
                  <el-select v-model="format.bodyLineHeight">
                    <el-option label="单倍行距" value="1.0倍" />
                    <el-option label="1.15倍行距" value="1.15倍" />
                    <el-option label="1.25倍行距" value="1.25倍" />
                    <el-option label="1.5倍行距" value="1.5倍" />
                    <el-option label="2倍行距" value="2.0倍" />
                  </el-select>
                </el-form-item>
              </el-col>
            </el-row>
          </el-form>
          
          <h4 class="sub-title">标题格式</h4>
          <el-table :data="headingTable" border size="small">
            <el-table-column prop="level" label="级别" width="80" />
            <el-table-column label="字体" width="150">
              <template #default="{ row }">
                <el-select v-model="row.font" size="small">
                  <el-option label="黑体" value="黑体" />
                  <el-option label="宋体" value="宋体" />
                  <el-option label="楷体" value="楷体" />
                  <el-option label="仿宋" value="仿宋" />
                </el-select>
              </template>
            </el-table-column>
            <el-table-column label="字号" width="120">
              <template #default="{ row }">
                <el-select v-model="row.fontSize" size="small">
                  <el-option label="二号" value="二号" />
                  <el-option label="三号" value="三号" />
                  <el-option label="四号" value="四号" />
                  <el-option label="小四" value="小四" />
                </el-select>
              </template>
            </el-table-column>
            <el-table-column label="段前" width="100">
              <template #default="{ row }">
                <el-input-number v-model="row.spacing.before" :min="0" :max="2" :step="0.5" size="small" />
              </template>
            </el-table-column>
            <el-table-column label="段后" width="100">
              <template #default="{ row }">
                <el-input-number v-model="row.spacing.after" :min="0" :max="2" :step="0.5" size="small" />
              </template>
            </el-table-column>
          </el-table>
        </el-collapse-item>
        
        <!-- 页眉页脚 -->
        <el-collapse-item title="📋 页眉页脚" name="header">
          <el-form label-width="100px">
            <el-row :gutter="20">
              <el-col :span="12">
                <el-form-item label="页眉内容">
                  <el-input v-model="format.header.content" placeholder="项目名称" />
                </el-form-item>
              </el-col>
              <el-col :span="6">
                <el-form-item label="页眉字体">
                  <el-select v-model="format.header.font">
                    <el-option label="宋体" value="宋体" />
                    <el-option label="仿宋" value="仿宋" />
                  </el-select>
                </el-form-item>
              </el-col>
              <el-col :span="6">
                <el-form-item label="页眉字号">
                  <el-select v-model="format.header.fontSize">
                    <el-option label="小五" value="小五" />
                    <el-option label="五号" value="五号" />
                  </el-select>
                </el-form-item>
              </el-col>
            </el-row>
            
            <el-divider />
            
            <el-row :gutter="20">
              <el-col :span="12">
                <el-form-item label="页码位置">
                  <el-select v-model="format.footer.position">
                    <el-option label="居中" value="居中" />
                    <el-option label="右下角" value="右下角" />
                    <el-option label="左下角" value="左下角" />
                  </el-select>
                </el-form-item>
              </el-col>
              <el-col :span="12">
                <el-form-item label="页码格式">
                  <el-select v-model="format.footer.format">
                    <el-option label="第X页" value="第X页" />
                    <el-option label="第X页 共X页" value="第X页 共X页" />
                    <el-option label="X/Y" value="X/Y" />
                    <el-option label="仅页码" value="X" />
                  </el-select>
                </el-form-item>
              </el-col>
            </el-row>
          </el-form>
        </el-collapse-item>
        
        <!-- 表格格式 -->
        <el-collapse-item title="📊 表格格式" name="table">
          <el-form label-width="100px">
            <el-row :gutter="20">
              <el-col :span="8">
                <el-form-item label="表格字体">
                  <el-select v-model="format.tableFont">
                    <el-option label="宋体" value="宋体" />
                    <el-option label="仿宋" value="仿宋" />
                    <el-option label="黑体" value="黑体" />
                  </el-select>
                </el-form-item>
              </el-col>
              <el-col :span="8">
                <el-form-item label="表格字号">
                  <el-select v-model="format.tableFontSize">
                    <el-option label="小四" value="小四" />
                    <el-option label="五号" value="五号" />
                    <el-option label="小五" value="小五" />
                  </el-select>
                </el-form-item>
              </el-col>
              <el-col :span="8">
                <el-form-item label="表格边框">
                  <el-select v-model="format.tableBorder">
                    <el-option label="单线" value="单线" />
                    <el-option label="双线" value="双线" />
                    <el-option label="无边框" value="无边框" />
                  </el-select>
                </el-form-item>
              </el-col>
            </el-row>
          </el-form>
        </el-collapse-item>
      </el-collapse>
      
      <!-- 操作按钮 -->
      <div class="action-bar">
        <el-button @click="router.back()">返回</el-button>
        <el-button type="info" @click="saveAsTemplate">
          <el-icon><Download /></el-icon>
          保存为模板
        </el-button>
        <el-button type="primary" @click="goToPreview">
          下一步：分析文档
          <el-icon class="el-icon--right"><ArrowRight /></el-icon>
        </el-button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage } from 'element-plus'
import { useFormatStore, type FormatRequirement } from '../stores/format'
import { invoke } from '@tauri-apps/api/core'

const router = useRouter()
const store = useFormatStore()

const activeCollapse = ref(['page', 'font', 'header'])

const format = reactive<FormatRequirement>({
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
})

// 标题表格数据
const headingTable = computed(() => [
  { level: '一级标题', ...format.heading1 },
  { level: '二级标题', ...format.heading2 },
  { level: '三级标题', ...format.heading3 },
  { level: '四级标题', ...format.heading4 }
])

const sourceLabel = computed(() => {
  return store.tenderDocument || '手动设置'
})

// 加载已有格式
onMounted(() => {
  if (store.currentRequirement) {
    Object.assign(format, store.currentRequirement)
  }
})

async function saveAsTemplate() {
  try {
    await invoke('save_template', {
      name: `自定义模板 ${new Date().toLocaleDateString()}`,
      description: generateTemplateDescription(),
      format: format
    })
    ElMessage.success('模板保存成功')
  } catch (error) {
    ElMessage.error('保存失败: ' + error)
  }
}

function generateTemplateDescription(): string {
  return `字体: ${format.bodyFont}${format.bodyFontSize} | 页边距: ${format.marginTop}/${format.marginLeft}cm | 行距: ${format.bodyLineHeight}`
}

async function goToPreview() {
  // 保存格式设置
  store.setFormatRequirement(format)
  
  // 如果有选择文档，分析格式差异
  if (store.currentDocument) {
    try {
      const currentFormat = await invoke<any>('analyze_document_format', {
        filePath: store.currentDocument
      })
      
      const diffs = await invoke<any[]>('compare_format_diff', {
        current: currentFormat,
        target: format
      })
      
      store.setFormatDiffs(diffs)
    } catch (error) {
      console.error('分析失败:', error)
    }
  }
  
  router.push('/preview')
}
</script>

<style scoped>
.format-confirm-page {
  max-width: 900px;
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
  color: #303133;
}

.source-info {
  font-size: 12px;
  color: #909399;
}

.sub-title {
  font-size: 14px;
  font-weight: 600;
  color: #606266;
  margin: 16px 0 12px;
}

.dark .sub-title {
  color: #a0aec0;
}

.action-bar {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  margin-top: 24px;
}
</style>
