// 收藏夹相关命令

use tauri::command;

use crate::bilibili;
use crate::bilibili::api::FavoritePage;
use crate::bilibili::types::FavoriteFolder;

/// 获取收藏夹列表
#[command]
pub async fn get_favorites() -> Result<Vec<FavoriteFolder>, String> {
    let config = crate::config::storage::load_config().map_err(|e| e.to_string())?;

    if config.bilibili.cookie.is_empty() {
        return Err("未配置 B站 Cookie，请先在设置中导入 Cookie".to_string());
    }

    bilibili::api::get_favorites(&config.bilibili.cookie)
        .await
        .map_err(|e| e.to_string())
}

/// 获取收藏夹中的视频列表（分页）
#[command]
pub async fn get_favorite_videos(media_id: String, page: Option<i64>) -> Result<FavoritePage, String> {
    let config = crate::config::storage::load_config().map_err(|e| e.to_string())?;

    if config.bilibili.cookie.is_empty() {
        return Err("未配置 B站 Cookie，请先在设置中导入 Cookie".to_string());
    }

    let mid: i64 = media_id.parse().map_err(|_| "无效的收藏夹 ID".to_string())?;
    let pn = page.unwrap_or(1);

    bilibili::api::get_favorite_videos(mid, pn, &config.bilibili.cookie)
        .await
        .map_err(|e| e.to_string())
}
