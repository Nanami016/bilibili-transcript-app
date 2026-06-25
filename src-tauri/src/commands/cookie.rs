// Cookie 相关命令
// 支持从浏览器读取和手动导入

use std::process::Command;
use tauri::command;

/// 导入 Cookie
#[command]
pub async fn import_cookie(cookie: String) -> Result<(), String> {
    log::info!("手动导入 Cookie: 长度={}", cookie.len());
    crate::bilibili::cookie::save_cookie(&cookie)
        .map_err(|e| e.to_string())
}

/// 获取 Cookie 状态
#[command]
pub async fn get_cookie_status() -> Result<bool, String> {
    Ok(crate::bilibili::cookie::has_cookie())
}

/// 从浏览器读取 Cookie 并保存
/// 使用 yt-dlp 的 --cookies-from-browser 功能导出 Cookie
#[command]
pub async fn import_cookie_from_browser(browser: String) -> Result<String, String> {
    log::info!("从浏览器读取 Cookie: browser={}", browser);

    // 使用 yt-dlp 导出 Cookie 到文件
    let cookie_path = std::env::temp_dir().join("bilibili-transcript-browser-cookies.txt");

    // 先删除旧文件
    let _ = std::fs::remove_file(&cookie_path);

    let output = Command::new("yt-dlp")
        .args([
            "--cookies-from-browser",
            &browser,
            "--cookies",
            cookie_path.to_str().unwrap_or(""),
            "--skip-download",
            "--print", "",
            "https://www.bilibili.com/video/BV1xx411c7mD",
        ])
        .output()
        .map_err(|e| format!("执行 yt-dlp 失败: {}", e))?;

    log::debug!("yt-dlp 退出状态: {:?}", output.status);

    if !cookie_path.exists() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::warn!("yt-dlp 未生成 Cookie 文件, stderr: {}", stderr);
        return Err(format!(
            "无法从 {} 读取 Cookie。请确保：\n1. 浏览器中已登录 B站\n2. yt-dlp 版本支持该浏览器\n\n错误信息: {}",
            browser, stderr
        ));
    }

    let content = std::fs::read_to_string(&cookie_path)
        .map_err(|e| format!("读取 Cookie 文件失败: {}", e))?;

    let (sessdata, dede_uid, bili_jct) = parse_cookie_content(&content)?;

    let cookie_str = format!(
        "SESSDATA={}; DedeUserID={}; bili_jct={}",
        sessdata, dede_uid, bili_jct
    );

    crate::bilibili::cookie::save_cookie(&cookie_str)
        .map_err(|e| e.to_string())?;

    let _ = std::fs::remove_file(&cookie_path);

    log::info!("从浏览器读取 Cookie 成功: uid={}", dede_uid);
    Ok(format!("成功从 {} 读取 Cookie (用户: {})", browser, dede_uid))
}

/// 解析 Cookie 内容
fn parse_cookie_content(content: &str) -> Result<(String, String, String), String> {
    let mut sessdata = String::new();
    let mut dede_uid = String::new();
    let mut bili_jct = String::new();

    for line in content.lines() {
        let line = line.trim();
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() >= 7 {
            let name = parts[5];
            let value = parts[6];
            match name {
                "SESSDATA" => sessdata = value.to_string(),
                "DedeUserID" => dede_uid = value.to_string(),
                "bili_jct" => bili_jct = value.to_string(),
                _ => {}
            }
        }
    }

    if sessdata.is_empty() || dede_uid.is_empty() {
        return Err("未找到 SESSDATA 或 DedeUserID".to_string());
    }

    Ok((sessdata, dede_uid, bili_jct))
}

/// 关闭登录窗口（已废弃，保留兼容）
#[command]
pub async fn close_bilibili_login() -> Result<(), String> {
    Ok(())
}

/// 清除 Cookie
#[command]
pub async fn clear_cookie() -> Result<String, String> {
    log::info!("清除 Cookie");
    crate::bilibili::cookie::save_cookie("")
        .map_err(|e| e.to_string())?;
    log::info!("Cookie 已清除");
    Ok("Cookie 已清除".to_string())
}

/// Cookie 条目
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CookieEntry {
    pub name: String,
    pub value: String,
}
