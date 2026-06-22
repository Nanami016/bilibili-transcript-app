// 转录流水线
// CC字幕 → AI字幕 → Whisper 三级降级

use anyhow::Result;

use crate::config::AppConfig;
use crate::download::audio;
use crate::transcribe::whisper::{OpenAIWhisperClient, WhisperClient};

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
    url: &str,
    bvid: &str,
    cid: i64,
    cookie: &str,
    config: &AppConfig,
    whisper_prompt_override: Option<&str>,
    language: Option<&str>,
) -> Result<TranscriptResult> {
    log::info!("开始转录: bvid={}, cid={}", bvid, cid);

    // Step 1: 尝试获取字幕（CC → AI）
    match crate::bilibili::subtitle::try_get_subtitle(bvid, cid, cookie).await {
        Ok(Some(subtitle)) => {
            log::info!("获取到字幕: source={}", subtitle.source);
            return Ok(TranscriptResult {
                text: subtitle.content,
                source: subtitle.source,
                language: Some(subtitle.language),
            });
        }
        Ok(None) => {
            log::info!("未找到字幕，将使用 Whisper 转录");
        }
        Err(e) => {
            log::warn!("获取字幕失败: {}，将使用 Whisper 转录", e);
        }
    }

    // Step 2: 使用 Whisper 转录
    // 检查 Whisper 配置
    if config.whisper.api_key.is_none() && config.whisper.mode == "openai" {
        anyhow::bail!("未配置 Whisper API Key，请在设置中配置");
    }

    // 下载音频
    let output_dir = std::path::PathBuf::from(
        shellexpand::tilde(&config.bilibili.audio_dir).to_string()
    );

    log::info!("正在下载音频...");
    let audio_path = audio::extract_audio(url, &output_dir, cookie).await?;
    log::info!("音频下载完成: {:?}", audio_path);

    // 创建 Whisper 客户端（优先使用按次传入的 prompt）
    let whisper_prompt = whisper_prompt_override
        .filter(|p| !p.is_empty())
        .map(|p| p.to_string())
        .or_else(|| if config.whisper.prompt.is_empty() { None } else { Some(config.whisper.prompt.clone()) });
    let whisper_lang = language.filter(|l| !l.is_empty()).map(|l| l.to_string());
    let whisper_client = OpenAIWhisperClient::new(
        config.whisper.api_url.clone(),
        config.whisper.api_key.clone(),
        config.whisper.model.clone(),
        whisper_prompt,
        whisper_lang,
    );

    // 调用 Whisper API
    log::info!("正在调用 Whisper API...");
    let text = whisper_client.transcribe(&audio_path).await?;

    // 清理临时音频文件
    let _ = std::fs::remove_file(&audio_path);

    log::info!("Whisper 转录完成，文本长度: {}", text.len());

    Ok(TranscriptResult {
        text,
        source: "whisper".to_string(),
        language: None,
    })
}
