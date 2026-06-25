// 字幕解析
// SRT 格式解析、CC/AI 字幕获取

use anyhow::Result;
use reqwest::Client;

use super::types::Subtitle;

const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";

/// 尝试获取视频字幕（CC → AI 降级）
pub async fn try_get_subtitle(bvid: &str, cid: i64, cookie: &str) -> Result<Option<Subtitle>> {
    log::info!("尝试获取字幕: bvid={}, cid={}", bvid, cid);

    let subtitles = get_subtitle_list(bvid, cid, cookie).await?;
    log::debug!("获取到 {} 个字幕选项", subtitles.len());

    if subtitles.is_empty() {
        log::info!("该视频没有可用字幕");
        return Ok(None);
    }

    for s in &subtitles {
        log::debug!("字幕选项: lan={}, doc={}, url={}", s.lan, s.lan_doc, s.subtitle_url);
    }

    // 优先查找 CC 字幕（人工上传）
    let cc_subtitle = subtitles.iter().find(|s| {
        s.lan.starts_with("zh") && !s.lan.contains("ai")
    });

    // 其次查找 AI 字幕
    let ai_subtitle = subtitles.iter().find(|s| {
        s.lan.starts_with("zh") && s.lan.contains("ai")
    });

    // 最后查找任意中文字幕
    let any_subtitle = subtitles.iter().find(|s| s.lan.starts_with("zh"));

    let target = cc_subtitle
        .or(ai_subtitle)
        .or(any_subtitle)
        .or(subtitles.first());

    if let Some(item) = target {
        let source = if item.lan.contains("ai") {
            "ai".to_string()
        } else {
            "cc".to_string()
        };

        log::info!("选择字幕: lan={}, source={}", item.lan, source);
        let (content, duration_secs) = download_subtitle(&item.subtitle_url).await?;
        log::info!("字幕下载成功: 长度={}, 时长={:?}s", content.len(), duration_secs);
        return Ok(Some(Subtitle {
            language: item.lan_doc.clone(),
            content,
            source,
            duration_secs,
        }));
    }

    log::info!("未找到合适的字幕");
    Ok(None)
}

/// 获取字幕列表
async fn get_subtitle_list(
    bvid: &str,
    cid: i64,
    cookie: &str,
) -> Result<Vec<super::types::SubtitleItem>> {
    log::debug!("获取字幕列表: bvid={}, cid={}", bvid, cid);
    let client = Client::new();
    let url = format!(
        "https://api.bilibili.com/x/player/v2?bvid={}&cid={}",
        bvid, cid
    );

    let mut request = client
        .get(&url)
        .header("User-Agent", USER_AGENT)
        .header("Referer", "https://www.bilibili.com");

    if !cookie.is_empty() {
        request = request.header("Cookie", cookie);
    }

    let resp = request.send().await?;
    let data: serde_json::Value = resp.json().await?;

    let code = data["code"].as_i64().unwrap_or(-1);
    if code != 0 {
        let msg = data["message"].as_str().unwrap_or("Unknown error");
        log::warn!("获取字幕列表失败 ({}): {}", code, msg);
        anyhow::bail!("获取字幕列表失败 ({}): {}", code, msg);
    }

    let mut subtitles = Vec::new();
    if let Some(sub_list) = data["data"]["subtitle"]["subtitles"].as_array() {
        for item in sub_list {
            let lan = item["lan"].as_str().unwrap_or("").to_string();
            let subtitle_url = item["subtitle_url"].as_str().unwrap_or("").to_string();
            let lan_doc = item["lan_doc"].as_str().unwrap_or("").to_string();

            if !subtitle_url.is_empty() {
                subtitles.push(super::types::SubtitleItem {
                    lan,
                    subtitle_url,
                    lan_doc,
                });
            }
        }
    }

    log::debug!("字幕列表获取成功: {} 个字幕", subtitles.len());
    Ok(subtitles)
}

/// 下载字幕内容并转为纯文本
/// 返回 (纯文本, 字幕末尾时间戳秒数)
async fn download_subtitle(url: &str) -> Result<(String, Option<f64>)> {
    log::debug!("下载字幕: {}", url);
    let client = Client::new();

    let full_url = if url.starts_with("//") {
        format!("https:{}", url)
    } else if url.starts_with("http") {
        url.to_string()
    } else {
        format!("https:{}", url)
    };

    let resp = client
        .get(&full_url)
        .header("User-Agent", USER_AGENT)
        .send()
        .await?;

    let data: serde_json::Value = resp.json().await?;

    let mut lines = Vec::new();
    let mut last_to: Option<f64> = None;
    if let Some(body) = data["body"].as_array() {
        for item in body {
            if let Some(content) = item["content"].as_str() {
                lines.push(content.to_string());
            }
            if let Some(to) = item["to"].as_f64() {
                last_to = Some(to);
            }
        }
    }

    log::debug!("字幕解析完成: {} 行, 末尾时间戳: {:?}s", lines.len(), last_to);
    Ok((lines.join("\n"), last_to))
}

/// 解析 SRT 格式字幕为纯文本
pub fn parse_srt(srt_content: &str) -> String {
    let mut lines = Vec::new();
    for line in srt_content.lines() {
        if line.trim().is_empty()
            || line.trim().parse::<u32>().is_ok()
            || line.contains("-->")
        {
            continue;
        }
        lines.push(line.trim());
    }
    lines.join("\n")
}
