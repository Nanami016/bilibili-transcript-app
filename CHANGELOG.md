# Changelog

## v1.3.0 (2026-06-26)

### Bug Fixes
- **文件名特殊字符导致下载失败**: 视频标题含 `[doge]` 等方括号字符时，yt-dlp 文件重命名失败。添加 `--restrict-filenames` 选项，将特殊字符替换为下划线

## v1.2.2 (2026-06-26)

### Bug Fixes
- **字幕校验过松**: AI 字幕时长校验从 0.5~2.0 收紧到 0.8~1.5，避免使用不相关视频的字幕（如 182s 字幕对应 98s 视频）
- **ffmpeg 查找失败无日志**: `resolve_ffmpeg_path()` 失败时静默忽略，现改为输出警告日志便于排查

### Features
- **启动日志输出版本号**: App 启动时输出版本号和构建模式（debug/release），便于问题排查

## v1.2.1 (2026-06-26)

### Bug Fixes
- **App bundle 找不到 ffmpeg**: yt-dlp 后处理（m4a→mp3 转换）时找不到 ffmpeg，报错 `ffprobe and ffmpeg not found`。新增 `resolve_ffmpeg_path()` + `--ffmpeg-location` 参数

## v1.2.0 (2026-06-26)

### Bug Fixes
- **App bundle 找不到 yt-dlp**: App 从 `/Applications` 启动时 PATH 不包含 `/opt/homebrew/bin/`，导致 `No such file or directory` 错误。新增 `resolve_ytdlp_path()` 自动搜索常见安装路径
- **下载失败无日志记录**: `mark_failed` 未调用 `log::error!`，导致任务失败信息不写入日志文件
- **yt-dlp stderr 未捕获**: yt-dlp 错误输出直接打到终端而非日志，现改为异步读取 stderr 并记录到日志

### Docs
- README 补充 DMG 安装方式的依赖说明（brew install yt-dlp ffmpeg）

## v1.1.3 (2026-06-26)

### Bug Fixes
- **下载失败无日志记录**: `mark_failed` 未调用 `log::error!`，导致任务失败信息不写入日志文件
- **yt-dlp stderr 未捕获**: yt-dlp 错误输出直接打到终端而非日志，现改为异步读取 stderr 并记录到日志

## v1.1.2 (2026-06-25)

### Bug Fixes
- **yt-dlp 下载/转录死锁**: v1.1.0 将 yt-dlp 的 stderr 改为 piped 但未读取，当 stderr 缓冲区写满时 yt-dlp 阻塞，导致下载和转录挂起。修复为 stderr 改回 inherit

## v1.1.1 (2026-06-25)

### Features
- **日志文件独立记录**: 日志文件名时间戳精确到秒，每次启动 app 生成独立日志文件（如 `2026-06-25_22-41-30.log`），不再同一天混在同一个文件中

## v0.4.0 (2026-06-23)

### Bug Fixes
- **设置页自动保存**: 配置变更后 800ms 自动写入磁盘，无需手动点击保存按钮，避免忘记保存导致后端使用旧配置
- **音频文件检测**: 使用 yt-dlp `--print after_move:filepath` 精确获取下载文件路径，避免 fallback 扫描拿到旧文件导致转录内容错乱
- **ffmpeg 转换失败**: WAV 文件跳过重复转换，避免输入输出同路径导致 "cannot edit existing files in-place" 错误
- **AI 摘要关闭提示**: AI 摘要未启用时在转录文档中显示明确提示，而非待处理占位符
- **收藏夹「加载更多」按钮**: 使用 `gridColumn` 跨列固定居中，不再随视频数量跳动

### Docs
- 本地 Whisper 服务参考链接替换为 [FunAudioLLM-Server](https://github.com/Nanami016/FunAudioLLM-Server)

## v0.3.0 (2026-06-18)

### Features
- 转录弹窗支持语言选择和提示词
- AI 摘要功能修复
- 设置页 Whisper/AI 摘要 API 地址标记必填

## v0.2.0

- 收藏夹分页加载
- 任务中心实时进度追踪
- 历史记录管理

## v0.1.0

- 初始版本
- 视频下载、音频提取
- Whisper 语音转文字
- B站链接解析
