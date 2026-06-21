// Bilibili Transcript App - Tauri 入口
// macOS 原生应用，支持 B站视频转录、字幕获取、Whisper 语音转文字

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code)]

mod bilibili;
mod commands;
mod config;
mod download;
mod storage;
mod summary;
mod transcribe;

use config::AppConfig;

fn main() {
    commands::log::init_logger();
    log::info!("Bilibili Transcript 启动中...");

    let app_config = config::storage::load_config().unwrap_or_else(|e| {
        log::warn!("加载配置失败，使用默认配置: {}", e);
        AppConfig::default()
    });

    log::info!("配置加载完成");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(app_config)
        .invoke_handler(tauri::generate_handler![
            commands::video::parse_video,
            commands::video::get_video_formats,
            commands::video::download_video,
            commands::video::download_audio,
            commands::video::fetch_cover,
            commands::transcribe::transcribe,
            commands::transcribe::test_whisper_connection,
            commands::favorite::get_favorites,
            commands::favorite::get_favorite_videos,
            commands::history::get_transcript_history,
            commands::history::get_transcript,
            commands::history::delete_transcript,
            commands::config::get_config,
            commands::config::update_config,
            commands::cookie::import_cookie,
            commands::cookie::get_cookie_status,
            commands::cookie::get_cookie,
            commands::cookie::import_cookie_from_browser,
            commands::log::get_run_logs,
            commands::log::clear_run_logs,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
