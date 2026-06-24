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
/// 优先使用直接读取 Chrome SQLite 数据库的方式，失败时回退到 yt-dlp
#[command]
pub async fn import_cookie_from_browser(browser: String) -> Result<String, String> {
    log::info!("从浏览器读取 Cookie: browser={}", browser);

    // 优先尝试直接读取 Chrome Cookie 数据库
    if browser == "chrome" || browser == "chromium" {
        match read_chrome_cookies_directly() {
            Ok(result) => {
                log::info!("直接读取 Chrome Cookie 成功: uid={}", result.1);
                return Ok(format!("成功从 Chrome 读取 Cookie (用户: {})", result.1));
            }
            Err(e) => {
                log::warn!("直接读取 Chrome Cookie 失败，尝试 yt-dlp 方式: {}", e);
            }
        }
    }

    // 回退到 yt-dlp 方式
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

    if !cookie_path.exists() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "无法从 {} 读取 Cookie。请确保：\n1. 浏览器中已登录 B站\n2. 浏览器已关闭\n3. yt-dlp 版本支持该浏览器\n\n错误信息: {}",
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

/// 直接读取 Chrome Cookie 数据库
fn read_chrome_cookies_directly() -> Result<(String, String, String), String> {
    let home = dirs::home_dir().ok_or("无法获取用户主目录")?;
    let cookie_db_path = home
        .join("Library/Application Support/Google/Chrome/Default/Cookies");

    if !cookie_db_path.exists() {
        return Err("Chrome Cookie 数据库不存在".to_string());
    }

    // 复制数据库到临时目录（避免锁定问题）
    let temp_db = std::env::temp_dir().join("bilibili-chrome-cookies.db");
    std::fs::copy(&cookie_db_path, &temp_db)
        .map_err(|e| format!("复制 Cookie 数据库失败: {}", e))?;

    let conn = rusqlite::Connection::open_with_flags(
        &temp_db,
        rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY | rusqlite::OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )
    .map_err(|e| format!("打开 Cookie 数据库失败: {}", e))?;

    // 查询 B站相关 Cookie
    let mut stmt = conn
        .prepare("SELECT name, encrypted_value, value FROM cookies WHERE host_key LIKE '%bilibili.com%'")
        .map_err(|e| format!("查询 Cookie 失败: {}", e))?;

    let mut sessdata = String::new();
    let mut dede_uid = String::new();
    let mut bili_jct = String::new();

    let rows = stmt
        .query_map([], |row| {
            let name: String = row.get(0)?;
            let encrypted_value: Vec<u8> = row.get(1)?;
            let value: String = row.get(2)?;
            Ok((name, encrypted_value, value))
        })
        .map_err(|e| format!("读取 Cookie 行失败: {}", e))?;

    for row in rows {
        let (name, encrypted_value, value) = row.map_err(|e| format!("解析 Cookie 失败: {}", e))?;

        log::debug!("找到 Cookie: name={}, value_len={}, encrypted_len={}", name, value.len(), encrypted_value.len());

        // 如果 value 为空，需要解密 encrypted_value
        let cookie_value = if value.is_empty() && !encrypted_value.is_empty() {
            match decrypt_chrome_cookie(&encrypted_value) {
                Ok(v) => {
                    log::debug!("解密成功: name={}, value_len={}", name, v.len());
                    v
                }
                Err(e) => {
                    log::warn!("解密失败: name={}, error={}", name, e);
                    String::new()
                }
            }
        } else {
            value
        };

        match name.as_str() {
            "SESSDATA" => sessdata = cookie_value,
            "DedeUserID" => dede_uid = cookie_value,
            "bili_jct" => bili_jct = cookie_value,
            _ => {}
        }
    }

    // 清理临时文件
    let _ = std::fs::remove_file(&temp_db);

    if sessdata.is_empty() || dede_uid.is_empty() {
        return Err("未找到 SESSDATA 或 DedeUserID，请确保已在 Chrome 中登录 B站".to_string());
    }

    Ok((sessdata, dede_uid, bili_jct))
}

/// 解密 Chrome Cookie（macOS Keychain）
/// 参考 yt-dlp: macOS Chrome 使用 AES-128-CBC，IV 为 16 个空格
fn decrypt_chrome_cookie(encrypted_value: &[u8]) -> Result<String, String> {
    // Chrome v10+ 使用 AES-128-CBC 加密
    // 前 3 字节是版本前缀 "v10" 或 "v11"，剩余部分是密文
    if encrypted_value.len() < 3 {
        return Err("加密数据太短".to_string());
    }

    let version = &encrypted_value[..3];
    if version != b"v10" && version != b"v11" {
        // 非 v10/v11 视为明文存储
        return String::from_utf8(encrypted_value.to_vec())
            .map_err(|e| format!("非加密 Cookie 解码失败: {}", e));
    }

    // v10/v11: 前 3 字节版本 + 剩余字节为密文
    let ciphertext = &encrypted_value[3..];

    // macOS Chrome 的 IV 是固定的 16 个空格 (0x20)
    let iv: [u8; 16] = [b' '; 16];

    // 从 Keychain 获取加密密钥（16 字节的派生密钥）
    let key_bytes = get_chrome_encryption_key()?;

    log::debug!("解密 Cookie: version={:?}, ciphertext_len={}, key_len={}",
        String::from_utf8_lossy(version), ciphertext.len(), key_bytes.len());

    // 使用 PKCS7 填充解密
    use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, KeyIvInit};
    use aes::Aes128;

    type Aes128CbcDec = cbc::Decryptor<Aes128>;

    let mut buf = ciphertext.to_vec();

    let decryptor = Aes128CbcDec::new(
        aes::cipher::generic_array::GenericArray::from_slice(&key_bytes),
        aes::cipher::generic_array::GenericArray::from_slice(&iv),
    );

    let decrypted = decryptor
        .decrypt_padded_mut::<Pkcs7>(&mut buf)
        .map_err(|e| format!("AES 解密失败 (可能密钥不正确): {:?}", e))?;

    log::debug!("解密完成: decrypted_len={}", decrypted.len());

    String::from_utf8(decrypted.to_vec())
        .map_err(|e| format!("解密结果不是有效的 UTF-8: {}", e))
}

/// 从 macOS Keychain 获取 Chrome 加密密钥（返回 16 字节的派生密钥）
fn get_chrome_encryption_key() -> Result<Vec<u8>, String> {
    use std::process::Command;

    let output = Command::new("security")
        .args(["find-generic-password", "-s", "Chrome Safe Storage", "-w"])
        .output()
        .map_err(|e| format!("执行 security 命令失败: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("从 Keychain 获取密钥失败: {}", stderr));
    }

    let key = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if key.is_empty() {
        return Err("Keychain 返回的密钥为空".to_string());
    }

    log::debug!("从 Keychain 获取到 Chrome 密钥 (长度={})", key.len());

    // Chrome 使用 PBKDF2 派生密钥
    // 密码: Keychain 中的密钥 (UTF-8 字节)
    // 盐: "saltysalt"
    // 迭代次数: 1003
    // 输出长度: 16 字节
    use pbkdf2::pbkdf2_hmac;
    use sha1::Sha1;

    let mut derived_key = [0u8; 16];
    pbkdf2_hmac::<Sha1>(key.as_bytes(), b"saltysalt", 1003, &mut derived_key);

    log::debug!("PBKDF2 派生密钥完成 (hex={})", hex::encode(derived_key));

    Ok(derived_key.to_vec())
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

/// Cookie 条目
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CookieEntry {
    pub name: String,
    pub value: String,
}
