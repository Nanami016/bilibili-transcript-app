// Bilibili Transcript App - Tauri 入口
// macOS 原生应用，支持 B站视频转录、字幕获取、Whisper 语音转文字

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod bilibili;
mod commands;
mod config;
mod download;
mod storage;
mod summary;
mod transcribe;

use config::AppConfig;

fn main() {
    // 初始化日志
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppConfig::default())
        .invoke_handler(tauri::generate_handler![
            // 视频相关
            commands::video::parse_video,
            commands::video::get_video_formats,
            commands::video::download_video,
            commands::video::download_audio,
            // 转录相关
            commands::transcribe::transcribe,
            commands::transcribe::test_whisper_connection,
            // 收藏夹相关
            commands::favorite::get_favorites,
            commands::favorite::get_favorite_videos,
            // 配置相关
            commands::config::get_config,
            commands::config::update_config,
            // Cookie 相关
            commands::cookie::import_cookie,
            commands::cookie::get_cookie_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
