# Changelog

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
