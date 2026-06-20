// SQLite 数据库管理
// 存储转录元数据、全文、摘要

use anyhow::Result;
use rusqlite::Connection;
use std::path::PathBuf;

/// 转录记录
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TranscriptRecord {
    pub id: i64,
    pub bvid: String,
    pub url: String,
    pub title: String,
    pub author: String,
    pub duration: String,
    pub upload_date: String,
    pub transcript_source: String,
    pub transcript_text: String,
    pub summary: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

/// 数据库管理器
pub struct Database {
    conn: Connection,
}

impl Database {
    /// 打开数据库
    pub fn open() -> Result<Self> {
        let db_path = Self::db_path();
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(db_path)?;
        let db = Self { conn };
        db.init_table()?;
        Ok(db)
    }

    /// 数据库文件路径
    fn db_path() -> PathBuf {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        home.join(".bilibili-transcript").join("transcripts.db")
    }

    /// 初始化表结构
    fn init_table(&self) -> Result<()> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS transcripts (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                bvid TEXT UNIQUE NOT NULL,
                url TEXT,
                title TEXT,
                author TEXT,
                duration TEXT,
                upload_date TEXT,
                transcript_source TEXT,
                transcript_text TEXT,
                summary TEXT,
                status TEXT DEFAULT 'transcribed',
                created_at TEXT DEFAULT (datetime('now', 'localtime')),
                updated_at TEXT DEFAULT (datetime('now', 'localtime'))
            );"
        )?;
        Ok(())
    }

    /// 插入或更新转录记录
    pub fn upsert(&self, record: &TranscriptRecord) -> Result<()> {
        // TODO: 实现插入/更新逻辑
        todo!("实现数据库插入/更新")
    }

    /// 根据 bvid 查询
    pub fn get_by_bvid(&self, bvid: &str) -> Result<Option<TranscriptRecord>> {
        // TODO: 实现查询逻辑
        todo!("实现数据库查询")
    }

    /// 获取所有记录
    pub fn get_all(&self) -> Result<Vec<TranscriptRecord>> {
        // TODO: 实现查询所有记录
        todo!("实现数据库查询所有")
    }
}
