// 转录流水线
// CC字幕 → AI字幕 → Whisper 三级降级

use anyhow::Result;

use crate::bilibili::types::Subtitle;
use crate::config::AppConfig;

/// 转录结果
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TranscriptResult {
    pub text: String,
    pub source: String, // "cc" | "ai" | "whisper"
    pub language: Option<String>,
}

/// 执行转录流水线
/// 1. 尝试获取 CC 字幕
/// 2. 尝试获取 AI 字幕
/// 3. 使用 Whisper 转录
pub async fn transcribe_video(
    bvid: &str,
    cid: i64,
    cookie: &str,
    config: &AppConfig,
) -> Result<TranscriptResult> {
    // Step 1: 尝试获取字幕（CC → AI）
    if let Some(subtitle) = crate::bilibili::subtitle::try_get_subtitle(bvid, cid, cookie).await? {
        return Ok(TranscriptResult {
            text: subtitle.content,
            source: subtitle.source,
            language: Some(subtitle.language),
        });
    }

    // Step 2: 使用 Whisper 转录
    // TODO: 下载音频 → 调用 Whisper API → 返回结果
    todo!("实现 Whisper 转录流程")
}
