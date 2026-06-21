// 配置数据结构

use serde::{Deserialize, Serialize};

/// 应用主配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub whisper: WhisperConfig,
    pub bilibili: BilibiliConfig,
    pub ai_summary: AiSummaryConfig,
}

/// Whisper 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhisperConfig {
    pub mode: String,
    pub api_url: String,
    pub api_key: Option<String>,
    pub model: String,
}

/// B站配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BilibiliConfig {
    pub cookie: String,
    /// 视频下载目录
    pub video_dir: String,
    /// 音频下载目录
    pub audio_dir: String,
    /// 转录结果目录
    pub transcript_dir: String,
    /// AI 分析结果目录
    #[serde(default)]
    pub ai_analysis_dir: String,
    /// 兼容旧配置：统一输出目录
    #[serde(default)]
    pub output_dir: String,
}

/// AI 摘要配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiSummaryConfig {
    pub enabled: bool,
    pub api_url: String,
    pub api_key: Option<String>,
    pub model: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            whisper: WhisperConfig {
                mode: "openai".to_string(),
                api_url: "https://api.openai.com/v1/audio/transcriptions".to_string(),
                api_key: None,
                model: "whisper-1".to_string(),
            },
            bilibili: BilibiliConfig {
                cookie: String::new(),
                video_dir: "~/Downloads/bilibili-transcript-app/bilibili-video".to_string(),
                audio_dir: "~/Downloads/bilibili-transcript-app/bilibili-audio".to_string(),
                transcript_dir: "~/Downloads/bilibili-transcript-app/bilibili-transfer".to_string(),
                ai_analysis_dir: "~/Downloads/bilibili-transcript-app/bilibili-ai-analysis".to_string(),
                output_dir: String::new(),
            },
            ai_summary: AiSummaryConfig {
                enabled: false,
                api_url: "https://api.openai.com/v1/chat/completions".to_string(),
                api_key: None,
                model: "gpt-4o-mini".to_string(),
            },
        }
    }
}
