<template>
  <div class="about-page">
    <div class="format-card about-content">
      <div class="logo-section">
        <span class="logo">📐</span>
        <h1 class="app-name">标书格式优化工具</h1>
        <p class="version">v1.0.0</p>
        <p class="slogan">一键优化标书格式，提升投标效率</p>
      </div>
      
      <el-divider />
      
      <!-- 设计者 -->
      <div class="designers-section">
        <h3>👥 设计团队</h3>
        <el-table :data="designers" border size="small">
          <el-table-column prop="role" label="角色" width="140">
            <template #default="{ row }">
              <span>{{ row.role }}</span>
            </template>
          </el-table-column>
          <el-table-column prop="name" label="名称" width="150" />
          <el-table-column prop="contribution" label="贡献" />
        </el-table>
      </div>
      
      <el-divider />
      
      <!-- 功能特性 -->
      <div class="features-section">
        <h3>✨ 功能特性</h3>
        <el-row :gutter="16">
          <el-col :span="12" v-for="feature in features" :key="feature.title">
            <div class="feature-item">
              <div class="feature-icon">{{ feature.icon }}</div>
              <div class="feature-content">
                <h4>{{ feature.title }}</h4>
                <p>{{ feature.desc }}</p>
              </div>
            </div>
          </el-col>
        </el-row>
      </div>
      
      <el-divider />
      
      <!-- 技术信息 -->
      <div class="tech-section">
        <h3>📋 技术信息</h3>
        <el-descriptions :column="2" border size="small">
          <el-descriptions-item label="技术栈">Vue 3 + Tauri 2.0 + Rust</el-descriptions-item>
          <el-descriptions-item label="UI框架">Element Plus</el-descriptions-item>
          <el-descriptions-item label="AI能力">百度千帆 / OpenAI / 通义千问</el-descriptions-item>
          <el-descriptions-item label="许可证">MIT</el-descriptions-item>
          <el-descriptions-item label="支持格式">.docx / .pdf</el-descriptions-item>
          <el-descriptions-item label="平台支持">Windows / macOS / Linux</el-descriptions-item>
        </el-descriptions>
      </div>
      
      <el-divider />
      
      <!-- 使用说明 -->
      <div class="usage-section">
        <h3>📖 使用说明</h3>
        <el-steps :active="4" direction="vertical" simple>
          <el-step title="上传招标文件" description="支持 docx、pdf 格式" />
          <el-step title="AI 提取格式要求" description="自动识别并脱敏敏感信息" />
          <el-step title="确认/调整格式" description="可视化编辑，支持保存模板" />
          <el-step title="分析差异" description="对比当前文档与目标格式" />
          <el-step title="执行优化" description="一键应用，生成优化后文档" />
        </el-steps>
      </div>
      
      <el-divider />
      
      <!-- 操作按钮 -->
      <div class="actions-section">
        <el-button @click="openHelp">
          <el-icon><Document /></el-icon>
          使用文档
        </el-button>
        <el-button @click="reportIssue">
          <el-icon><Warning /></el-icon>
          反馈问题
        </el-button>
        <el-button @click="checkUpdate">
          <el-icon><Refresh /></el-icon>
          检查更新
        </el-button>
      </div>
      
      <el-divider />
      
      <!-- 版权信息 -->
      <div class="copyright">
        <p>© 2026 wonder-宏 & JARVIS AI Assistant</p>
        <p class="slogan">让标书格式优化更简单</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ElMessage } from 'element-plus'

const designers = [
  {
    role: '🎯 产品设计',
    name: 'wonder-宏',
    contribution: '产品需求定义、功能规划、用户体验指导'
  },
  {
    role: '🤖 架构设计',
    name: 'JARVIS AI',
    contribution: '技术架构设计、系统方案规划、技术选型'
  },
  {
    role: '💻 开发实现',
    name: 'JARVIS AI',
    contribution: '前后端开发、测试、文档编写、持续优化'
  }
]

const features = [
  {
    icon: '🚀',
    title: '智能提取',
    desc: 'AI 自动识别招标文件中的格式要求'
  },
  {
    icon: '🔒',
    title: '安全脱敏',
    desc: '自动脱敏敏感信息，保护商业机密'
  },
  {
    icon: '📊',
    title: '可视化对比',
    desc: '直观展示格式差异，一目了然'
  },
  {
    icon: '⚡',
    title: '一键优化',
    desc: '批量修改格式，大幅提升效率'
  },
  {
    icon: '💾',
    title: '模板管理',
    desc: '保存常用格式模板，快速复用'
  },
  {
    icon: '🐛',
    title: '调试日志',
    desc: '详细日志记录，快速定位问题'
  }
]

function openHelp() {
  ElMessage.info('使用文档开发中...')
}

function reportIssue() {
  ElMessage.info('反馈功能开发中...')
}

async function checkUpdate() {
  ElMessage.info('正在检查更新...')
  // 模拟检查
  setTimeout(() => {
    ElMessage.success('当前已是最新版本')
  }, 1000)
}
</script>

<style scoped>
.about-page {
  display: flex;
  justify-content: center;
  padding: 20px;
}

.about-content {
  max-width: 700px;
  text-align: center;
}

.logo-section {
  padding: 40px 0;
}

.logo {
  font-size: 64px;
}

.app-name {
  font-size: 24px;
  font-weight: 600;
  margin: 16px 0 8px;
}

.version {
  color: #909399;
  font-size: 14px;
}

.slogan {
  color: #409eff;
  font-size: 14px;
  margin-top: 8px;
}

.designers-section,
.tech-section,
.usage-section,
.features-section {
  text-align: left;
  margin: 24px 0;
}

.designers-section h3,
.tech-section h3,
.usage-section h3,
.features-section h3 {
  font-size: 14px;
  font-weight: 600;
  margin-bottom: 16px;
}

.feature-item {
  display: flex;
  gap: 12px;
  padding: 12px;
  background: #f5f7fa;
  border-radius: 8px;
  margin-bottom: 12px;
}

.dark .feature-item {
  background: #1e3a5f;
}

.feature-icon {
  font-size: 24px;
}

.feature-content h4 {
  font-size: 14px;
  font-weight: 600;
  margin: 0 0 4px 0;
}

.feature-content p {
  font-size: 12px;
  color: #909399;
  margin: 0;
}

.actions-section {
  display: flex;
  justify-content: center;
  gap: 12px;
  margin: 24px 0;
}

.copyright {
  color: #909399;
  font-size: 12px;
  line-height: 2;
}

/* 覆盖 steps 样式 */
.usage-section :deep(.el-step__title) {
  font-size: 13px;
}

.usage-section :deep(.el-step__description) {
  font-size: 12px;
  color: #909399;
}
</style>
