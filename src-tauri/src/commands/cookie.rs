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
/// 使用 yt-dlp 的 --cookies-from-browser 功能
#[command]
pub async fn import_cookie_from_browser(browser: String) -> Result<String, String> {
    log::info!("从浏览器读取 Cookie: browser={}", browser);

    // 使用 yt-dlp 导出 Cookie 到文件
    let cookie_path = std::env::temp_dir().join("bilibili-transcript-browser-cookies.txt");

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

    // 即使命令失败（比如视频不存在），cookie 文件可能已经导出
    if !cookie_path.exists() {
        // 尝试另一种方式：直接用 yt-dlp dump cookies
        let _output2 = Command::new("yt-dlp")
            .args([
                "--cookies-from-browser",
                &browser,
                "--dump-json",
                "--no-download",
                "https://www.bilibili.com/video/BV1xx411c7mD",
            ])
            .output();

        if !cookie_path.exists() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!(
                "无法从 {} 读取 Cookie。请确保：\n1. 浏览器中已登录 B站\n2. 浏览器已关闭\n3. yt-dlp 版本支持该浏览器\n\n错误信息: {}",
                browser, stderr
            ));
        }
    }

    // 读取导出的 Cookie 文件，提取关键字段
    let content = std::fs::read_to_string(&cookie_path)
        .map_err(|e| format!("读取 Cookie 文件失败: {}", e))?;

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
        return Err(format!(
            "从 {} 读取的 Cookie 中未找到 SESSDATA 或 DedeUserID。请确保浏览器中已登录 B站。",
            browser
        ));
    }

    // 组装 Cookie 字符串
    let cookie_str = format!(
        "SESSDATA={}; DedeUserID={}; bili_jct={}",
        sessdata, dede_uid, bili_jct
    );

    crate::bilibili::cookie::save_cookie(&cookie_str)
        .map_err(|e| e.to_string())?;

    // 清理临时文件
    let _ = std::fs::remove_file(&cookie_path);

    log::info!("从浏览器读取 Cookie 成功: uid={}", dede_uid);
    Ok(format!("成功从 {} 读取 Cookie (用户: {})", browser, dede_uid))
}

/// 关闭登录窗口（已废弃，保留兼容）
#[command]
pub async fn close_bilibili_login() -> Result<(), String> {
    Ok(())
}

/// Cookie 条目
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CookieEntry {
    pub name: String,
    pub value: String,
}
