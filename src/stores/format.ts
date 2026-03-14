import { defineStore } from 'pinia'
import { ref } from 'vue'

export interface FormatRequirement {
  // 页面设置
  paperSize: string
  marginTop: number
  marginBottom: number
  marginLeft: number
  marginRight: number
  gutter: number
  gutterPosition: 'left' | 'top'
  orientation: 'portrait' | 'landscape'
  
  // 字体格式
  bodyFont: string
  bodyFontSize: string
  bodyLineHeight: string
  bodyParagraphSpacing: { before: number; after: number }
  
  heading1: { font: string; fontSize: string; spacing: { before: number; after: number } }
  heading2: { font: string; fontSize: string; spacing: { before: number; after: number } }
  heading3: { font: string; fontSize: string; spacing: { before: number; after: number } }
  heading4: { font: string; fontSize: string; spacing: { before: number; after: number } }
  
  // 页眉页脚
  header: { content: string; font: string; fontSize: string }
  footer: { position: string; format: string }
  
  // 表格格式
  tableFont: string
  tableFontSize: string
  tableBorder: string
  tableAlign: string
}

export interface FormatDiff {
  category: string
  name: string
  current: string
  target: string
}

export interface FormatTemplate {
  id: string
  name: string
  description: string
  format: FormatRequirement
  isPreset: boolean
  createdAt: string
  lastUsedAt?: string
  tags?: string[]
}

export const useFormatStore = defineStore('format', () => {
  // 当前格式要求
  const currentRequirement = ref<FormatRequirement | null>(null)
  
  // 格式差异列表
  const formatDiffs = ref<FormatDiff[]>([])
  
  // 模板列表
  const templates = ref<FormatTemplate[]>([])
  
  // 当前文档路径
  const currentDocument = ref<string>('')
  
  // 招标文件路径
  const tenderDocument = ref<string>('')
  
  // 来源类型
  const sourceType = ref<'tender' | 'template'>('tender')
  
  // 选中的模板
  const selectedTemplate = ref<string>('')
  
  // 输出方式
  const outputMode = ref<'modify' | 'new' | 'copy'>('new')
  
  // 设置格式要求
  function setFormatRequirement(format: FormatRequirement) {
    currentRequirement.value = format
  }
  
  // 设置格式差异
  function setFormatDiffs(diffs: FormatDiff[]) {
    formatDiffs.value = diffs
  }
  
  // 设置当前文档
  function setCurrentDocument(path: string) {
    currentDocument.value = path
  }
  
  // 设置招标文件
  function setTenderDocument(path: string) {
    tenderDocument.value = path
  }
  
  // 添加模板
  function addTemplate(template: FormatTemplate) {
    templates.value.push(template)
  }
  
  // 删除模板
  function removeTemplate(id: string) {
    templates.value = templates.value.filter(t => t.id !== id)
  }
  
  // 更新模板
  function updateTemplate(id: string, template: Partial<FormatTemplate>) {
    const index = templates.value.findIndex(t => t.id === id)
    if (index !== -1) {
      templates.value[index] = { ...templates.value[index], ...template }
    }
  }
  
  // 重置状态
  function reset() {
    currentRequirement.value = null
    formatDiffs.value = []
    currentDocument.value = ''
    tenderDocument.value = ''
    sourceType.value = 'tender'
    selectedTemplate.value = ''
  }
  
  return {
    currentRequirement,
    formatDiffs,
    templates,
    currentDocument,
    tenderDocument,
    sourceType,
    selectedTemplate,
    outputMode,
    setFormatRequirement,
    setFormatDiffs,
    setCurrentDocument,
    setTenderDocument,
    addTemplate,
    removeTemplate,
    updateTemplate,
    reset
  }
})
