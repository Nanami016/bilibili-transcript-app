// 视频相关命令

use std::path::PathBuf;
use tauri::command;

use crate::bilibili;
use crate::bilibili::types::{VideoInfo, VideoFormat};

/// 解析视频信息
#[command]
pub async fn parse_video(url: String) -> Result<VideoInfo, String> {
    log::info!("解析视频: url={}", url);
    let bvid = bilibili::api::extract_bvid(&url)
        .ok_or_else(|| "无法从 URL 中提取 BVid，请检查链接格式".to_string())?;

    let config = crate::config::storage::load_config().map_err(|e| e.to_string())?;

    bilibili::api::get_video_info(&bvid, &config.bilibili.cookie)
        .await
        .map_err(|e| e.to_string())
}

/// 获取视频可用格式列表
#[command]
pub async fn get_video_formats(url: String) -> Result<Vec<VideoFormat>, String> {
    log::info!("获取视频格式: url={}", url);
    let config = crate::config::storage::load_config().map_err(|e| e.to_string())?;
    bilibili::api::get_video_formats(&url, &config.bilibili.cookie)
        .await
        .map_err(|e| e.to_string())
}

/// 下载视频
#[command]
pub async fn download_video(url: String, format_id: String) -> Result<String, String> {
    log::info!("下载视频: url={}, format={}", url, format_id);
    let config = crate::config::storage::load_config().map_err(|e| e.to_string())?;
    let output_dir = PathBuf::from(shellexpand::tilde(&config.bilibili.video_dir).to_string());

    crate::download::video::download_video(&url, &format_id, &output_dir, &config.bilibili.cookie, None)
        .await
        .map(|path| path.to_string_lossy().to_string())
        .map_err(|e| e.to_string())
}

/// 获取封面图片（代理，返回 base64 data URL）
#[command]
pub async fn fetch_cover(url: String) -> Result<String, String> {
    log::debug!("获取封面图片: {}", url);
    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36")
        .header("Referer", "https://www.bilibili.com")
        .send()
        .await
        .map_err(|e| format!("请求封面失败: {}", e))?;

    let bytes = resp.bytes().await.map_err(|e| format!("读取封面失败: {}", e))?;
    let b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &bytes);
    let data_url = format!("data:image/jpeg;base64,{}", b64);
    log::debug!("封面获取成功: {} bytes", bytes.len());
    Ok(data_url)
}

/// 下载音频
#[command]
pub async fn download_audio(url: String) -> Result<String, String> {
    log::info!("下载音频: url={}", url);
    let config = crate::config::storage::load_config().map_err(|e| e.to_string())?;
    let output_dir = PathBuf::from(shellexpand::tilde(&config.bilibili.audio_dir).to_string());

    crate::download::audio::download_audio(&url, &output_dir, &config.bilibili.cookie, None)
        .await
        .map(|path| path.to_string_lossy().to_string())
        .map_err(|e| e.to_string())
}
