// 视频相关命令

use tauri::command;

use crate::bilibili::types::{VideoInfo, VideoFormat};

/// 解析视频信息
#[command]
pub async fn parse_video(url: String) -> Result<VideoInfo, String> {
    // TODO: 从 URL 提取 bvid，调用 API 获取视频信息
    todo!("实现视频解析命令")
}

/// 获取视频可用格式列表
#[command]
pub async fn get_video_formats(url: String) -> Result<Vec<VideoFormat>, String> {
    // TODO: 获取视频格式列表
    todo!("实现格式列表命令")
}

/// 下载视频
#[command]
pub async fn download_video(url: String, format_id: String) -> Result<String, String> {
    // TODO: 调用 yt-dlp 下载视频
    todo!("实现视频下载命令")
}

/// 下载音频
#[command]
pub async fn download_audio(url: String) -> Result<String, String> {
    // TODO: 提取音频
    todo!("实现音频下载命令")
}
