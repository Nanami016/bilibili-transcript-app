// B站 API 调用
// 收藏夹列表、视频信息、字幕列表

use anyhow::{bail, Result};
use regex::Regex;
use reqwest::Client;

use super::types::*;

const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";

/// 从各种 B站 URL 格式中提取 BVid
pub fn extract_bvid(url: &str) -> Option<String> {
    let re = Regex::new(r"(BV[a-zA-Z0-9]+)").ok()?;
    let result = re.find(url).map(|m| m.as_str().to_string());
    log::debug!("extract_bvid: url={} -> {:?}", url, result);
    result
}

/// 通用 B站 API 请求
async fn bili_get(url: &str, cookie: &str) -> Result<serde_json::Value> {
    log::debug!("B站 API 请求: {}", url);
    let client = Client::new();
    let mut request = client
        .get(url)
        .header("User-Agent", USER_AGENT)
        .header("Referer", "https://www.bilibili.com");

    if !cookie.is_empty() {
        request = request.header("Cookie", cookie);
    }

    let resp = request.send().await?;
    let status = resp.status();
    log::debug!("B站 API 响应: status={}", status);

    let data: serde_json::Value = resp.json().await?;

    let code = data["code"].as_i64().unwrap_or(-1);
    if code != 0 {
        let msg = data["message"].as_str().unwrap_or("Unknown error");
        log::warn!("B站 API 错误 ({}): {}", code, msg);
        bail!("B站 API 错误 ({}): {}", code, msg);
    }

    log::debug!("B站 API 成功: code={}", code);
    Ok(data)
}

/// 获取视频信息
pub async fn get_video_info(bvid: &str, cookie: &str) -> Result<VideoInfo> {
    log::info!("获取视频信息: bvid={}", bvid);
    let url = format!("https://api.bilibili.com/x/web-interface/view?bvid={}", bvid);
    let data = bili_get(&url, cookie).await?;

    let info = &data["data"];
    let aid = info["aid"].as_i64().unwrap_or(0);
    let cid = info["cid"].as_i64().unwrap_or(0);
    let title = info["title"].as_str().unwrap_or("").to_string();
    let author = info["owner"]["name"].as_str().unwrap_or("").to_string();
    let duration = info["duration"].as_i64().unwrap_or(0);
    let description = info["desc"].as_str().unwrap_or("").to_string();
    let cover_url = info["pic"].as_str().unwrap_or("").to_string();
    // B站返回的封面 URL 可能是 http://，强制转为 https://
    let cover_url = cover_url.replace("http://", "https://");
    let pubdate = info["pubdate"].as_i64().unwrap_or(0);

    let upload_date = if pubdate > 0 {
        chrono::DateTime::from_timestamp(pubdate, 0)
            .map(|dt| dt.format("%Y-%m-%d").to_string())
            .unwrap_or_default()
    } else {
        String::new()
    };

    log::info!("视频信息获取成功: title={}, author={}, duration={}s", title, author, duration);

    Ok(VideoInfo {
        bvid: bvid.to_string(),
        aid,
        cid,
        title,
        author,
        duration,
        description,
        cover_url,
        upload_date,
    })
}

/// 从 cookie 中提取 uid (DedeUserID)
fn extract_uid_from_cookie(cookie: &str) -> Option<String> {
    for part in cookie.split(';') {
        let part = part.trim();
        if let Some(val) = part.strip_prefix("DedeUserID=") {
            let uid = val.trim();
            if !uid.is_empty() {
                log::debug!("从 Cookie 提取到 uid={}", uid);
                return Some(uid.to_string());
            }
        }
    }
    log::warn!("无法从 Cookie 中提取 DedeUserID");
    None
}

/// 获取用户收藏夹列表
pub async fn get_favorites(cookie: &str) -> Result<Vec<FavoriteFolder>> {
    log::info!("获取收藏夹列表...");
    let uid = extract_uid_from_cookie(cookie)
        .ok_or_else(|| anyhow::anyhow!("无法从 Cookie 中提取用户 ID (DedeUserID)，请检查 Cookie 是否正确"))?;

    let url = format!(
        "https://api.bilibili.com/x/v3/fav/folder/created?up_mid={}&ps=50&pn=1",
        uid
    );
    let data = bili_get(&url, cookie).await?;

    let mut folders = Vec::new();
    if let Some(list) = data["data"]["list"].as_array() {
        for item in list {
            let id = item["id"].as_i64().unwrap_or(0);
            let title = item["title"].as_str().unwrap_or("").to_string();
            let media_count = item["media_count"].as_i64().unwrap_or(0);
            folders.push(FavoriteFolder {
                id,
                title,
                media_count,
            });
        }
    }

    log::info!("收藏夹列表获取成功: {} 个收藏夹", folders.len());
    Ok(folders)
}

/// 获取收藏夹中的视频列表
pub async fn get_favorite_videos(media_id: i64, cookie: &str) -> Result<Vec<VideoInfo>> {
    log::info!("获取收藏夹视频列表: media_id={}", media_id);
    let url = format!(
        "https://api.bilibili.com/x/v3/fav/resource/list?media_id={}&ps=20&pn=1",
        media_id
    );
    let data = bili_get(&url, cookie).await?;

    let mut videos = Vec::new();
    if let Some(medias) = data["data"]["medias"].as_array() {
        for item in medias {
            let bvid = item["bvid"].as_str().unwrap_or("").to_string();
            let aid = item["id"].as_i64().unwrap_or(0);
            let title = item["title"].as_str().unwrap_or("").to_string();
            let author = item["upper"]["name"].as_str().unwrap_or("").to_string();
            let duration = item["duration"].as_i64().unwrap_or(0);
            let description = item["intro"].as_str().unwrap_or("").to_string();
            let cover_url = item["cover"].as_str().unwrap_or("").to_string();

            videos.push(VideoInfo {
                bvid,
                aid,
                cid: 0,
                title,
                author,
                duration,
                description,
                cover_url,
                upload_date: String::new(),
            });
        }
    }

    log::info!("收藏夹视频列表获取成功: {} 个视频", videos.len());
    Ok(videos)
}

/// 获取视频可用格式列表（通过 yt-dlp）
pub async fn get_video_formats(url: &str, _cookie: &str) -> Result<Vec<VideoFormat>> {
    log::info!("获取视频格式列表: url={}", url);
    use std::process::Command;

    let output = Command::new("yt-dlp")
        .args(["-F", "--dump-json", url])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::error!("yt-dlp 获取格式失败: {}", stderr);
        bail!("yt-dlp 获取格式失败: {}", stderr);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut formats = Vec::new();

    for line in stdout.lines() {
        if let Ok(data) = serde_json::from_str::<serde_json::Value>(line) {
            let format_id = data["format_id"].as_str().unwrap_or("").to_string();
            let ext = data["ext"].as_str().unwrap_or("").to_string();
            let resolution = data["resolution"].as_str().unwrap_or("").to_string();
            let note = data["format_note"].as_str().unwrap_or("").to_string();
            let filesize = data["filesize"].as_i64().or_else(|| data["filesize_approx"].as_i64());

            let description = if !note.is_empty() {
                format!("{} ({}) {}", resolution, ext, note)
            } else {
                format!("{} ({})", resolution, ext)
            };

            formats.push(VideoFormat {
                format_id,
                quality: resolution,
                description,
                filesize,
            });
        }
    }

    log::info!("视频格式列表获取成功: {} 个格式", formats.len());
    Ok(formats)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_bvid() {
        assert_eq!(
            extract_bvid("https://www.bilibili.com/video/BV1xx411c7mD"),
            Some("BV1xx411c7mD".to_string())
        );
        assert_eq!(
            extract_bvid("BV1xx411c7mD"),
            Some("BV1xx411c7mD".to_string())
        );
        assert_eq!(
            extract_bvid("https://m.bilibili.com/video/BV1xx411c7mD?p=1"),
            Some("BV1xx411c7mD".to_string())
        );
        assert_eq!(extract_bvid("https://www.bilibili.com"), None);
    }

    #[test]
    fn test_extract_uid_from_cookie() {
        assert_eq!(
            extract_uid_from_cookie("DedeUserID=12345; bili_jct=abc"),
            Some("12345".to_string())
        );
        assert_eq!(extract_uid_from_cookie("bili_jct=abc"), None);
    }
}
