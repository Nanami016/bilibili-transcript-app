# Bilibili Transcript App - 项目实施方案

## 一、项目概述

### 1.1 项目目标

将 bilibili-auto-transcript（Claude Code skill）改造为 macOS 原生应用，实现：
- B站视频下载和音频提取
- 字幕获取（CC → AI 降级）
- Whisper 语音转文字（支持远程 API 和本地 REST API）
- AI 摘要生成（可选）
- 收藏夹管理和批量转录

### 1.2 技术选型

| 层级 | 技术方案 | 说明 |
|------|---------|------|
| **GUI 框架** | Tauri 2.0 | Rust 后端 + Web 前端，原生性能 |
| **前端** | React + TypeScript | 组件化开发，生态成熟 |
| **后端** | Rust | 高性能、内存安全 |
| **数据库** | SQLite (rusqlite) | 轻量级嵌入式数据库 |
| **HTTP 客户端** | reqwest | 异步 HTTP 请求 |
| **视频下载** | yt-dlp (CLI) | 外部依赖，用户自行安装 |
| **音频处理** | ffmpeg (CLI) | 外部依赖，用户自行安装 |

### 1.3 目标用户

- 会使用 Terminal 和 brew 安装基本依赖
- 需要一个简洁的 GUI 来操作 B站视频转录
- 不想手动执行命令行脚本

---

## 二、项目结构

```
bilibili-transcript-app/
├── Cargo.toml                          # Rust workspace 配置
├── README.md
├── LICENSE
├── .gitignore
│
├── src-tauri/                          # Tauri 后端（Rust）
│   ├── Cargo.toml
│   ├── tauri.conf.json                 # Tauri 配置
│   ├── build.rs
│   ├── icons/                          # 应用图标
│   │   └── icon.icns
│   │
│   └── src/
│       ├── main.rs                     # 入口，初始化 Tauri
│       ├── lib.rs                      # 模块导出
│       │
│       ├── config/                     # 配置管理
│       │   ├── mod.rs
│       │   ├── types.rs                # 配置数据结构
│       │   └── storage.rs              # 读写配置文件
│       │
│       ├── bilibili/                   # B站相关
│       │   ├── mod.rs
│       │   ├── api.rs                  # B站 API 调用
│       │   ├── cookie.rs               # Cookie 管理
│       │   ├── subtitle.rs             # 字幕解析
│       │   └── types.rs                # B站数据结构
│       │
│       ├── download/                   # 下载模块
│       │   ├── mod.rs
│       │   ├── video.rs                # yt-dlp 调用
│       │   └── audio.rs                # 音频提取
│       │
│       ├── transcribe/                 # 转录模块
│       │   ├── mod.rs
│       │   ├── whisper.rs              # Whisper API 客户端
│       │   └── pipeline.rs             # 转录流水线
│       │
│       ├── summary/                    # AI 摘要模块
│       │   ├── mod.rs
│       │   └── openai.rs               # OpenAI 兼容 API
│       │
│       ├── storage/                    # 数据存储
│       │   ├── mod.rs
│       │   ├── database.rs             # SQLite 数据库
│       │   └── file.rs                 # TXT 文件渲染
│       │
│       └── commands/                   # Tauri 命令
│           ├── mod.rs
│           ├── video.rs                # 视频相关命令
│           ├── favorite.rs             # 收藏夹相关命令
│           ├── transcribe.rs           # 转录相关命令
│           ├── config.rs               # 配置相关命令
│           └── cookie.rs               # Cookie 相关命令
│
├── src/                                # 前端（React + TypeScript）
│   ├── main.tsx                        # 前端入口
│   ├── App.tsx                         # 主应用组件
│   │
│   ├── components/                     # UI 组件
│   │   ├── Layout.tsx                  # 布局框架
│   │   ├── Sidebar.tsx                 # 左侧栏（含运行日志导航）
│   │   ├── InputBar.tsx                # 顶部输入框
│   │   ├── VideoCard.tsx               # 视频卡片（含封面）
│   │   ├── LogViewer.tsx               # 日志查看器（设置页内嵌）
│   │   └── common/                     # 通用组件
│   │       ├── Button.tsx
│   │       ├── Spinner.tsx
│   │       └── Toast.tsx
│   │
│   ├── pages/                          # 页面
│   │   ├── Home.tsx                    # 首页（视频解析+操作）
│   │   ├── Favorite.tsx                # 收藏夹页
│   │   ├── Logs.tsx                    # 运行日志页
│   │   └── Settings.tsx                # 设置页
│   │
│   ├── lib/                            # 工具函数
│   │   └── tauri.ts                    # Tauri API 封装
│   │
│   └── styles/                         # 样式
│       └── global.css
│
└── resources/                          # 静态资源
    └── default.toml                    # 默认配置模板
```

---

## 三、核心模块实现方案

### 3.1 配置管理 (`config/`)

**存储路径**: `~/.bilibili-transcript/config.toml`

```toml
[whisper]
mode = "openai"              # "openai" | "local"
api_url = "https://api.openai.com/v1/audio/transcriptions"
api_key = "sk-xxx"           # 可选，本地模式可留空
model = "whisper-1"

[bilibili]
cookie = ""                  # B站 Cookie 字符串
video_dir = "~/Downloads/bilibili-transcript-app/bilibili-video"      # 视频下载目录
audio_dir = "~/Downloads/bilibili-transcript-app/bilibili-audio"      # 音频下载目录
transcript_dir = "~/Downloads/bilibili-transcript-app/bilibili-transfer"  # 转录结果目录
ai_analysis_dir = "~/Downloads/bilibili-transcript-app/bilibili-ai-analysis"  # AI 分析结果目录

[ai_summary]
enabled = true
api_url = "https://api.openai.com/v1/chat/completions"
api_key = ""
model = "gpt-4o-mini"
```

**实现要点**:
- 使用 `toml` crate 进行序列化/反序列化
- 首次运行时自动创建默认配置文件
- 配置变更后立即持久化到文件

### 3.2 B站 API (`bilibili/`)

**核心 API**:

| API | 端点 | 说明 |
|-----|------|------|
| 收藏夹列表 | `GET /x/v3/fav/folder/created?up_mid={uid}` | 获取用户所有收藏夹 |
| 收藏夹内容 | `GET /x/v3/fav/resource/list?media_id={fid}&ps=20&pn=1` | 获取收藏夹中的视频 |
| 视频信息 | `GET /x/web-interface/view?bvid={bvid}` | 获取视频详细信息 |
| 字幕列表 | `GET /x/player/v2?bvid={bvid}&cid={cid}` | 获取视频字幕列表 |

**Cookie 管理**:
- 支持从浏览器导出 Cookie 字符串
- 存储在配置文件中（不上传到 GitHub）
- 定期检查 Cookie 有效性

### 3.3 Whisper 客户端 (`transcribe/whisper.rs`)

**两种模式**:

| 模式 | 说明 | API Key |
|------|------|---------|
| OpenAI 格式 | 兼容 OpenAI Whisper API | 必需 |
| 本地 REST API | 兼容 whisper.cpp server 等 | 可选 |

**统一接口**:

```rust
#[async_trait]
pub trait WhisperClient {
    async fn transcribe(&self, audio_path: &Path) -> Result<String>;
    async fn test_connection(&self) -> Result<bool>;
}
```

**请求格式**:
- `POST` 请求
- `multipart/form-data` 格式
- 字段: `file` (音频文件) + `model` (模型名称)

**响应格式**:
```json
{
  "text": "转录文本内容"
}
```

### 3.4 转录流水线 (`transcribe/pipeline.rs`)

**三级降级策略**:

```
1. CC 字幕 (人工字幕，100% 准确)
   ↓ 失败
2. AI 字幕 (B站 AI 生成，85-90% 准确)
   ↓ 失败
3. Whisper 转录 (本地/远程，准确率取决于模型)
   ↓ 失败
4. 返回错误
```

**实现逻辑**:

```rust
pub async fn transcribe_video(
    bvid: &str,
    cid: i64,
    cookie: &str,
    config: &AppConfig,
) -> Result<TranscriptResult> {
    // Step 1: 尝试获取字幕（CC → AI）
    if let Some(subtitle) = try_get_subtitle(bvid, cid, cookie).await? {
        return Ok(TranscriptResult::from_subtitle(subtitle));
    }

    // Step 2: 使用 Whisper 转录
    let audio_path = extract_audio(url).await?;
    let text = whisper_client.transcribe(&audio_path).await?;
    Ok(TranscriptResult::from_whisper(text))
}
```

### 3.5 数据存储 (`storage/`)

**SQLite 数据库** (`~/.bilibili-transcript/transcripts.db`):

```sql
CREATE TABLE transcripts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    bvid TEXT UNIQUE NOT NULL,
    url TEXT,
    title TEXT,
    author TEXT,
    duration TEXT,
    upload_date TEXT,
    transcript_source TEXT,
    transcript_text TEXT,
    summary TEXT,
    status TEXT DEFAULT 'transcribed',
    created_at TEXT DEFAULT (datetime('now', 'localtime')),
    updated_at TEXT DEFAULT (datetime('now', 'localtime'))
);
```

**TXT 文件渲染** (`~/bilibili-voice-only/`):

```
~/bilibili-voice-only/
├── 2024/
│   ├── 01/
│   │   ├── 视频标题_UP主名_2024-01-15_BVxxx.txt
│   │   └── ...
│   └── 06/
│       └── ...
└── 2025/
    └── ...
```

### 3.6 Tauri 命令 (`commands/`)

**前端调用接口**:

```rust
// 视频相关
#[tauri::command]
async fn parse_video(url: String) -> Result<VideoInfo, String>;

#[tauri::command]
async fn get_video_formats(url: String) -> Result<Vec<VideoFormat>, String>;

#[tauri::command]
async fn download_video(url: String, format_id: String) -> Result<String, String>;

#[tauri::command]
async fn download_audio(url: String) -> Result<String, String>;

// 转录相关
#[tauri::command]
async fn transcribe(url: String) -> Result<TranscriptResult, String>;

#[tauri::command]
async fn test_whisper_connection() -> Result<bool, String>;

// 收藏夹相关
#[tauri::command]
async fn get_favorites() -> Result<Vec<FavoriteFolder>, String>;

#[tauri::command]
async fn get_favorite_videos(media_id: String) -> Result<Vec<VideoInfo>, String>;

// 配置相关
#[tauri::command]
async fn get_config() -> Result<AppConfig, String>;

#[tauri::command]
async fn update_config(config: AppConfig) -> Result<(), String>;

// Cookie 相关
#[tauri::command]
async fn import_cookie(cookie: String) -> Result<(), String>;

#[tauri::command]
async fn get_cookie_status() -> Result<bool, String>;
```

---

## 四、依赖清单

### 4.1 Rust 后端 (`src-tauri/Cargo.toml`)

```toml
[dependencies]
tauri = { version = "2", features = ["shell-open"] }
tauri-plugin-shell = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12", features = ["json", "multipart"] }
rusqlite = { version = "0.31", features = ["bundled"] }
toml = "0.8"
dirs = "5"
anyhow = "1"
thiserror = "1"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1", features = ["v4"] }
log = "0.4"
env_logger = "0.11"
```

### 4.2 前端 (`package.json`)

```json
{
  "dependencies": {
    "@tauri-apps/api": "^2.0.0",
    "react": "^18.3.0",
    "react-dom": "^18.3.0",
    "react-router-dom": "^6.26.0",
    "lucide-react": "^0.400.0"
  },
  "devDependencies": {
    "@tauri-apps/cli": "^2.0.0",
    "@types/react": "^18.3.0",
    "@types/react-dom": "^18.3.0",
    "@vitejs/plugin-react": "^4.3.0",
    "typescript": "^5.5.0",
    "vite": "^5.4.0"
  }
}
```

### 4.3 系统依赖（用户需自行安装）

```bash
# 必需
brew install yt-dlp ffmpeg

# 可选（如果使用本地 Whisper）
# 安装 whisper.cpp 或其他兼容 OpenAI 格式的 Whisper 服务
```

---

## 五、UI 设计

### 5.1 整体布局

```
┌─────────────────────────────────────────────────────────────┐
│  📺 Bili Transcript                                          │
├──────────┬──────────────────────────────────────────────────┤
│          │ ┌──────────────────────────────────────────────┐ │
│  首页    │ │  输入 B站视频链接...                    [解析] │ │
│          │ └──────────────────────────────────────────────┘ │
│  收藏夹  │                                                  │
│          │ ┌──────────────────────────────────────────────┐ │
│  ─────── │ │  视频封面                                     │ │
│  任务中心 │ │  视频标题                                     │ │
│  视频下载 │ │  UP主 | 时长                                  │ │
│  音频下载 │ │  [下载视频] [下载音频] [语音转录] [AI摘要]     │ │
│  AI分析   │ └──────────────────────────────────────────────┘ │
│  音频转录 │                                                  │
│  ─────── │                                                  │
│  运行日志 │                                                  │
│  设置    │                                                  │
└──────────┴──────────────────────────────────────────────────┘
```

### 5.2 任务中心页面设计（视频下载 / 音频下载 / AI 分析 / 音频转录）

每个任务中心页面包含两个区域：

```
┌──────────────────────────────────────────────────────────────┐
│  视频下载                                              [清空] │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  ▶ 当前任务                                                   │
│  ┌──────────────────────────────────────────────────────────┐ │
│  │ 📹 视频标题_xxx                    72%  ██░░░░  2.3MB/s  │ │
│  │    预计剩余: 1分30秒                                      │ │
│  └──────────────────────────────────────────────────────────┘ │
│                                                              │
│  📋 历史记录                                                   │
│  ┌──────────────────────────────────────────────────────────┐ │
│  │ ✅ 视频A  UP主  2024-01-15 14:30  128.5MB                │ │
│  │ ✅ 视频B  UP主  2024-01-15 13:20   95.2MB                │ │
│  │ ❌ 视频C  UP主  2024-01-15 12:10  下载失败               │ │
│  └──────────────────────────────────────────────────────────┘ │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

| 区域 | 说明 |
|------|------|
| **当前任务** | 实时进度条、下载/转换速度、预计剩余时间 |
| **历史记录** | 按时间倒序展示已完成/失败的任务，支持打开文件/目录 |

### 5.3 页面说明

| 页面 | 功能 |
|------|------|
| **首页** | 输入视频链接、解析视频信息（含封面）、执行操作 |
| **收藏夹** | 显示收藏夹列表、收藏夹视频列表 |
| **视频下载** | 视频下载任务的实时进度 + 历史记录 |
| **音频下载** | 音频下载任务的实时进度 + 历史记录 |
| **AI 分析** | AI 摘要生成任务的实时进度 + 历史记录 |
| **音频转录** | 语音转录任务的实时进度 + 历史记录 |
| **运行日志** | 查看 DEBUG 级别运行日志，支持筛选/清空/自动滚动 |
| **设置** | B站 Cookie（浏览器读取/手动导入）、Whisper 配置、AI 摘要、三路径配置 |

### 5.4 视频卡片功能

| 按钮 | 功能 | 说明 |
|------|------|------|
| 下载视频 | 弹出分辨率选择下拉框 | 调用 yt-dlp 下载 |
| 下载音频 | 直接下载音频文件 | 调用 yt-dlp -x |
| 语音转录 | 执行三级降级转录 | CC → AI → Whisper |
| AI 摘要 | 生成视频摘要 | 调用 OpenAI 兼容 API |

---

## 六、配置文件结构

### 6.1 应用配置目录

**路径**: `~/.bilibili-transcript/`

```
~/.bilibili-transcript/
├── config.toml                 # 主配置文件
├── transcripts.db              # SQLite 数据库
├── logs/                       # 日志目录
│   └── app.log
└── cache/                      # 缓存（临时音频文件等）
    └── ...
```

### 6.2 输出目录

```
~/Downloads/bilibili-transcript-app/
├── bilibili-video/             # 视频下载目录
│   └── 视频标题.mp4
├── bilibili-audio/             # 音频下载目录
│   └── 视频标题.mp3
├── bilibili-transfer/          # 转录结果目录
│   └── 2024/
│       └── 01/
│           └── 视频标题_UP主名_2024-01-15_BVxxx.txt
└── bilibili-ai-analysis/       # AI 分析结果目录
    └── ...
```

---

## 七、实施步骤

### Phase 1: 项目骨架搭建 ✅

- [x] 创建项目目录结构
- [x] 配置 Tauri + React + TypeScript
- [x] 实现基础 UI 框架（Layout、Sidebar、页面路由）
- [x] 确保项目能正常启动和运行

### Phase 2: 配置管理 + B站 API 集成 ✅

- [x] 实现配置文件读写（config.toml）
- [x] 实现 B站 API 调用（视频信息、字幕列表）
- [x] 实现 Cookie 管理（从浏览器读取、手动导入、存储）
- [x] 实现收藏夹 API 调用
- [x] 配置文件格式自动迁移（旧格式 → 新格式）

### Phase 3: 视频解析 + 格式列表 + 下载功能 ✅

- [x] 实现视频信息解析（BVid 提取、API 调用）
- [x] 实现格式列表获取（yt-dlp -F）
- [x] 实现视频下载（yt-dlp + Cookie 文件传递）
- [x] 实现音频提取（yt-dlp -x + ffmpeg 转 WAV）

### Phase 4: 字幕获取 + Whisper 集成 ✅

- [x] 实现字幕获取（CC → AI 降级）
- [x] 实现 Whisper API 客户端（OpenAI 格式 + 本地 REST API）
- [x] 实现转录流水线（三级降级：CC → AI → Whisper）
- [x] 实现 Whisper 连接测试功能

### Phase 5: AI 摘要 + 数据库 + TXT 渲染 ✅

- [x] 实现 AI 摘要生成（OpenAI 兼容 API）
- [x] 实现 SQLite 数据库（创建表、CRUD 操作）
- [x] 实现 TXT 文件渲染输出
- [x] 实现按年月组织目录

### Phase 6: 收藏夹功能 + 设置页面 ✅

- [x] 实现收藏夹列表显示
- [x] 实现收藏夹视频列表
- [x] 收藏夹视频封面通过 Rust 代理加载（避免 WebView 外部图片限制）
- [x] 收藏夹视频支持下载视频、下载音频、语音转录、AI 摘要四个功能
- [x] 完善设置页面（Whisper、Cookie、AI 摘要、三路径配置）
- [ ] 实现批量转录功能
- [ ] 支持读取隐藏/私密收藏夹（当前仅显示公开收藏夹，参见 Issue #6）

### Phase 7: UI 美化 + 错误处理 + 打包分发

- [ ] UI 美化和交互优化
- [x] 完善错误处理和用户提示（Toast 通知、运行日志）
- [ ] 测试所有功能
- [ ] 打包为 macOS .dmg
- [ ] 编写 GitHub Release 说明

### Phase 8: 任务中心 — 左侧导航 + 历史记录 + 进度追踪 ✅

- [x] **左侧导航栏新增 4 个页面入口**：视频下载、音频下载、AI 分析、音频转录
- [x] **每个页面包含历史记录列表**：展示已完成的任务（标题、时间、状态、文件大小）
- [x] **每个页面包含实时进度区域**：显示当前正在进行的任务进度条（百分比、速度、预计剩余时间）
- [x] **后端任务队列系统**：支持并发任务管理、状态更新事件推送（Tauri event）
- [x] **数据库新增 tasks 表**：记录所有任务的历史（类型、URL、标题、状态、进度、开始/结束时间、输出路径）
- [x] **全局进度提示框**：右上角 TaskToast 组件，实时显示任务进度，完成后自动消失
- [x] **视频清晰度选择**：下拉框显示 360P/480P/720P/1080P，普通用户受限于 480P
- [x] **打开文件夹按钮**：每个任务页面可直接打开对应输出目录
- [x] **默认路径修改**：统一为 `~/Downloads/bilibili-transcript-app/` 子目录
- [x] **AI 分析路径配置**：设置页面新增 `ai_analysis_dir` 配置项

### Phase 8 实现差异（与原计划不同之处）

| 原计划 | 实际实现 | 原因 |
|--------|----------|------|
| 首页操作直接调用下载/转录 | 首页操作改为启动异步任务 | 需要走 TaskManager 才能记录历史和推送进度 |
| 视频下载无清晰度选择 | 新增下拉框选择 360P/480P/720P/1080P | 用户需要选择画质 |
| 格式列表通过 yt-dlp 获取 | yt-dlp + Cookie 传递，普通用户最高 480P | B站限制：720P+ 需大会员 |
| 输出路径 `~/Downloads/bilibili-download/` | `~/Downloads/bilibili-transcript-app/` 子目录 | 用户要求统一父目录 |
| 无 AI 分析独立路径 | 新增 `ai_analysis_dir` 配置 | AI 分析结果需要独立存储 |

### 额外实现的功能（不在原计划中）

- [x] **运行日志系统** — DEBUG 级别日志捕获，侧边栏独立页面查看
- [x] **从浏览器读取 Cookie** — 支持 Chrome/Safari/Firefox/Edge
- [x] **视频封面代理** — 通过 Rust 后端代理获取封面图片（解决 WebView 外部图片限制）
- [x] **四路径配置** — 视频/音频/转录/AI分析独立输出目录
- [x] **Toast 通知** — 自动消失 + 手动关闭
- [x] **全局 TaskToast** — 右上角进度提示框，支持多任务同时显示

---

## 八、关键技术点

### 8.1 yt-dlp 调用

通过 `std::process::Command` 调用系统安装的 yt-dlp：

```rust
use std::process::Command;

let output = Command::new("yt-dlp")
    .args(["--cookies-from-browser", "chrome", "--dump-json", &url])
    .output()?;
```

### 8.2 ffmpeg 调用

音频格式转换：

```rust
Command::new("ffmpeg")
    .args(["-y", "-i", input_path, "-ar", "16000", "-ac", "1", output_path])
    .output()?;
```

### 8.3 Whisper API 请求

```rust
let form = reqwest::multipart::Form::new()
    .part("file", file_part)
    .text("model", model);

let resp = client
    .post(&api_url)
    .header("Authorization", format!("Bearer {}", api_key))
    .multipart(form)
    .send()
    .await?;
```

### 8.4 Cookie 安全存储

Cookie 存储在本地配置文件中，不上传到 GitHub：

```gitignore
# .gitignore
config.toml
*.db
```

---

## 九、注意事项

1. **外部依赖**: yt-dlp 和 ffmpeg 需要用户自行安装（`brew install yt-dlp ffmpeg`）
2. **Cookie 获取**: SESSDATA 是 HttpOnly Cookie，WebView 无法读取。推荐使用「从浏览器读取」功能（需关闭浏览器）或手动导入
3. **Cookie 有效期**: B站 Cookie 约 30 天过期，需要定期更新
4. **Whisper 模型**: 首次使用远程 Whisper 时需要确保 API 可用
5. **文件权限**: macOS 可能需要授权访问文件系统
6. **网络安全**: 调用 B站 API 和 yt-dlp 时需要正确的 User-Agent 和 Referer
7. **封面图片**: 通过 Rust 后端代理获取（WebView 外部图片加载受限）

---

## 十、参考资料

- [Tauri 2.0 文档](https://v2.tauri.app/)
- [B站 API 文档](https://github.com/SocialSisterYi/bilibili-API-collect)
- [yt-dlp 文档](https://github.com/yt-dlp/yt-dlp)
- [OpenAI Whisper API](https://platform.openai.com/docs/guides/speech-to-text)
- [whisper.cpp](https://github.com/ggerganov/whisper.cpp)
