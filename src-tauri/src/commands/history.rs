// 历史记录相关命令

use tauri::command;

use crate::storage::database::{Database, TranscriptRecord};

/// 获取所有转录记录
#[command]
pub async fn get_transcript_history() -> Result<Vec<TranscriptRecord>, String> {
    let db = Database::open().map_err(|e| e.to_string())?;
    db.get_all().map_err(|e| e.to_string())
}

/// 根据 bvid 获取转录记录
#[command]
pub async fn get_transcript(bvid: String) -> Result<Option<TranscriptRecord>, String> {
    let db = Database::open().map_err(|e| e.to_string())?;
    db.get_by_bvid(&bvid).map_err(|e| e.to_string())
}

/// 删除转录记录
#[command]
pub async fn delete_transcript(bvid: String) -> Result<(), String> {
    let db = Database::open().map_err(|e| e.to_string())?;
    db.delete(&bvid).map_err(|e| e.to_string())
}
