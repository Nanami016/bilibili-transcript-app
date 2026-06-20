// 转录相关命令

use tauri::command;

use crate::transcribe::pipeline::TranscriptResult;

/// 执行转录
#[command]
pub async fn transcribe(url: String) -> Result<TranscriptResult, String> {
    // TODO: 执行转录流水线
    todo!("实现转录命令")
}

/// 测试 Whisper 连接
#[command]
pub async fn test_whisper_connection() -> Result<bool, String> {
    // TODO: 测试 Whisper API 连接
    todo!("实现连接测试命令")
}
