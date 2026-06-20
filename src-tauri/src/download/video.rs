// 视频下载
// 通过 yt-dlp 调用实现

use anyhow::Result;
use std::path::PathBuf;

/// 下载视频
/// url: B站视频链接
/// format_id: yt-dlp 格式 ID
/// output_dir: 输出目录
pub async fn download_video(url: &str, format_id: &str, output_dir: &PathBuf) -> Result<PathBuf> {
    // TODO: 调用 yt-dlp 下载视频
    // yt-dlp -f {format_id} -o {output_dir}/%(title)s.%(ext)s {url}
    todo!("实现视频下载")
}

/// 获取视频可用格式列表
pub async fn list_formats(url: &str, cookie: &str) -> Result<Vec<serde_json::Value>> {
    // TODO: 调用 yt-dlp -F --dump-json 获取格式列表
    todo!("实现格式列表获取")
}
