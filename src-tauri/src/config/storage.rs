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
        let config: AppConfig = toml::from_str(&content)?;
        Ok(config)
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
