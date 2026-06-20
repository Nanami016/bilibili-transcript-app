// B站 API 调用
// 收藏夹列表、视频信息、字幕列表

use anyhow::Result;
use reqwest::Client;

use super::types::{FavoriteFolder, VideoInfo, VideoFormat};

/// 获取视频信息
pub async fn get_video_info(bvid: &str, cookie: &str) -> Result<VideoInfo> {
    let client = Client::new();
    let url = format!("https://api.bilibili.com/x/web-interface/view?bvid={}", bvid);

    let mut request = client.get(&url)
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36");

    if !cookie.is_empty() {
        request = request.header("Cookie", cookie);
    }

    let resp = request.send().await?;
    let data: serde_json::Value = resp.json().await?;

    // TODO: 解析响应，提取视频信息
    todo!("实现视频信息解析")
}

/// 获取视频可用格式列表
pub async fn get_video_formats(bvid: &str, cookie: &str) -> Result<Vec<VideoFormat>> {
    // TODO: 实现格式列表获取
    todo!("实现格式列表获取")
}

/// 获取用户收藏夹列表
pub async fn get_favorites(cookie: &str) -> Result<Vec<FavoriteFolder>> {
    // TODO: 解析 cookie 获取 uid，然后调用收藏夹 API
    todo!("实现收藏夹列表获取")
}

/// 获取收藏夹中的视频列表
pub async fn get_favorite_videos(media_id: i64, cookie: &str) -> Result<Vec<VideoInfo>> {
    // TODO: 实现收藏夹视频列表获取
    todo!("实现收藏夹视频列表获取")
}
