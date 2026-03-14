# 标书格式一键优化工具 (Tender Formatter)

<div align="center">

![Version](https://img.shields.io/badge/version-1.0.0-blue.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)
![Platform](https://img.shields.io/badge/platform-Windows-lightgrey.svg)
![Tauri](https://img.shields.io/badge/Tauri-2.0-blue.svg)

**让用户专注于内容写作，格式交给工具一键优化**

</div>

---

## 👥 设计者

| 角色 | 名称 | 贡献 |
|------|------|------|
| 🎯 **产品设计** | wonder-宏 | 产品需求定义、功能规划、用户体验指导 |
| 🤖 **架构设计** | JARVIS AI Assistant | 技术架构、代码实现、安全设计 |
| 💻 **开发实现** | JARVIS AI Assistant | 前后端开发、测试、文档编写 |

---

## 📖 产品定位

### 核心价值

**让用户专注于内容写作，格式交给工具一键优化**

### 使用场景

```
用户写标书（无格式/任意格式）
        ↓
   一键优化格式
        ↓
   检查工具校验
        ↓
      提交投标
```

### 与检查工具的关系

| 工具 | 角色 | 时机 | 职责 |
|------|------|------|------|
| **优化工具** | 主动修复 | 写完标书后 | 格式属性调整 |
| **检查工具** | 再次校验 | 优化完成后 | 内容合规检查 |

---

## ✨ 核心功能

- **📄 导入招标文件** - AI 自动提取格式要求
- **📋 格式模板管理** - 预设模板 + 自定义模板
- **🔍 格式差异分析** - 对比当前格式与目标格式
- **📊 预览对比** - 左右对比优化效果
- **⚡ 一键优化** - 执行格式调整
- **🔒 数据安全** - 本地处理，敏感信息脱敏

---

## 🚀 快速开始

### 开发模式

```bash
# 安装依赖
npm install

# 启动开发服务器
npm run tauri:dev
```

### 构建发布

```bash
# 构建生产版本
npm run tauri:build
```

---

## 📁 项目结构

```
tender-formatter/
├── src/                    # Vue 前端代码
│   ├── views/              # 页面组件
│   ├── components/         # 通用组件
│   ├── stores/             # Pinia 状态管理
│   └── styles/             # 样式文件
│
├── src-tauri/              # Rust 后端代码
│   ├── src/
│   │   ├── commands/       # Tauri 命令
│   │   ├── services/       # 业务服务
│   │   ├── security/       # 安全模块
│   │   └── models/         # 数据模型
│   └── Cargo.toml
│
├── docs/                   # 文档
└── README.md
```

---

## 🔧 技术栈

| 层级 | 技术 |
|------|------|
| 前端框架 | Vue 3 + Vite |
| UI 组件库 | Element Plus |
| 状态管理 | Pinia |
| 桌面框架 | Tauri 2.0 |
| 后端语言 | Rust |
| LLM 支持 | 百度千帆 / 阿里通义 / OpenAI / DeepSeek |

---

## 📄 许可证

MIT License

---

<div align="center">

**Made with ❤️ by JARVIS AI Assistant**

*Designed for wonder-宏*

</div>
