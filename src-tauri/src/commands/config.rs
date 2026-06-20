// 配置相关命令

use tauri::command;

use crate::config::AppConfig;

/// 获取当前配置
#[command]
pub async fn get_config() -> Result<AppConfig, String> {
    crate::config::storage::load_config()
        .map_err(|e| e.to_string())
}

/// 更新配置
#[command]
pub async fn update_config(config: AppConfig) -> Result<(), String> {
    crate::config::storage::save_config(&config)
        .map_err(|e| e.to_string())
}
