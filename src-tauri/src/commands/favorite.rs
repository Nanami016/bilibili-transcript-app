// 收藏夹相关命令

use tauri::command;

use crate::bilibili::types::{FavoriteFolder, VideoInfo};

/// 获取收藏夹列表
#[command]
pub async fn get_favorites() -> Result<Vec<FavoriteFolder>, String> {
    // TODO: 调用 API 获取收藏夹列表
    todo!("实现收藏夹列表命令")
}

/// 获取收藏夹中的视频列表
#[command]
pub async fn get_favorite_videos(media_id: String) -> Result<Vec<VideoInfo>, String> {
    // TODO: 获取收藏夹视频列表
    todo!("实现收藏夹视频列表命令")
}
