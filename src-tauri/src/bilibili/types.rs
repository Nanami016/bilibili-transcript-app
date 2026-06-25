// B站数据结构

use serde::{Deserialize, Serialize};

/// 视频信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoInfo {
    pub bvid: String,
    pub aid: i64,
    pub cid: i64,
    pub title: String,
    pub author: String,
    pub duration: i64,
    pub description: String,
    pub cover_url: String,
    pub upload_date: String,
}

/// 视频格式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoFormat {
    pub format_id: String,
    pub quality: String,
    pub description: String,
    pub filesize: Option<i64>,
}

/// 收藏夹
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FavoriteFolder {
    pub id: i64,
    pub title: String,
    pub media_count: i64,
}

/// 字幕
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subtitle {
    pub language: String,
    pub content: String,
    pub source: String, // "cc" | "ai"
    /// 字幕末尾时间戳（秒），用于校验字幕是否属于目标视频
    pub duration_secs: Option<f64>,
}

/// 字幕 URL 信息（从 B站 API 获取）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtitleItem {
    #[serde(default)]
    pub lan: String,
    #[serde(default)]
    pub subtitle_url: String,
    #[serde(default)]
    pub lan_doc: String,
}

/// B站 API 通用响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiliApiResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

/// 收藏夹列表响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FavFolderData {
    pub list: Vec<FavFolderItem>,
}

/// 收藏夹列表项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FavFolderItem {
    pub id: i64,
    pub title: String,
    pub media_count: i64,
}

/// 收藏夹内容响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FavResourceData {
    pub medias: Option<Vec<FavMediaItem>>,
}

/// 收藏夹内容项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FavMediaItem {
    pub bvid: String,
    pub id: i64,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub upper: Option<FavUpper>,
    #[serde(default)]
    pub duration: i64,
    #[serde(default)]
    pub intro: String,
    #[serde(default)]
    pub cover: String,
}

/// UP 主信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FavUpper {
    #[serde(default)]
    pub mid: i64,
    #[serde(default)]
    pub name: String,
}

/// 用户信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavData {
    #[serde(default)]
    pub mid: i64,
}
