// 任务管理器
// 管理后台任务的生命周期，支持进度推送和取消

use std::collections::HashMap;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

use crate::storage::database::Database;
use crate::storage::task;

/// 活跃任务信息
struct ActiveTask {
    handle: JoinHandle<()>,
}

/// 任务管理器
pub struct TaskManager {
    active_tasks: Arc<Mutex<HashMap<i64, ActiveTask>>>,
}

impl Default for TaskManager {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            active_tasks: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// 启动视频下载任务
    pub async fn start_video_download(
        &self,
        app: AppHandle,
        url: String,
        format_id: String,
    ) -> Result<i64, String> {
        let db = Database::open().map_err(|e| e.to_string())?;
        let task_id = task::insert_task(db.conn(), "video_download", &url, "", "")
            .map_err(|e| e.to_string())?;

        let active = self.active_tasks.clone();
        let app_clone = app.clone();

        let handle = tokio::spawn(async move {
            // 加载配置
            let config = match crate::config::storage::load_config() {
                Ok(c) => c,
                Err(e) => {
                    let _ = Self::mark_failed(&app_clone, task_id, "video_download", &format!("加载配置失败: {}", e));
                    active.lock().await.remove(&task_id);
                    return;
                }
            };

            let output_dir = std::path::PathBuf::from(
                shellexpand::tilde(&config.bilibili.video_dir).to_string()
            );

            // 发送开始事件
            let _ = app_clone.emit("task-progress", task::TaskProgressEvent {
                task_id,
                task_type: "video_download".to_string(),
                status: "running".to_string(),
                progress: 0.0,
                speed: String::new(),
                eta: "计算中...".to_string(),
            });

            // 构建进度回调
            let app_progress = app_clone.clone();
            let progress_cb: crate::download::video::ProgressCallback = Box::new(move |pct, speed, eta| {
                let _ = app_progress.emit("task-progress", task::TaskProgressEvent {
                    task_id,
                    task_type: "video_download".to_string(),
                    status: "running".to_string(),
                    progress: pct,
                    speed,
                    eta,
                });
            });

            // 执行下载
            match crate::download::video::download_video(&url, &format_id, &output_dir, &config.bilibili.cookie, Some(progress_cb)).await {
                Ok(path) => {
                    let file_size = Self::format_file_size(&path);
                    let title = path.file_stem()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_default();

                    // 更新标题
                    let db = Database::open().ok();
                    if let Some(db) = db {
                        let _ = task::update_task_title(db.conn(), task_id, &title);
                    }

                    let _ = Self::mark_completed(&app_clone, task_id, "video_download", &title, Some(&path.to_string_lossy()), file_size.as_deref());
                }
                Err(e) => {
                    let _ = Self::mark_failed(&app_clone, task_id, "video_download", &e.to_string());
                }
            }

            active.lock().await.remove(&task_id);
        });

        self.active_tasks.lock().await.insert(task_id, ActiveTask { handle });
        Ok(task_id)
    }

    /// 启动音频下载任务
    pub async fn start_audio_download(
        &self,
        app: AppHandle,
        url: String,
    ) -> Result<i64, String> {
        let db = Database::open().map_err(|e| e.to_string())?;
        let task_id = task::insert_task(db.conn(), "audio_download", &url, "", "")
            .map_err(|e| e.to_string())?;

        let active = self.active_tasks.clone();
        let app_clone = app.clone();

        let handle = tokio::spawn(async move {
            let config = match crate::config::storage::load_config() {
                Ok(c) => c,
                Err(e) => {
                    let _ = Self::mark_failed(&app_clone, task_id, "audio_download", &format!("加载配置失败: {}", e));
                    active.lock().await.remove(&task_id);
                    return;
                }
            };

            let output_dir = std::path::PathBuf::from(
                shellexpand::tilde(&config.bilibili.audio_dir).to_string()
            );

            let _ = app_clone.emit("task-progress", task::TaskProgressEvent {
                task_id,
                task_type: "audio_download".to_string(),
                status: "running".to_string(),
                progress: 0.0,
                speed: String::new(),
                eta: "计算中...".to_string(),
            });

            // 构建进度回调
            let app_progress = app_clone.clone();
            let progress_cb: crate::download::audio::ProgressCallback = Box::new(move |pct, speed, eta| {
                let _ = app_progress.emit("task-progress", task::TaskProgressEvent {
                    task_id,
                    task_type: "audio_download".to_string(),
                    status: "running".to_string(),
                    progress: pct,
                    speed,
                    eta,
                });
            });

            match crate::download::audio::download_audio(&url, &output_dir, &config.bilibili.cookie, Some(progress_cb)).await {
                Ok(path) => {
                    let file_size = Self::format_file_size(&path);
                    let title = path.file_stem()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_default();

                    let db = Database::open().ok();
                    if let Some(db) = db {
                        let _ = task::update_task_title(db.conn(), task_id, &title);
                    }

                    let _ = Self::mark_completed(&app_clone, task_id, "audio_download", &title, Some(&path.to_string_lossy()), file_size.as_deref());
                }
                Err(e) => {
                    let _ = Self::mark_failed(&app_clone, task_id, "audio_download", &e.to_string());
                }
            }

            active.lock().await.remove(&task_id);
        });

        self.active_tasks.lock().await.insert(task_id, ActiveTask { handle });
        Ok(task_id)
    }

    /// 启动转录任务
    pub async fn start_transcribe(
        &self,
        app: AppHandle,
        url: String,
        language: Option<String>,
        whisper_prompt: Option<String>,
        ai_prompt: Option<String>,
        ai_context: Option<String>,
    ) -> Result<i64, String> {
        let db = Database::open().map_err(|e| e.to_string())?;
        let task_id = task::insert_task(db.conn(), "transcribe", &url, "", "")
            .map_err(|e| e.to_string())?;

        let active = self.active_tasks.clone();
        let app_clone = app.clone();

        let handle = tokio::spawn(async move {
            let config = match crate::config::storage::load_config() {
                Ok(c) => c,
                Err(e) => {
                    let _ = Self::mark_failed(&app_clone, task_id, "transcribe", &format!("加载配置失败: {}", e));
                    active.lock().await.remove(&task_id);
                    return;
                }
            };

            // 提取 BVid
            let bvid = match crate::bilibili::api::extract_bvid(&url) {
                Some(b) => b,
                None => {
                    let _ = Self::mark_failed(&app_clone, task_id, "transcribe", "无法从 URL 中提取 BVid");
                    active.lock().await.remove(&task_id);
                    return;
                }
            };

            // 获取视频信息
            let video_info = match crate::bilibili::api::get_video_info(&bvid, &config.bilibili.cookie).await {
                Ok(info) => info,
                Err(e) => {
                    let _ = Self::mark_failed(&app_clone, task_id, "transcribe", &format!("获取视频信息失败: {}", e));
                    active.lock().await.remove(&task_id);
                    return;
                }
            };

            // 更新标题
            {
                let db = Database::open().ok();
                if let Some(db) = db {
                    let _ = task::update_task_title(db.conn(), task_id, &video_info.title);
                }
            }

            let _ = app_clone.emit("task-progress", task::TaskProgressEvent {
                task_id,
                task_type: "transcribe".to_string(),
                status: "running".to_string(),
                progress: 5.0,
                speed: String::new(),
                eta: "正在准备...".to_string(),
            });

            // 记录转录开始时间
            let transcribe_start = std::time::Instant::now();

            // 构建进度回调
            let app_progress = app_clone.clone();
            let progress_cb: crate::transcribe::pipeline::ProgressCallback = Box::new(move |pct, speed, eta| {
                let _ = app_progress.emit("task-progress", task::TaskProgressEvent {
                    task_id,
                    task_type: "transcribe".to_string(),
                    status: "running".to_string(),
                    progress: pct,
                    speed,
                    eta,
                });
            });

            // 执行转录
            let wp = whisper_prompt.as_deref();
            let lang = language.as_deref();
            match crate::transcribe::pipeline::transcribe_video(
                &url, &bvid, video_info.cid, &config.bilibili.cookie, &config, wp, lang, Some(progress_cb),
            ).await {
                Ok(result) => {
                    // 计算转录耗时
                    let elapsed = transcribe_start.elapsed();
                    let duration_label = Self::format_elapsed(elapsed);

                    // 存储到数据库
                    let db = Database::open().ok();
                    if let Some(db) = db {
                        let video_duration = format!("{}:{:02}", video_info.duration / 60, video_info.duration % 60);
                        let record = crate::storage::database::TranscriptRecord {
                            id: 0,
                            bvid: bvid.clone(),
                            url: url.clone(),
                            title: video_info.title.clone(),
                            author: video_info.author.clone(),
                            duration: video_duration,
                            upload_date: video_info.upload_date.clone(),
                            transcript_source: result.source.clone(),
                            transcript_text: result.text.clone(),
                            summary: None,
                            status: "transcribed".to_string(),
                            created_at: String::new(),
                            updated_at: String::new(),
                        };
                        let _ = db.upsert(&record);

                        // 渲染 TXT
                        let output_dir = std::path::PathBuf::from(
                            shellexpand::tilde(&config.bilibili.transcript_dir).to_string()
                        );
                        let _ = crate::storage::file::render_txt(&record, &output_dir);
                    }

                    let source_label = match result.source.as_str() {
                        "cc" => "CC字幕",
                        "ai" => "AI字幕",
                        _ => "Whisper",
                    };
                    let file_info = format!("{} · 耗时{}", source_label, duration_label);
                    let _ = Self::mark_completed(&app_clone, task_id, "transcribe", &video_info.title, None, Some(&file_info));

                    // 如果启用了 AI 摘要，自动触发
                    if config.ai_summary.enabled {
                        let _ = app_clone.emit("task-progress", task::TaskProgressEvent {
                            task_id,
                            task_type: "transcribe".to_string(),
                            status: "running".to_string(),
                            progress: 80.0,
                            speed: String::new(),
                            eta: "正在生成AI摘要...".to_string(),
                        });

                        let ap = ai_prompt.as_deref();
                        let ac = ai_context.as_deref();
                        match Self::run_ai_summary(&bvid, &config, ap, ac).await {
                            Ok(summary) => {
                                let db = Database::open().ok();
                                if let Some(db) = db {
                                    let _ = db.update_summary(&bvid, &summary);
                                    // 重新渲染 TXT（包含摘要）
                                    if let Ok(Some(record)) = db.get_by_bvid(&bvid) {
                                        let output_dir = std::path::PathBuf::from(
                                            shellexpand::tilde(&config.bilibili.transcript_dir).to_string()
                                        );
                                        let _ = crate::storage::file::render_txt(&record, &output_dir);
                                    }
                                }
                                log::info!("AI 摘要生成成功: bvid={}", bvid);
                            }
                            Err(e) => {
                                log::warn!("AI 摘要生成失败（不影响转录结果）: {}", e);
                            }
                        }
                    }
                }
                Err(e) => {
                    let _ = Self::mark_failed(&app_clone, task_id, "transcribe", &e.to_string());
                }
            }

            active.lock().await.remove(&task_id);
        });

        self.active_tasks.lock().await.insert(task_id, ActiveTask { handle });
        Ok(task_id)
    }

    /// 启动 AI 摘要任务
    pub async fn start_ai_summary(
        &self,
        app: AppHandle,
        bvid: String,
    ) -> Result<i64, String> {
        let db = Database::open().map_err(|e| e.to_string())?;
        let url = format!("https://www.bilibili.com/video/{}", bvid);
        let task_id = task::insert_task(db.conn(), "ai_summary", &url, "", "")
            .map_err(|e| e.to_string())?;

        let active = self.active_tasks.clone();
        let app_clone = app.clone();

        let handle = tokio::spawn(async move {
            let config = match crate::config::storage::load_config() {
                Ok(c) => c,
                Err(e) => {
                    let _ = Self::mark_failed(&app_clone, task_id, "ai_summary", &format!("加载配置失败: {}", e));
                    active.lock().await.remove(&task_id);
                    return;
                }
            };

            // 从数据库获取转录文本
            let db = match Database::open() {
                Ok(db) => db,
                Err(e) => {
                    let _ = Self::mark_failed(&app_clone, task_id, "ai_summary", &format!("数据库打开失败: {}", e));
                    active.lock().await.remove(&task_id);
                    return;
                }
            };

            let record = match db.get_by_bvid(&bvid) {
                Ok(Some(r)) => r,
                Ok(None) => {
                    let _ = Self::mark_failed(&app_clone, task_id, "ai_summary", "未找到该视频的转录记录，请先转录");
                    active.lock().await.remove(&task_id);
                    return;
                }
                Err(e) => {
                    let _ = Self::mark_failed(&app_clone, task_id, "ai_summary", &format!("查询失败: {}", e));
                    active.lock().await.remove(&task_id);
                    return;
                }
            };

            // 更新标题
            let _ = task::update_task_title(db.conn(), task_id, &record.title);

            let _ = app_clone.emit("task-progress", task::TaskProgressEvent {
                task_id,
                task_type: "ai_summary".to_string(),
                status: "running".to_string(),
                progress: 20.0,
                speed: String::new(),
                eta: "正在生成摘要...".to_string(),
            });

            // 检查 AI 摘要配置
            if config.ai_summary.api_key.is_none() || config.ai_summary.api_key.as_deref() == Some("") {
                let _ = Self::mark_failed(&app_clone, task_id, "ai_summary", "未配置 AI 摘要 API Key，请在设置中配置");
                active.lock().await.remove(&task_id);
                return;
            }

            let api_key = config.ai_summary.api_key.unwrap_or_default();
            let prompt = if config.ai_summary.prompt.is_empty() { None } else { Some(config.ai_summary.prompt.as_str()) };
            let context = if config.ai_summary.context.is_empty() { None } else { Some(config.ai_summary.context.as_str()) };

            match crate::summary::openai::generate_summary(
                &record.title,
                &record.transcript_text,
                &config.ai_summary.api_url,
                &api_key,
                &config.ai_summary.model,
                prompt,
                context,
            ).await {
                Ok(summary) => {
                    // 更新数据库
                    let _ = db.update_summary(&bvid, &summary);
                    let _ = Self::mark_completed(&app_clone, task_id, "ai_summary", &record.title, None, None);
                }
                Err(e) => {
                    let _ = Self::mark_failed(&app_clone, task_id, "ai_summary", &e.to_string());
                }
            }

            active.lock().await.remove(&task_id);
        });

        self.active_tasks.lock().await.insert(task_id, ActiveTask { handle });
        Ok(task_id)
    }

    /// 取消任务
    pub async fn cancel_task(&self, task_id: i64) -> Result<(), String> {
        let mut active = self.active_tasks.lock().await;
        if let Some(task) = active.remove(&task_id) {
            task.handle.abort();

            let db = Database::open().map_err(|e| e.to_string())?;
            task::update_task_status(db.conn(), task_id, "cancelled", None, None, None)
                .map_err(|e| e.to_string())?;

            Ok(())
        } else {
            Err("任务不存在或已完成".to_string())
        }
    }

    /// 标记任务完成
    fn mark_completed(
        app: &AppHandle,
        task_id: i64,
        task_type: &str,
        title: &str,
        output_path: Option<&str>,
        file_size: Option<&str>,
    ) -> Result<(), String> {
        let db = Database::open().map_err(|e| e.to_string())?;
        task::update_task_status(db.conn(), task_id, "completed", output_path, None, file_size)
            .map_err(|e| e.to_string())?;

        let _ = app.emit("task-completed", task::TaskCompletedEvent {
            task_id,
            task_type: task_type.to_string(),
            status: "completed".to_string(),
            title: title.to_string(),
            output_path: output_path.map(|s| s.to_string()),
            error: None,
            file_size: file_size.map(|s| s.to_string()),
        });

        Ok(())
    }

    /// 标记任务失败
    fn mark_failed(
        app: &AppHandle,
        task_id: i64,
        task_type: &str,
        error: &str,
    ) -> Result<(), String> {
        let db = Database::open().map_err(|e| e.to_string())?;
        task::update_task_status(db.conn(), task_id, "failed", None, Some(error), None)
            .map_err(|e| e.to_string())?;

        let _ = app.emit("task-completed", task::TaskCompletedEvent {
            task_id,
            task_type: task_type.to_string(),
            status: "failed".to_string(),
            title: String::new(),
            output_path: None,
            error: Some(error.to_string()),
            file_size: None,
        });

        Ok(())
    }

    /// 格式化耗时
    fn format_elapsed(elapsed: std::time::Duration) -> String {
        let secs = elapsed.as_secs();
        if secs >= 60 {
            format!("{}分{}秒", secs / 60, secs % 60)
        } else {
            format!("{}秒", secs)
        }
    }

    /// 执行 AI 摘要（内部辅助方法）
    async fn run_ai_summary(bvid: &str, config: &crate::config::AppConfig, ai_prompt: Option<&str>, ai_context: Option<&str>) -> Result<String, String> {
        let api_key = config.ai_summary.api_key.as_deref()
            .filter(|k| !k.is_empty())
            .ok_or("未配置 AI 摘要 API Key")?;

        let db = Database::open().map_err(|e| e.to_string())?;
        let record = db.get_by_bvid(bvid)
            .map_err(|e| e.to_string())?
            .ok_or("未找到转录记录")?;

        let prompt = ai_prompt
            .filter(|p| !p.is_empty())
            .or(if config.ai_summary.prompt.is_empty() { None } else { Some(config.ai_summary.prompt.as_str()) });
        let context = ai_context
            .filter(|c| !c.is_empty())
            .or(if config.ai_summary.context.is_empty() { None } else { Some(config.ai_summary.context.as_str()) });

        crate::summary::openai::generate_summary(
            &record.title,
            &record.transcript_text,
            &config.ai_summary.api_url,
            api_key,
            &config.ai_summary.model,
            prompt,
            context,
        )
        .await
        .map_err(|e| e.to_string())
    }

    /// 格式化文件大小
    fn format_file_size(path: &std::path::Path) -> Option<String> {
        let metadata = std::fs::metadata(path).ok()?;
        let size = metadata.len();
        if size >= 1024 * 1024 * 1024 {
            Some(format!("{:.1} GB", size as f64 / (1024.0 * 1024.0 * 1024.0)))
        } else if size >= 1024 * 1024 {
            Some(format!("{:.1} MB", size as f64 / (1024.0 * 1024.0)))
        } else if size >= 1024 {
            Some(format!("{:.1} KB", size as f64 / 1024.0))
        } else {
            Some(format!("{} B", size))
        }
    }
}
