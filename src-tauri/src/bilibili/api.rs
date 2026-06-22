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
        "https://api.bilibili.com/x/v3/fav/folder/created/list-all?up_mid={}",
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

/// 收藏夹视频分页结果
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct FavoritePage {
    pub videos: Vec<VideoInfo>,
    pub has_more: bool,
    pub page: i64,
}

/// 获取收藏夹中的视频列表（分页）
pub async fn get_favorite_videos(media_id: i64, page: i64, cookie: &str) -> Result<FavoritePage> {
    log::info!("获取收藏夹视频列表: media_id={}, page={}", media_id, page);
    let url = format!(
        "https://api.bilibili.com/x/v3/fav/resource/list?media_id={}&ps=20&pn={}",
        media_id, page
    );
    let data = bili_get(&url, cookie).await?;

    let has_more = data["data"]["has_more"].as_bool().unwrap_or(false);

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
            // B站返回的封面 URL 可能是 http://，强制转为 https://
            let cover_url = cover_url.replace("http://", "https://");

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

    log::info!("收藏夹视频列表获取成功: {} 个视频, has_more={}", videos.len(), has_more);
    Ok(FavoritePage { videos, has_more, page })
}

/// 将 Cookie 字符串写入 Netscape 格式的 cookie 文件
fn write_cookie_file(cookie: &str) -> Result<std::path::PathBuf> {
    let cookie_path = std::env::temp_dir().join("bilibili-transcript-cookies.txt");
    let mut lines = vec!["# Netscape HTTP Cookie File".to_string()];
    for part in cookie.split(';') {
        let part = part.trim();
        if let Some((name, value)) = part.split_once('=') {
            let name = name.trim();
            let value = value.trim();
            lines.push(format!(".bilibili.com\tTRUE\t/\tFALSE\t0\t{}\t{}", name, value));
        }
    }
    std::fs::write(&cookie_path, lines.join("\n"))?;
    Ok(cookie_path)
}

/// 获取视频可用格式列表（通过 yt-dlp）
pub async fn get_video_formats(url: &str, cookie: &str) -> Result<Vec<VideoFormat>> {
    log::info!("获取视频格式列表: url={}", url);
    use std::process::Command;

    let mut args = vec![
        "-F".to_string(),
        "--dump-json".to_string(),
        "--no-playlist".to_string(),
        "--user-agent".to_string(),
        USER_AGENT.to_string(),
        "--referer".to_string(),
        "https://www.bilibili.com".to_string(),
    ];

    // 通过 Cookie 文件传递
    let cookie_file = if !cookie.is_empty() {
        let path = write_cookie_file(cookie)?;
        args.push("--cookies".to_string());
        args.push(path.to_string_lossy().to_string());
        log::debug!("使用 Cookie 文件: {:?}", path);
        Some(path)
    } else {
        log::warn!("未配置 Cookie，格式列表可能不完整");
        None
    };

    args.push(url.to_string());

    let output = Command::new("yt-dlp")
        .args(&args)
        .output()?;

    // 清理 Cookie 文件
    if let Some(path) = &cookie_file {
        let _ = std::fs::remove_file(path);
    }

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log::error!("yt-dlp 获取格式失败: {}", stderr);
        bail!("yt-dlp 获取格式失败: {}", stderr);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    // 解析所有视频格式
    struct RawFormat {
        format_id: String,
        height: i64,
        ext: String,
        filesize: Option<i64>,
    }

    let mut raw_formats: Vec<RawFormat> = Vec::new();

    for line in stdout.lines() {
        if let Ok(data) = serde_json::from_str::<serde_json::Value>(line) {
            let format_id = data["format_id"].as_str().unwrap_or("").to_string();
            let vcodec = data["vcodec"].as_str().unwrap_or("").to_string();
            let ext = data["ext"].as_str().unwrap_or("").to_string();
            let filesize = data["filesize"].as_i64().or_else(|| data["filesize_approx"].as_i64());

            // 跳过纯音频格式
            if vcodec == "none" {
                continue;
            }

            // 从 height 字段获取高度，如果没有则从 resolution 解析
            let height = data["height"].as_i64().unwrap_or_else(|| {
                let resolution = data["resolution"].as_str().unwrap_or("");
                if let Some(h_str) = resolution.split('x').nth(1) {
                    h_str.parse::<i64>().unwrap_or(0)
                } else {
                    0
                }
            });

            if height > 0 {
                raw_formats.push(RawFormat {
                    format_id,
                    height,
                    ext,
                    filesize,
                });
            }
        }
    }

    // 按标准清晰度分组，每个清晰度保留最佳格式（优先 mp4，文件最大的）
    let standard_resolutions = [2160, 1440, 1080, 720, 480, 360];
    let mut formats: Vec<VideoFormat> = Vec::new();

    for &target_height in &standard_resolutions {
        // 找到最接近目标高度的格式（允许 ±10% 误差）
        let min_h = (target_height as f64 * 0.9) as i64;
        let max_h = (target_height as f64 * 1.1) as i64;

        let candidates: Vec<&RawFormat> = raw_formats
            .iter()
            .filter(|f| f.height >= min_h && f.height <= max_h)
            .collect();

        if candidates.is_empty() {
            continue;
        }

        // 优先选择 mp4 格式，然后选择 filesize 最大的
        let best = candidates
            .iter()
            .max_by_key(|f| {
                let ext_score = if f.ext == "mp4" { 1000 } else { 0 };
                let size_score = f.filesize.unwrap_or(0);
                ext_score + size_score
            })
            .unwrap();

        let quality = format!("{}P", target_height);
        let size_str = best
            .filesize
            .map(|s| {
                if s >= 1024 * 1024 * 1024 {
                    format!(" {:.1}GB", s as f64 / (1024.0 * 1024.0 * 1024.0))
                } else if s >= 1024 * 1024 {
                    format!(" {:.1}MB", s as f64 / (1024.0 * 1024.0))
                } else {
                    String::new()
                }
            })
            .unwrap_or_default();

        formats.push(VideoFormat {
            format_id: best.format_id.clone(),
            quality: quality.clone(),
            description: format!("{}{}", quality, size_str),
            filesize: best.filesize,
        });
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
