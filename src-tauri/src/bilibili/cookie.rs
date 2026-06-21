// Cookie 管理
// 导入、导出、持久化

use anyhow::Result;

use crate::config::storage;

/// 保存 Cookie 到配置文件
pub fn save_cookie(cookie: &str) -> Result<()> {
    log::info!("保存 Cookie: 长度={}", cookie.len());
    log::debug!("Cookie 内容: {}", &cookie[..cookie.len().min(100)]);
    let mut config = storage::load_config()?;
    config.bilibili.cookie = cookie.to_string();
    storage::save_config(&config)?;
    log::info!("Cookie 保存成功");
    Ok(())
}

/// 获取当前 Cookie
pub fn get_cookie() -> Result<String> {
    log::debug!("读取 Cookie");
    let config = storage::load_config()?;
    let cookie = config.bilibili.cookie;
    log::debug!("Cookie 长度: {}", cookie.len());
    Ok(cookie)
}

/// 检查 Cookie 是否已配置
pub fn has_cookie() -> bool {
    let result = if let Ok(config) = storage::load_config() {
        !config.bilibili.cookie.is_empty()
    } else {
        false
    };
    log::debug!("检查 Cookie 状态: {}", result);
    result
}
