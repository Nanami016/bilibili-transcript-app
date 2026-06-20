// 音频提取
// 通过 yt-dlp + ffmpeg 实现

use anyhow::Result;
use std::path::PathBuf;

/// 提取音频
/// url: B站视频链接
/// output_dir: 输出目录
/// 返回: 音频文件路径
pub async fn extract_audio(url: &str, output_dir: &PathBuf) -> Result<PathBuf> {
    // TODO: 调用 yt-dlp -x --audio-format mp3 下载音频
    // 然后用 ffmpeg 转为 16kHz 单声道 WAV（Whisper 推荐格式）
    todo!("实现音频提取")
}
