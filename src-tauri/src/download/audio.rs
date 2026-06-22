// 音频提取
// 通过 yt-dlp + ffmpeg 实现

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
            lines.push(format!(".bilibili.com\tTRUE\t/\tFALSE\t0\t{}\t{}", name, value));
        }
    }

    std::fs::write(&cookie_path, lines.join("\n"))?;
    log::debug!("Cookie 文件已写入: {:?}", cookie_path);
    Ok(cookie_path)
}

/// 清理文件名中的格式 ID 后缀（如 .f30033.mp3 -> .mp3）
fn clean_format_suffix(path: &std::path::Path) -> std::path::PathBuf {
    let filename = match path.file_stem().and_then(|n| n.to_str()) {
        Some(f) => f,
        None => return path.to_path_buf(),
    };
    let ext = match path.extension().and_then(|e| e.to_str()) {
        Some(e) => e,
        None => return path.to_path_buf(),
    };

    let re = regex::Regex::new(r"\.f\d+$").unwrap();
    if re.is_match(filename) {
        let cleaned = re.replace(filename, "");
        if let Some(parent) = path.parent() {
            return parent.join(format!("{}.{}", cleaned, ext));
        }
    }

    path.to_path_buf()
}

/// 提取音频（下载 + 转 WAV）
pub async fn extract_audio(url: &str, output_dir: &PathBuf, cookie: &str) -> Result<PathBuf> {
    log::info!("开始提取音频: {}", url);
    std::fs::create_dir_all(output_dir)?;

    // Step 1: 使用 yt-dlp 下载音频
    log::debug!("Step 1: yt-dlp 下载音频");
    let mp3_template = output_dir.join("%(title)s.%(ext)s");
    let mp3_str = mp3_template.to_string_lossy();

    let mut args = vec![
        "-x".to_string(),
        "--audio-format".to_string(),
        "mp3".to_string(),
        "-o".to_string(),
        mp3_str.to_string(),
        "--no-playlist".to_string(),
        "--user-agent".to_string(),
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36".to_string(),
        "--referer".to_string(),
        "https://www.bilibili.com".to_string(),
    ];

    let cookie_file = if !cookie.is_empty() {
        let path = write_cookie_file(cookie)?;
        args.push("--cookies".to_string());
        args.push(path.to_string_lossy().to_string());
        log::debug!("使用 Cookie 文件: {:?}", path);
        Some(path)
    } else {
        None
    };

    args.push(url.to_string());

    log::debug!("执行 yt-dlp {:?}", args);

    let output = Command::new("yt-dlp")
        .args(&args)
        .output()?;

    if let Some(path) = &cookie_file {
        let _ = std::fs::remove_file(path);
    }

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let err_msg = if !stderr.is_empty() { stderr.to_string() } else { stdout.to_string() };
        log::error!("yt-dlp 音频下载失败: {}", err_msg);
        bail!("yt-dlp 音频下载失败: {}", err_msg);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    log::debug!("yt-dlp 输出: {}", stdout);

    let mp3_path = find_downloaded_file(&stdout, output_dir, "mp3")
        .ok_or_else(|| anyhow::anyhow!("无法找到下载的音频文件"))?;

    // 清理文件名中的格式 ID 后缀
    let mp3_path = clean_format_suffix(&mp3_path);
    log::debug!("音频文件: {:?}", mp3_path);

    // Step 2: 使用 ffmpeg 转为 16kHz 单声道 WAV
    log::debug!("Step 2: ffmpeg 转换为 16kHz WAV");
    let wav_path = mp3_path.with_extension("wav");
    let ffmpeg_output = Command::new("ffmpeg")
        .args([
            "-y",
            "-i",
            &mp3_path.to_string_lossy(),
            "-ar",
            "16000",
            "-ac",
            "1",
            "-c:a",
            "pcm_s16le",
            &wav_path.to_string_lossy(),
        ])
        .output()?;

    if !ffmpeg_output.status.success() {
        let stderr = String::from_utf8_lossy(&ffmpeg_output.stderr);
        log::error!("ffmpeg 音频转换失败: {}", stderr);
        bail!("ffmpeg 音频转换失败: {}", stderr);
    }

    let _ = std::fs::remove_file(&mp3_path);

    log::info!("音频提取完成: {:?}", wav_path);
    Ok(wav_path)
}

/// 仅下载音频（不转换格式）
pub async fn download_audio(url: &str, output_dir: &PathBuf, cookie: &str) -> Result<PathBuf> {
    log::info!("下载音频: {}", url);
    std::fs::create_dir_all(output_dir)?;

    let output_template = output_dir.join("%(title)s.%(ext)s");
    let output_str = output_template.to_string_lossy();

    let mut args = vec![
        "-x".to_string(),
        "--audio-format".to_string(),
        "mp3".to_string(),
        "-o".to_string(),
        output_str.to_string(),
        "--no-playlist".to_string(),
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
        let stdout = String::from_utf8_lossy(&output.stdout);
        let err_msg = if !stderr.is_empty() { stderr.to_string() } else { stdout.to_string() };
        log::error!("yt-dlp 音频下载失败: {}", err_msg);
        bail!("yt-dlp 音频下载失败: {}", err_msg);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let filepath = find_downloaded_file(&stdout, output_dir, "mp3")
        .ok_or_else(|| anyhow::anyhow!("无法找到下载的音频文件"))?;

    // 清理文件名中的格式 ID 后缀
    let clean_path = clean_format_suffix(&filepath);
    if clean_path != filepath {
        if let Err(e) = std::fs::rename(&filepath, &clean_path) {
            log::warn!("重命名文件失败（保留原文件名）: {}", e);
        } else {
            log::debug!("文件名已清理: {:?} -> {:?}", filepath, clean_path);
        }
    }

    let result = if clean_path.exists() { clean_path } else { filepath };
    log::info!("音频下载完成: {:?}", result);
    Ok(result)
}

/// 从 yt-dlp 输出中查找下载的文件
fn find_downloaded_file(output: &str, output_dir: &PathBuf, ext: &str) -> Option<PathBuf> {
    // 优先从 yt-dlp 输出的 Destination: 行获取路径
    // 注意：yt-dlp -x 转换格式后，原始文件会被删除，Destination 路径可能已不存在
    for line in output.lines() {
        if line.contains("Destination:") {
            if let Some(path) = line.split("Destination:").nth(1) {
                let p = PathBuf::from(path.trim());
                if p.exists() {
                    return Some(p);
                }
                log::debug!("Destination 路径已不存在（可能已被格式转换删除）: {:?}", p);
            }
        }
    }

    // Fallback: 扫描目录中最近修改的音频文件
    // 优先匹配目标格式，也接受其他常见音频格式（yt-dlp 可能未能转换）
    let audio_exts = [ext, "m4a", "wav", "ogg", "flac", "aac", "opus"];
    let mut latest: Option<(std::time::SystemTime, PathBuf)> = None;
    if let Ok(entries) = std::fs::read_dir(output_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            let is_audio = path
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| audio_exts.contains(&e))
                .unwrap_or(false);
            if is_audio {
                if let Ok(metadata) = entry.metadata() {
                    if let Ok(modified) = metadata.modified() {
                        if latest.is_none() || modified > latest.as_ref().unwrap().0 {
                            latest = Some((modified, path));
                        }
                    }
                }
            }
        }
    }

    latest.map(|(_, path)| path)
}
