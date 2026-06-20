# Bilibili Transcript App

B站视频转录 macOS 原生应用 — 支持字幕获取、Whisper 语音转文字、AI 摘要。

## 功能

- 🎬 **视频下载** — 选择分辨率下载视频，或仅下载音频
- 📝 **字幕获取** — CC 字幕 → AI 字幕，自动降级
- 🎤 **语音转文字** — Whisper 支持（OpenAI API / 本地 REST API）
- 🤖 **AI 摘要** — 可选，支持 OpenAI 兼容 API
- ⭐ **收藏夹管理** — 登录后自动列出收藏夹，批量转录

## 前置依赖

```bash
# 必需
brew install yt-dlp ffmpeg

# 可选（如果使用本地 Whisper）
# 安装 whisper.cpp 或其他兼容 OpenAI 格式的 Whisper 服务
```

## 开发

```bash
# 安装前端依赖
npm install

# 启动开发服务器
npm run tauri dev

# 构建应用
npm run tauri build
```

## 配置

应用配置存储在 `~/.bilibili-transcript/config.toml`：

```toml
[whisper]
mode = "openai"              # "openai" | "local"
api_url = "https://api.openai.com/v1/audio/transcriptions"
api_key = "sk-xxx"           # 可选，本地模式可留空
model = "whisper-1"

[bilibili]
cookie = ""                  # B站 Cookie
output_dir = "~/bilibili-voice-only"

[ai_summary]
enabled = true
api_url = "https://api.openai.com/v1/chat/completions"
api_key = ""
model = "gpt-4o-mini"
```

## 许可

MIT
