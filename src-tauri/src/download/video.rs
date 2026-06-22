// 视频下载
// 通过 yt-dlp 调用实现

use anyhow::{bail, Result};
use std::path::PathBuf;
use std::process::Command;

/// 将 Cookie 字符串写入 Netscape 格式的 cookie 文件
fn write_cookie_file(cookie: &str) -> Result<PathBuf> {
    let cookie_path = std::env::temp_dir().join("bilibili-transcript-cookies.txt");

    let mut lines = vec!["# Netscape HTTP Cookie File".to_string()];

    for part in cookie.split(';') {
        let part = part.trim();
        if let Some((name, value)) = part.split_once('=') {
            let name = name.trim();
            let value = value.trim();
            // 格式: domain	flag	path	secure	expires	name	value
            lines.push(format!(".bilibili.com\tTRUE\t/\tFALSE\t0\t{}\t{}", name, value));
        }
    }

    std::fs::write(&cookie_path, lines.join("\n"))?;
    log::debug!("Cookie 文件已写入: {:?}", cookie_path);
    Ok(cookie_path)
}

/// 下载视频
pub async fn download_video(url: &str, format_id: &str, output_dir: &PathBuf, cookie: &str) -> Result<PathBuf> {
    log::info!("开始下载视频: url={}, format={}", url, format_id);
    log::debug!("输出目录: {:?}", output_dir);

    std::fs::create_dir_all(output_dir)?;

    let output_template = output_dir.join("%(title)s.%(ext)s");
    let output_str = output_template.to_string_lossy();

    let mut args = vec![
        "-o".to_string(),
        output_str.to_string(),
        "--no-playlist".to_string(),
        "--user-agent".to_string(),
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36".to_string(),
        "--referer".to_string(),
        "https://www.bilibili.com".to_string(),
    ];

    // 格式选择：空或 "best" 都不传 -f，让 yt-dlp 自动选择最佳格式
    if !format_id.is_empty() && format_id != "best" {
        args.push("-f".to_string());
        args.push(format_id.to_string());
    }

    // 通过 Cookie 文件传递（B站需要 Cookie 才能下载）
    let cookie_file = if !cookie.is_empty() {
        let path = write_cookie_file(cookie)?;
        args.push("--cookies".to_string());
        args.push(path.to_string_lossy().to_string());
        log::debug!("使用 Cookie 文件: {:?}", path);
        Some(path)
    } else {
        log::warn!("未配置 Cookie，下载可能会失败");
        None
    };

    args.push(url.to_string());

    log::debug!("执行 yt-dlp {:?}", args);

    let output = Command::new("yt-dlp")
        .args(&args)
        .output()?;

    // 清理 Cookie 文件
    if let Some(path) = &cookie_file {
        let _ = std::fs::remove_file(path);
    }

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let err_msg = if !stderr.is_empty() { stderr.to_string() } else { stdout.to_string() };
        log::error!("yt-dlp 下载失败: {}", err_msg);
        bail!("yt-dlp 下载失败: {}", err_msg);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    log::debug!("yt-dlp 输出: {}", stdout);
    let filepath = parse_download_path(&stdout, output_dir)
        .unwrap_or_else(|| output_dir.join("download.mp4"));

    // 清理文件名中的格式 ID 后缀（如 .f30033）
    let clean_path = clean_format_suffix(&filepath);
    if clean_path != filepath {
        if let Err(e) = std::fs::rename(&filepath, &clean_path) {
            log::warn!("重命名文件失败（保留原文件名）: {}", e);
        } else {
            log::debug!("文件名已清理: {:?} -> {:?}", filepath, clean_path);
        }
    }

    let result = if clean_path.exists() { clean_path } else { filepath };
    log::info!("视频下载完成: {:?}", result);
    Ok(result)
}

/// 获取视频可用格式列表
pub async fn list_formats(url: &str, cookie: &str) -> Result<Vec<serde_json::Value>> {
    log::info!("获取视频格式列表: {}", url);

    let mut args = vec![
        "-F".to_string(),
        "--dump-json".to_string(),
        "--user-agent".to_string(),
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36".to_string(),
        "--referer".to_string(),
        "https://www.bilibili.com".to_string(),
    ];

    let cookie_file = if !cookie.is_empty() {
        let path = write_cookie_file(cookie)?;
        args.push("--cookies".to_string());
        args.push(path.to_string_lossy().to_string());
        Some(path)
    } else {
        None
    };

    args.push(url.to_string());

    let output = Command::new("yt-dlp")
        .args(&args)
        .output()?;

    if let Some(path) = &cookie_file {
        let _ = std::fs::remove_file(path);
    }

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::error!("yt-dlp 获取格式失败: {}", stderr);
        bail!("yt-dlp 获取格式失败: {}", stderr);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut formats = Vec::new();

    for line in stdout.lines() {
        if let Ok(data) = serde_json::from_str::<serde_json::Value>(line) {
            formats.push(data);
        }
    }

    log::info!("格式列表获取成功: {} 个格式", formats.len());
    Ok(formats)
}

/// 清理文件名中的格式 ID 后缀（如 .f30033.mp4 -> .mp4）
fn clean_format_suffix(path: &std::path::Path) -> PathBuf {
    let filename = match path.file_stem().and_then(|n| n.to_str()) {
        Some(f) => f,
        None => return path.to_path_buf(),
    };
    let ext = match path.extension().and_then(|e| e.to_str()) {
        Some(e) => e,
        None => return path.to_path_buf(),
    };

    // 匹配 .f + 数字 的格式 ID 后缀（如 f30033, f30280）
    let re = regex::Regex::new(r"\.f\d+$").unwrap();
    if re.is_match(filename) {
        let cleaned = re.replace(filename, "");
        if let Some(parent) = path.parent() {
            return parent.join(format!("{}.{}", cleaned, ext));
        }
    }

    path.to_path_buf()
}

/// 从 yt-dlp 输出中解析下载的文件路径
fn parse_download_path(output: &str, _output_dir: &PathBuf) -> Option<PathBuf> {
    for line in output.lines() {
        if line.contains("Destination:") {
            if let Some(path) = line.split("Destination:").nth(1) {
                return Some(PathBuf::from(path.trim()));
            }
        }
        if line.contains("has already been downloaded") {
            if let Some(path) = line.split("has already been downloaded").next() {
                let path = path.trim();
                if let Some(path) = path.strip_prefix("[download]") {
                    return Some(PathBuf::from(path.trim()));
                }
            }
        }
    }
    None
}
