// 运行日志查看命令
// 捕获程序运行期间的所有日志

use once_cell::sync::Lazy;
use std::sync::Mutex;
use tauri::command;

/// 全局日志缓冲区
static LOG_BUFFER: Lazy<Mutex<Vec<LogEntry>>> = Lazy::new(|| Mutex::new(Vec::new()));

/// 日志条目
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String,
    pub message: String,
}

/// 向日志缓冲区追加一条日志
pub fn push_log(level: &str, message: &str) {
    let timestamp = chrono::Local::now().format("%H:%M:%S").to_string();
    let entry = LogEntry {
        timestamp,
        level: level.to_string(),
        message: message.to_string(),
    };
    if let Ok(mut buf) = LOG_BUFFER.lock() {
        buf.push(entry);
        // 限制最大 500 条
        if buf.len() > 500 {
            buf.remove(0);
        }
    }
}

/// 自定义日志后端，同时输出到 stderr 和内存缓冲区
pub struct AppLogger;

impl log::Log for AppLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Trace
            || metadata.target().starts_with("bilibili_transcript")
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let level = match record.level() {
                log::Level::Error => "ERROR",
                log::Level::Warn => "WARN",
                log::Level::Info => "INFO",
                log::Level::Debug => "DEBUG",
                log::Level::Trace => "TRACE",
            };
            let msg = format!("{}", record.args());
            // 输出到 stderr
            eprintln!("[{}] {}", level, msg);
            // 存入缓冲区
            push_log(level, &msg);
        }
    }

    fn flush(&self) {}
}

/// 初始化日志系统
pub fn init_logger() {
    let logger = Box::new(AppLogger);
    // 忽略错误（可能已被初始化）
    let _ = log::set_boxed_logger(logger);
    log::set_max_level(log::LevelFilter::Debug);
}

/// 获取所有运行日志
#[command]
pub async fn get_run_logs() -> Result<Vec<LogEntry>, String> {
    let buf = LOG_BUFFER.lock().map_err(|e| e.to_string())?;
    Ok(buf.clone())
}

/// 清空日志
#[command]
pub async fn clear_run_logs() -> Result<(), String> {
    let mut buf = LOG_BUFFER.lock().map_err(|e| e.to_string())?;
    buf.clear();
    Ok(())
}
