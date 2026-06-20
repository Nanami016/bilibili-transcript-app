// Cookie 管理
// 导入、导出、持久化

use anyhow::Result;

use crate::config::storage;

/// 保存 Cookie 到配置文件
pub fn save_cookie(cookie: &str) -> Result<()> {
    let mut config = storage::load_config()?;
    config.bilibili.cookie = cookie.to_string();
    storage::save_config(&config)?;
    Ok(())
}

/// 获取当前 Cookie
pub fn get_cookie() -> Result<String> {
    let config = storage::load_config()?;
    Ok(config.bilibili.cookie)
}

/// 检查 Cookie 是否已配置
pub fn has_cookie() -> bool {
    if let Ok(config) = storage::load_config() {
        !config.bilibili.cookie.is_empty()
    } else {
        false
    }
}
