// 配置文件读写
// 存储路径: ~/.bilibili-transcript/config.toml

use anyhow::Result;
use std::fs;
use std::path::PathBuf;

use super::types::AppConfig;

/// 获取配置文件路径
pub fn config_path() -> PathBuf {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    home.join(".bilibili-transcript").join("config.toml")
}

/// 加载配置，如果不存在则返回默认配置
pub fn load_config() -> Result<AppConfig> {
    let path = config_path();
    if path.exists() {
        let content = fs::read_to_string(&path)?;
        // 先尝试直接解析
        match toml::from_str::<AppConfig>(&content) {
            Ok(config) => Ok(config),
            Err(_) => {
                // 解析失败，可能是旧格式，尝试手动迁移
                log::warn!("配置文件格式过旧，尝试迁移...");
                let mut config = AppConfig::default();
                if let Ok(old) = toml::from_str::<toml::Value>(&content) {
                    // 提取旧字段
                    if let Some(bili) = old.get("bilibili") {
                        if let Some(cookie) = bili.get("cookie").and_then(|v| v.as_str()) {
                            config.bilibili.cookie = cookie.to_string();
                        }
                        if let Some(dir) = bili.get("output_dir").and_then(|v| v.as_str()) {
                            // 旧的 output_dir 迁移到 transcript_dir
                            config.bilibili.transcript_dir = dir.to_string();
                            config.bilibili.video_dir = format!("{}/video", dir);
                            config.bilibili.audio_dir = format!("{}/audio", dir);
                        }
                    }
                    if let Some(whisper) = old.get("whisper") {
                        if let Some(mode) = whisper.get("mode").and_then(|v| v.as_str()) {
                            config.whisper.mode = mode.to_string();
                        }
                        if let Some(url) = whisper.get("api_url").and_then(|v| v.as_str()) {
                            config.whisper.api_url = url.to_string();
                        }
                        if let Some(key) = whisper.get("api_key").and_then(|v| v.as_str()) {
                            if !key.is_empty() {
                                config.whisper.api_key = Some(key.to_string());
                            }
                        }
                        if let Some(model) = whisper.get("model").and_then(|v| v.as_str()) {
                            config.whisper.model = model.to_string();
                        }
                    }
                    if let Some(ai) = old.get("ai_summary") {
                        if let Some(enabled) = ai.get("enabled").and_then(|v| v.as_bool()) {
                            config.ai_summary.enabled = enabled;
                        }
                        if let Some(url) = ai.get("api_url").and_then(|v| v.as_str()) {
                            config.ai_summary.api_url = url.to_string();
                        }
                        if let Some(key) = ai.get("api_key").and_then(|v| v.as_str()) {
                            if !key.is_empty() {
                                config.ai_summary.api_key = Some(key.to_string());
                            }
                        }
                        if let Some(model) = ai.get("model").and_then(|v| v.as_str()) {
                            config.ai_summary.model = model.to_string();
                        }
                    }
                }
                // 保存迁移后的配置
                let _ = save_config(&config);
                log::info!("配置迁移完成");
                Ok(config)
            }
        }
    } else {
        Ok(AppConfig::default())
    }
}

/// 保存配置到文件
pub fn save_config(config: &AppConfig) -> Result<()> {
    let path = config_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let content = toml::to_string_pretty(config)?;
    fs::write(&path, content)?;
    Ok(())
}
