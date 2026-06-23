// 转录相关命令

use std::path::PathBuf;
use tauri::command;

use crate::bilibili;
use crate::storage::database::{Database, TranscriptRecord};
use crate::transcribe::pipeline::TranscriptResult;

/// 执行转录
/// 完整流程: 解析 URL → 获取视频信息 → 执行转录流水线 → 存储到数据库 → 渲染 TXT
#[command]
pub async fn transcribe(url: String) -> Result<TranscriptResult, String> {
    let config = crate::config::storage::load_config().map_err(|e| e.to_string())?;

    // 提取 BVid
    let bvid = bilibili::api::extract_bvid(&url)
        .ok_or_else(|| "无法从 URL 中提取 BVid，请检查链接格式".to_string())?;

    // 获取视频信息
    let video_info = bilibili::api::get_video_info(&bvid, &config.bilibili.cookie)
        .await
        .map_err(|e| format!("获取视频信息失败: {}", e))?;

    // 执行转录流水线
    let result = crate::transcribe::pipeline::transcribe_video(
        &url,
        &bvid,
        video_info.cid,
        &config.bilibili.cookie,
        &config,
        None,
        None,
        None,
    )
    .await
    .map_err(|e| format!("转录失败: {}", e))?;

    // 存储到数据库
    let db = Database::open().map_err(|e| format!("数据库打开失败: {}", e))?;

    let duration_str = format_duration(video_info.duration);

    let record = TranscriptRecord {
        id: 0,
        bvid: bvid.clone(),
        url: url.clone(),
        title: video_info.title.clone(),
        author: video_info.author.clone(),
        duration: duration_str,
        upload_date: video_info.upload_date.clone(),
        transcript_source: result.source.clone(),
        transcript_text: result.text.clone(),
        summary: None,
        status: "transcribed".to_string(),
        created_at: String::new(),
        updated_at: String::new(),
    };

    db.upsert(&record).map_err(|e| format!("保存转录记录失败: {}", e))?;

    // 渲染 TXT 文件
    let output_dir = PathBuf::from(shellexpand::tilde(&config.bilibili.transcript_dir).to_string());
    if let Err(e) = crate::storage::file::render_txt(&record, &output_dir) {
        log::warn!("TXT 文件渲染失败: {}", e);
    }

    Ok(result)
}

/// 测试 Whisper 连接
#[command]
pub async fn test_whisper_connection() -> Result<bool, String> {
    let config = crate::config::storage::load_config().map_err(|e| e.to_string())?;

    let whisper_prompt = if config.whisper.prompt.is_empty() { None } else { Some(config.whisper.prompt.clone()) };
    let client = crate::transcribe::whisper::OpenAIWhisperClient::new(
        config.whisper.api_url.clone(),
        config.whisper.api_key.clone(),
        config.whisper.model.clone(),
        whisper_prompt,
        None,
    );

    crate::transcribe::whisper::WhisperClient::test_connection(&client)
        .await
        .map_err(|e| e.to_string())
}

/// 测试 AI 摘要 API 连接
#[command]
pub async fn test_ai_summary_connection() -> Result<bool, String> {
    let config = crate::config::storage::load_config().map_err(|e| e.to_string())?;

    let api_key = config.ai_summary.api_key
        .filter(|k| !k.is_empty())
        .ok_or("未配置 AI 摘要 API Key")?;

    crate::summary::openai::test_connection(
        &config.ai_summary.api_url,
        &api_key,
        &config.ai_summary.model,
    )
    .await
    .map_err(|e| e.to_string())
}

/// 格式化秒数为 HH:MM:SS
fn format_duration(seconds: i64) -> String {
    let h = seconds / 3600;
    let m = (seconds % 3600) / 60;
    let s = seconds % 60;
    if h > 0 {
        format!("{:02}:{:02}:{:02}", h, m, s)
    } else {
        format!("{:02}:{:02}", m, s)
    }
}
