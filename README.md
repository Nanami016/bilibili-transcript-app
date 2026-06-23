# Bilibili Transcript App

B站视频转录 macOS 原生应用 — 支持视频下载、音频提取、Whisper 语音转文字、AI 摘要。

基于 [Tauri v2](https://v2.tauri.app/) + React + TypeScript 构建的轻量级桌面应用。

## 功能

| 功能 | 状态 | 说明 |
|------|------|------|
| 🎬 视频下载 | ✅ 已测试 | 支持多种分辨率/格式选择，自动清理格式 ID 后缀 |
| 🎵 音频下载 | ✅ 已测试 | yt-dlp 提取 + ffmpeg 转码 |
| 📺 视频解析 | ✅ 已测试 | B站链接解析、封面获取、视频信息展示 |
| 📁 收藏夹 | ✅ 已测试 | 浏览收藏夹、分页加载、下载视频/音频、语音转录 |
| 📜 历史记录 | ✅ 已测试 | 下载/转录的历史记录，转录显示耗时 |
| 📋 任务中心 | ✅ 已测试 | 实时进度追踪、历史记录、任务管理 |
| ⚙️ 设置 | ✅ 已测试 | Cookie 导入（浏览器/手动）、输出目录配置、自动保存 |
| 🎤 语音转文字 | ✅ 已测试 | Whisper 支持（OpenAI API / 本地 REST API），转录弹窗支持语言选择和提示词 |
| 🤖 AI 摘要 | ⚠️ 未充分测试 | 集成到转录流程，转录后自动触发（可选），支持自定义指令和上下文 |

## 安装

### 方式一：下载 Release（推荐）

前往 [Releases](https://github.com/Nanami016/bilibili-transcript-app/releases) 页面下载最新 `.dmg` 文件。

1. 双击打开 DMG，将应用拖入 Applications 文件夹
2. 首次打开需在「系统设置 → 隐私与安全性」中允许运行

**系统要求：** macOS 12.0 (Monterey) 或更高版本，Apple Silicon (M1/M2/M3/M4)

### 方式二：从源码构建

```bash
# 前置依赖
brew install yt-dlp ffmpeg

# 克隆并构建
git clone https://github.com/Nanami016/bilibili-transcript-app.git
cd bilibili-transcript-app
npm install
npm run tauri build
```

## 使用说明

### 1. 配置 B站 Cookie

首次使用需导入 B站 Cookie 以访问个人数据（收藏夹、历史记录等）。

**推荐方式：从浏览器读取**
- 在应用设置中点击「从 Chrome/Safari/Firefox 读取」
- 读取前请关闭对应浏览器

**手动方式：**
- 浏览器登录 B站 → F12 开发者工具 → Application → Cookies
- 复制 `SESSDATA`、`DedeUserID`、`bili_jct` 的值
- 在设置页粘贴导入

### 2. 下载视频/音频

在首页输入 B站视频链接，选择分辨率后下载。音频会自动通过 ffmpeg 转码为 MP3。

### 3. 语音转文字

在设置页配置 Whisper API：
- **远程模式**：填入 OpenAI API Key
- **本地模式**：填入本地 Whisper 服务地址（如 [FunAudioLLM-Server](https://github.com/Nanami016/FunAudioLLM-Server)），无需 API Key

点击「语音转录」时弹出配置弹窗：
- **视频语言**：中文/日文/英文/韩文/法文/德文/俄文/西班牙文
- **转录提示词**（可选）：提高特定词汇的识别准确度
- **AI 摘要指令**（可选）：自定义摘要风格
- **上下文文本**（可选）：为 AI 提供额外背景信息

转录完成后，如已启用 AI 摘要，会自动调用 AI 生成摘要（失败不影响转录结果）。

## 输出目录

默认输出路径（可在设置中修改）：

```
~/Downloads/bilibili-transcript-app/
├── bilibili-video/          # 视频文件
├── bilibili-audio/          # 音频文件
└── bilibili-transfer/       # 转录结果（含 AI 摘要）
```

---

## 项目结构

```
bilibili-transcript-app/
├── src/                          # 前端（React + TypeScript）
│   ├── components/               # UI 组件
│   │   ├── InputBar.tsx          #   链接输入栏
│   │   ├── Sidebar.tsx           #   左侧导航
│   │   ├── TaskPanel.tsx         #   任务面板
│   │   ├── VideoCard.tsx         #   视频信息卡片
│   │   ├── TranscribeModal.tsx   #   转录配置弹窗（语言/提示词/AI指令）
│   │   └── common/               #   通用组件（Button, Spinner, Toast）
│   ├── pages/                    # 页面
│   │   ├── Home.tsx              #   首页
│   │   ├── VideoDownload.tsx     #   视频下载
│   │   ├── AudioDownload.tsx     #   音频下载
│   │   ├── AudioTranscribe.tsx   #   语音转文字（含 AI 摘要）
│   │   ├── Favorite.tsx          #   收藏夹（分页加载）
│   │   ├── Settings.tsx          #   设置
│   │   └── Logs.tsx              #   运行日志
│   └── lib/tauri.ts              # Tauri API 封装
│
├── src-tauri/                    # 后端（Rust + Tauri v2）
│   ├── src/
│   │   ├── bilibili/             # B站 API 交互
│   │   │   ├── api.rs            #   请求封装（视频信息、收藏夹等）
│   │   │   ├── cookie.rs         #   Cookie 管理
│   │   │   ├── subtitle.rs       #   字幕获取
│   │   │   └── types.rs          #   数据类型定义
│   │   ├── commands/             # Tauri 命令（前端可调用）
│   │   │   ├── video.rs          #   视频相关命令
│   │   │   ├── transcribe.rs     #   转录命令
│   │   │   ├── favorite.rs       #   收藏夹命令
│   │   │   ├── history.rs        #   任务历史记录命令
│   │   │   ├── config.rs         #   配置命令
│   │   │   ├── cookie.rs         #   Cookie 命令
│   │   │   ├── task.rs           #   任务管理命令
│   │   │   └── log.rs            #   日志命令
│   │   ├── config/               # 配置管理
│   │   ├── download/             # 下载模块（yt-dlp + ffmpeg）
│   │   ├── storage/              # 本地存储（SQLite）
│   │   ├── summary/              # AI 摘要（OpenAI API）
│   │   ├── task/                 # 任务管理器
│   │   └── transcribe/           # 转录模块（Whisper）
│   ├── capabilities/             # Tauri v2 权限配置
│   ├── icons/                    # 应用图标（多平台）
│   ├── tauri.conf.json           # Tauri 配置
│   └── Cargo.toml                # Rust 依赖
│
├── resources/                    # 资源文件
│   └── default.toml              # 默认配置模板
└── docs/                         # 文档
    └── implementation-plan.md    # 实施方案
```

## 安全特性

- **CSP（内容安全策略）** — 限制脚本、连接来源，防止 XSS 攻击
- **Tauri Capabilities** — 最小权限原则，仅授权必要的系统能力
- **分支保护** — main 分支禁止强制推送，所有变更必须通过 PR
- **凭据隔离** — Cookie 和 API Key 仅在 Rust 后端处理，不暴露给前端
- **本地存储** — 配置和数据库存储在用户目录，不上传云端

## 开发指南

### 环境准备

```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 Node.js（推荐 v18+）
brew install node

# 安装 Tauri CLI
cargo install tauri-cli

# 安装系统依赖
brew install yt-dlp ffmpeg
```

### 开发调试

```bash
npm install
npm run tauri dev
```

### 提交规范

项目使用分支保护，所有变更需通过 PR 合并，且必须通过 Linter 检查：

```bash
# 提交前运行 Linter
npm run lint           # TypeScript/React
npm run lint:rust      # Rust (clippy)

git checkout -b feat/your-feature
# ... 修改代码 ...
git commit -m "feat: 描述你的改动"
git push origin feat/your-feature
# 在 GitHub 上创建 PR
```

### Commit 类型

| 类型 | 说明 |
|------|------|
| `feat` | 新功能 |
| `fix` | Bug 修复 |
| `refactor` | 重构（不影响功能） |
| `style` | 格式调整 |
| `docs` | 文档更新 |
| `build` | 构建系统或依赖变更 |
| `security` | 安全相关 |

## 许可

[MIT](LICENSE)
