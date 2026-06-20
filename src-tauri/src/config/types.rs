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
    /// 模式: "openai" | "local"
    pub mode: String,
    /// API 地址
    pub api_url: String,
    /// API Key（可选，本地模式可留空）
    pub api_key: Option<String>,
    /// 模型名称
    pub model: String,
}

/// B站配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BilibiliConfig {
    /// Cookie 字符串
    pub cookie: String,
    /// 输出目录
    pub output_dir: String,
}

/// AI 摘要配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiSummaryConfig {
    /// 是否启用
    pub enabled: bool,
    /// API 地址
    pub api_url: String,
    /// API Key
    pub api_key: Option<String>,
    /// 模型名称
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
                output_dir: "~/bilibili-voice-only".to_string(),
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
