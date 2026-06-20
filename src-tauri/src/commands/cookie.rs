// Cookie 相关命令

use tauri::command;

/// 导入 Cookie
#[command]
pub async fn import_cookie(cookie: String) -> Result<(), String> {
    crate::bilibili::cookie::save_cookie(&cookie)
        .map_err(|e| e.to_string())
}

/// 获取 Cookie 状态
#[command]
pub async fn get_cookie_status() -> Result<bool, String> {
    Ok(crate::bilibili::cookie::has_cookie())
}
