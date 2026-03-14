<template>
  <div class="app-container" :class="{ 'dark-mode': isDark }">
    <el-config-provider :locale="zhCn">
      <el-container class="main-container">
        <!-- 顶部导航 -->
        <el-header class="app-header">
          <div class="header-left">
            <span class="app-title">📐 标书格式优化工具</span>
          </div>
          <div class="header-right">
            <el-button text @click="router.push('/templates')">
              <el-icon><Collection /></el-icon>
              模板
            </el-button>
            <el-button text @click="router.push('/settings')">
              <el-icon><Setting /></el-icon>
              设置
            </el-button>
            <el-button text @click="router.push('/about')">
              <el-icon><InfoFilled /></el-icon>
              关于
            </el-button>
            <el-button text @click="toggleDark">
              <el-icon v-if="isDark"><Sunny /></el-icon>
              <el-icon v-else><Moon /></el-icon>
            </el-button>
          </div>
        </el-header>
        
        <!-- 主内容区 -->
        <el-main class="app-main">
          <router-view />
        </el-main>
      </el-container>
    </el-config-provider>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { ElConfigProvider } from 'element-plus'
import zhCn from 'element-plus/es/locale/lang/zh-cn'

const router = useRouter()
const isDark = ref(false)

function toggleDark() {
  isDark.value = !isDark.value
  document.documentElement.classList.toggle('dark', isDark.value)
}
</script>

<style scoped>
.app-container {
  min-height: 100vh;
  background: #f5f7fa;
}

.app-container.dark-mode {
  background: #1a1a2e;
}

.main-container {
  height: 100vh;
}

.app-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  background: #fff;
  border-bottom: 1px solid #e4e7ed;
  padding: 0 20px;
  height: 56px;
}

.dark-mode .app-header {
  background: #16213e;
  border-bottom-color: #2d3748;
}

.header-left {
  display: flex;
  align-items: center;
}

.app-title {
  font-size: 18px;
  font-weight: 600;
  color: #303133;
}

.dark-mode .app-title {
  color: #e5e7eb;
}

.header-right {
  display: flex;
  gap: 8px;
}

.app-main {
  padding: 20px;
  overflow: auto;
}
</style>
