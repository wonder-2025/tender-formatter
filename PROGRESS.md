# 标书格式一键优化工具 - 开发进度

## 设计者信息

| 角色 | 名称 |
|------|------|
| 🎯 产品设计 | wonder-宏 |
| 🤖 架构设计 | JARVIS AI Assistant |
| 💻 开发实现 | JARVIS AI Assistant |

## 已完成的功能模块

### 后端服务 (Rust/Tauri)

#### 1. 核心服务 `src-tauri/src/services/`

| 文件 | 功能 | 状态 |
|------|------|------|
| `format_extractor.rs` | 格式提取器 - 读取招标文件、调用AI提取格式要求 | ✅ 完成 |
| `docx_editor.rs` | Word文档编辑器 - 修改格式属性、样式、保存文档 | ✅ 完成 |
| `diff_engine.rs` | 差异对比引擎 - 分析格式差异、生成差异列表 | ✅ 完成 |
| `llm_client.rs` | LLM客户端 - 支持百度/阿里/OpenAI/DeepSeek | ✅ 完成 |
| `logger.rs` | 日志记录器 - Debug模式日志记录 | ✅ 完成 |

#### 2. 命令层 `src-tauri/src/commands/`

| 文件 | 功能 | 状态 |
|------|------|------|
| `format.rs` | 格式提取、分析、对比、应用命令 | ✅ 完成 |
| `document.rs` | 文档打开、保存、备份命令 | ✅ 完成 |
| `template.rs` | 模板管理命令 | ✅ 完成 |
| `config.rs` | 配置加载、保存、API测试命令 | ✅ 完成 |
| `debug.rs` | Debug配置、日志读取、清理命令 | ✅ 完成 |

#### 3. 数据模型 `src-tauri/src/models/`

| 文件 | 功能 | 状态 |
|------|------|------|
| `format.rs` | 格式要求、模板数据结构 | ✅ 完成 |
| `debug.rs` | Debug配置数据结构 | ✅ 完成 |

#### 4. 安全模块 `src-tauri/src/security/`

| 文件 | 功能 | 状态 |
|------|------|------|
| `desensitize.rs` | 敏感信息脱敏处理 | ✅ 完成 |
| `audit.rs` | 审计日志记录 | ✅ 完成 |

### 前端页面 (Vue 3 + Element Plus)

#### 视图组件 `src/views/`

| 文件 | 功能 | 状态 |
|------|------|------|
| `Home.vue` | 首页 - 文件选择、模板选择入口 | ✅ 完成 |
| `FormatConfirm.vue` | 格式确认页 - 显示/编辑提取的格式要求 | ✅ 完成 |
| `Preview.vue` | 预览对比页 - 显示差异列表、执行优化 | ✅ 完成 |
| `Templates.vue` | 模板管理页 - 预设/自定义模板管理 | ✅ 完成 |
| `Settings.vue` | 设置页 - API配置、输出设置、Debug设置 | ✅ 完成 |
| `About.vue` | 关于页 - 版本信息、设计者信息 | ✅ 完成 |

## Debug 调试功能

### 功能特性

1. **开关控制**: 用户可手动开启/关闭 Debug 模式
2. **日志级别**: INFO / DEBUG / TRACE 三级
3. **日志内容选项**:
   - 脱敏过程
   - API 请求
   - API 响应
   - 文件操作
   - 格式修改

4. **日志管理**:
   - 查看日志文件
   - 打开日志目录
   - 清空日志
   - 自动按天分割
   - 保留最近 7 天

5. **敏感信息保护**:
   - API Key 只显示前4位和后4位
   - 自动脱敏手机号、身份证等

### 日志格式示例

```
[2026-03-11 09:35:00.123] [DEBUG] [DESENSITIZE] 
  Original: 张三，身份证号：123456789012345678
  Desensitized: 张三，身份证号：1234****5678
  Applied rules: [身份证号]

[2026-03-11 09:35:01.456] [DEBUG] [API_REQUEST]
  Provider: 百度千帆
  Endpoint: https://aip.baidubce.com/...
  Authorization: Bearer 1234****5678...

[2026-03-11 09:35:03.789] [DEBUG] [FORMAT_CHANGE]
  Property: bodyFontSize
  Old: 五号
  New: 小四
```

## 技术栈

### 后端
- **框架**: Tauri 2.0
- **语言**: Rust
- **依赖**:
  - `docx-rs` - Word 文档操作
  - `zip` + `quick-xml` - DOCX 文件解析
  - `lopdf` - PDF 读取
  - `reqwest` - HTTP 客户端
  - `serde` / `serde_json` - 序列化
  - `regex` - 正则表达式
  - `chrono` - 时间处理
  - `parking_lot` - 并发锁
  - `once_cell` - 单例模式

### 前端
- **框架**: Vue 3
- **UI**: Element Plus
- **状态管理**: Pinia
- **路由**: Vue Router
- **构建**: Vite + TypeScript

## 编译和运行

### 开发模式
```bash
cd tender-formatter
npm run tauri dev
```

### 生产构建
```bash
npm run tauri build
```

## 文件结构

```
tender-formatter/
├── src/                          # 前端源码
│   ├── views/                    # 页面组件
│   │   ├── Home.vue             # 首页
│   │   ├── FormatConfirm.vue    # 格式确认
│   │   ├── Preview.vue          # 预览对比
│   │   ├── Templates.vue        # 模板管理
│   │   ├── Settings.vue         # 设置（含Debug）
│   │   └── About.vue            # 关于
│   ├── stores/                   # Pinia 状态
│   ├── router/                   # 路由配置
│   └── App.vue                   # 根组件
│
├── src-tauri/                    # 后端源码
│   ├── src/
│   │   ├── services/            # 服务层
│   │   │   ├── format_extractor.rs  # 格式提取
│   │   │   ├── docx_editor.rs       # Word编辑
│   │   │   ├── diff_engine.rs       # 差异对比
│   │   │   ├── llm_client.rs        # LLM调用
│   │   │   └── logger.rs            # 日志服务
│   │   ├── commands/            # Tauri 命令
│   │   │   ├── format.rs
│   │   │   ├── document.rs
│   │   │   ├── template.rs
│   │   │   ├── config.rs
│   │   │   └── debug.rs
│   │   ├── models/              # 数据模型
│   │   ├── security/            # 安全模块
│   │   └── main.rs              # 入口
│   └── Cargo.toml               # Rust 依赖
│
└── package.json                  # Node 依赖
```

## 下一步计划

1. ✅ 后端核心服务完成
2. ✅ 前端页面完成
3. ⏳ 集成测试
4. ⏳ 打包发布
