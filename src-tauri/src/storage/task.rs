// 任务记录存储
// 管理视频下载/音频下载/转录/AI摘要的任务历史和进度

use anyhow::Result;
use rusqlite::Connection;

/// 任务记录
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TaskRecord {
    pub id: i64,
    pub task_type: String,      // "video_download" | "audio_download" | "transcribe" | "ai_summary"
    pub url: String,
    pub title: String,
    pub author: String,
    pub status: String,         // "pending" | "running" | "completed" | "failed" | "cancelled"
    pub progress: f64,          // 0.0 ~ 100.0
    pub speed: String,          // "2.3MB/s"
    pub eta: String,            // "1分30秒"
    pub output_path: Option<String>,
    pub error: Option<String>,
    pub file_size: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub completed_at: Option<String>,
}

/// 任务进度更新事件（发送给前端）
#[derive(Debug, Clone, serde::Serialize)]
pub struct TaskProgressEvent {
    pub task_id: i64,
    pub task_type: String,
    pub status: String,
    pub progress: f64,
    pub speed: String,
    pub eta: String,
}

/// 任务完成事件（发送给前端）
#[derive(Debug, Clone, serde::Serialize)]
pub struct TaskCompletedEvent {
    pub task_id: i64,
    pub task_type: String,
    pub status: String,
    pub title: String,
    pub output_path: Option<String>,
    pub error: Option<String>,
    pub file_size: Option<String>,
}

/// 初始化 tasks 表
pub fn init_tasks_table(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            task_type TEXT NOT NULL,
            url TEXT NOT NULL,
            title TEXT DEFAULT '',
            author TEXT DEFAULT '',
            status TEXT DEFAULT 'pending',
            progress REAL DEFAULT 0.0,
            speed TEXT DEFAULT '',
            eta TEXT DEFAULT '',
            output_path TEXT,
            error TEXT,
            file_size TEXT,
            created_at TEXT DEFAULT (datetime('now', 'localtime')),
            updated_at TEXT DEFAULT (datetime('now', 'localtime')),
            completed_at TEXT
        );"
    )?;
    Ok(())
}

/// 插入新任务，返回 task_id
pub fn insert_task(
    conn: &Connection,
    task_type: &str,
    url: &str,
    title: &str,
    author: &str,
) -> Result<i64> {
    conn.execute(
        "INSERT INTO tasks (task_type, url, title, author, status, progress) VALUES (?1, ?2, ?3, ?4, 'pending', 0.0)",
        rusqlite::params![task_type, url, title, author],
    )?;
    Ok(conn.last_insert_rowid())
}

/// 更新任务进度
pub fn update_task_progress(
    conn: &Connection,
    task_id: i64,
    progress: f64,
    speed: &str,
    eta: &str,
) -> Result<()> {
    conn.execute(
        "UPDATE tasks SET progress = ?1, speed = ?2, eta = ?3, status = 'running', updated_at = datetime('now', 'localtime') WHERE id = ?4",
        rusqlite::params![progress, speed, eta, task_id],
    )?;
    Ok(())
}

/// 更新任务状态（完成/失败/取消）
pub fn update_task_status(
    conn: &Connection,
    task_id: i64,
    status: &str,
    output_path: Option<&str>,
    error: Option<&str>,
    file_size: Option<&str>,
) -> Result<()> {
    conn.execute(
        "UPDATE tasks SET status = ?1, output_path = ?2, error = ?3, file_size = ?4, updated_at = datetime('now', 'localtime'), completed_at = datetime('now', 'localtime') WHERE id = ?5",
        rusqlite::params![status, output_path, error, file_size, task_id],
    )?;
    Ok(())
}

/// 更新任务标题（下载后才知道标题时使用）
pub fn update_task_title(conn: &Connection, task_id: i64, title: &str) -> Result<()> {
    conn.execute(
        "UPDATE tasks SET title = ?1, updated_at = datetime('now', 'localtime') WHERE id = ?2",
        rusqlite::params![title, task_id],
    )?;
    Ok(())
}

/// 按类型获取任务历史
pub fn get_tasks_by_type(conn: &Connection, task_type: &str) -> Result<Vec<TaskRecord>> {
    let mut stmt = conn.prepare(
        "SELECT id, task_type, url, title, author, status, progress, speed, eta, output_path, error, file_size, created_at, updated_at, completed_at
         FROM tasks WHERE task_type = ?1 ORDER BY created_at DESC"
    )?;

    let rows = stmt.query_map(rusqlite::params![task_type], |row| {
        Ok(TaskRecord {
            id: row.get(0)?,
            task_type: row.get(1)?,
            url: row.get(2)?,
            title: row.get(3)?,
            author: row.get(4)?,
            status: row.get(5)?,
            progress: row.get(6)?,
            speed: row.get(7)?,
            eta: row.get(8)?,
            output_path: row.get(9)?,
            error: row.get(10)?,
            file_size: row.get(11)?,
            created_at: row.get(12)?,
            updated_at: row.get(13)?,
            completed_at: row.get(14)?,
        })
    })?;

    let mut records = Vec::new();
    for row in rows {
        records.push(row?);
    }
    Ok(records)
}

/// 获取所有进行中的任务
pub fn get_active_tasks(conn: &Connection) -> Result<Vec<TaskRecord>> {
    let mut stmt = conn.prepare(
        "SELECT id, task_type, url, title, author, status, progress, speed, eta, output_path, error, file_size, created_at, updated_at, completed_at
         FROM tasks WHERE status IN ('pending', 'running') ORDER BY created_at DESC"
    )?;

    let rows = stmt.query_map([], |row| {
        Ok(TaskRecord {
            id: row.get(0)?,
            task_type: row.get(1)?,
            url: row.get(2)?,
            title: row.get(3)?,
            author: row.get(4)?,
            status: row.get(5)?,
            progress: row.get(6)?,
            speed: row.get(7)?,
            eta: row.get(8)?,
            output_path: row.get(9)?,
            error: row.get(10)?,
            file_size: row.get(11)?,
            created_at: row.get(12)?,
            updated_at: row.get(13)?,
            completed_at: row.get(14)?,
        })
    })?;

    let mut records = Vec::new();
    for row in rows {
        records.push(row?);
    }
    Ok(records)
}

/// 删除任务记录
pub fn delete_task(conn: &Connection, task_id: i64) -> Result<()> {
    conn.execute("DELETE FROM tasks WHERE id = ?1", rusqlite::params![task_id])?;
    Ok(())
}

/// 清空某类型的历史记录
pub fn clear_tasks_by_type(conn: &Connection, task_type: &str) -> Result<()> {
    conn.execute(
        "DELETE FROM tasks WHERE task_type = ?1 AND status IN ('completed', 'failed', 'cancelled')",
        rusqlite::params![task_type],
    )?;
    Ok(())
}
