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
        self.conn.execute(
            "INSERT INTO transcripts (bvid, url, title, author, duration, upload_date, transcript_source, transcript_text, summary, status, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, datetime('now', 'localtime'))
             ON CONFLICT(bvid) DO UPDATE SET
                url = excluded.url,
                title = excluded.title,
                author = excluded.author,
                duration = excluded.duration,
                upload_date = excluded.upload_date,
                transcript_source = excluded.transcript_source,
                transcript_text = excluded.transcript_text,
                summary = COALESCE(excluded.summary, transcripts.summary),
                status = excluded.status,
                updated_at = datetime('now', 'localtime')",
            rusqlite::params![
                record.bvid,
                record.url,
                record.title,
                record.author,
                record.duration,
                record.upload_date,
                record.transcript_source,
                record.transcript_text,
                record.summary,
                record.status,
            ],
        )?;
        Ok(())
    }

    /// 根据 bvid 查询
    pub fn get_by_bvid(&self, bvid: &str) -> Result<Option<TranscriptRecord>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, bvid, url, title, author, duration, upload_date, transcript_source, transcript_text, summary, status, created_at, updated_at
             FROM transcripts WHERE bvid = ?1"
        )?;

        let mut rows = stmt.query_map(rusqlite::params![bvid], |row| {
            Ok(TranscriptRecord {
                id: row.get(0)?,
                bvid: row.get(1)?,
                url: row.get(2)?,
                title: row.get(3)?,
                author: row.get(4)?,
                duration: row.get(5)?,
                upload_date: row.get(6)?,
                transcript_source: row.get(7)?,
                transcript_text: row.get(8)?,
                summary: row.get(9)?,
                status: row.get(10)?,
                created_at: row.get(11)?,
                updated_at: row.get(12)?,
            })
        })?;

        match rows.next() {
            Some(row) => Ok(Some(row?)),
            None => Ok(None),
        }
    }

    /// 获取所有记录
    pub fn get_all(&self) -> Result<Vec<TranscriptRecord>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, bvid, url, title, author, duration, upload_date, transcript_source, transcript_text, summary, status, created_at, updated_at
             FROM transcripts ORDER BY updated_at DESC"
        )?;

        let rows = stmt.query_map([], |row| {
            Ok(TranscriptRecord {
                id: row.get(0)?,
                bvid: row.get(1)?,
                url: row.get(2)?,
                title: row.get(3)?,
                author: row.get(4)?,
                duration: row.get(5)?,
                upload_date: row.get(6)?,
                transcript_source: row.get(7)?,
                transcript_text: row.get(8)?,
                summary: row.get(9)?,
                status: row.get(10)?,
                created_at: row.get(11)?,
                updated_at: row.get(12)?,
            })
        })?;

        let mut records = Vec::new();
        for row in rows {
            records.push(row?);
        }

        Ok(records)
    }

    /// 更新摘要
    pub fn update_summary(&self, bvid: &str, summary: &str) -> Result<()> {
        self.conn.execute(
            "UPDATE transcripts SET summary = ?1, updated_at = datetime('now', 'localtime') WHERE bvid = ?2",
            rusqlite::params![summary, bvid],
        )?;
        Ok(())
    }

    /// 删除记录
    pub fn delete(&self, bvid: &str) -> Result<()> {
        self.conn.execute(
            "DELETE FROM transcripts WHERE bvid = ?1",
            rusqlite::params![bvid],
        )?;
        Ok(())
    }
}
