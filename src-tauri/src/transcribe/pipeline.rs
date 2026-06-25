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

/// 进度回调: (progress: 0.0~100.0, speed: String, eta: String)
pub type ProgressCallback = Box<dyn Fn(f64, String, String) + Send + 'static>;

/// 执行转录流水线
/// 1. 尝试获取 CC 字幕
/// 2. 尝试获取 AI 字幕
/// 3. 使用 Whisper 转录
#[allow(clippy::too_many_arguments)]
pub async fn transcribe_video(
    url: &str,
    bvid: &str,
    cid: i64,
    cookie: &str,
    config: &AppConfig,
    whisper_prompt_override: Option<&str>,
    language: Option<&str>,
    on_progress: Option<ProgressCallback>,
    video_duration_secs: Option<i64>,
    skip_bilibili_subtitle: bool,
) -> Result<TranscriptResult> {
    log::info!("开始转录: bvid={}, cid={}, skip_subtitle={}", bvid, cid, skip_bilibili_subtitle);

    // Step 1: 尝试获取字幕（CC → AI），如果用户选择跳过则直接进入 Whisper
    if skip_bilibili_subtitle {
        log::info!("用户选择跳过 B站字幕，直接使用 Whisper 转录");
    } else {
    match crate::bilibili::subtitle::try_get_subtitle(bvid, cid, cookie).await {
        Ok(Some(subtitle)) => {
            // 校验字幕时长与视频时长是否匹配
            if let Some(sub_dur) = subtitle.duration_secs {
                let video_dur = video_duration_secs.unwrap_or(0) as f64;
                // 仅在已知视频时长时做校验
                if video_dur > 0.0 {
                    let ratio = sub_dur / video_dur;
                    if !(0.5..=2.0).contains(&ratio) {
                        log::warn!(
                            "字幕时长({:.1}s)与视频时长({:.1}s)差异过大(ratio={:.2})，可能不是同一视频，降级到 Whisper",
                            sub_dur, video_dur, ratio
                        );
                        // 不返回字幕结果，继续走 Whisper 流程
                    } else {
                        log::info!("获取到字幕: source={}, 时长校验通过({:.1}s vs {:.1}s)", subtitle.source, sub_dur, video_dur);
                        return Ok(TranscriptResult {
                            text: subtitle.content,
                            source: subtitle.source,
                            language: Some(subtitle.language),
                        });
                    }
                } else {
                    log::info!("获取到字幕: source={} (未知视频时长，跳过时长校验)", subtitle.source);
                    return Ok(TranscriptResult {
                        text: subtitle.content,
                        source: subtitle.source,
                        language: Some(subtitle.language),
                    });
                }
            } else {
                log::info!("获取到字幕: source={} (字幕无时间戳，跳过时长校验)", subtitle.source);
                return Ok(TranscriptResult {
                    text: subtitle.content,
                    source: subtitle.source,
                    language: Some(subtitle.language),
                });
            }
        }
        Ok(None) => {
            log::info!("未找到字幕，将使用 Whisper 转录");
        }
        Err(e) => {
            log::warn!("获取字幕失败: {}，将使用 Whisper 转录", e);
        }
    }
    } // end of skip_bilibili_subtitle else block

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

    // 构建进度回调，映射到 10%~80% 范围
    let audio_progress: Option<audio::ProgressCallback> = on_progress.map(|cb| {
        Box::new(move |pct: f64, speed: String, eta: String| {
            // yt-dlp 下载进度 0~100% 映射到 10%~80%
            let mapped = 10.0 + pct * 0.7;
            cb(mapped, speed, eta);
        }) as audio::ProgressCallback
    });

    let audio_path = audio::extract_audio(url, &output_dir, cookie, audio_progress).await?;
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
