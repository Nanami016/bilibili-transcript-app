// 字幕解析
// SRT 格式解析、CC/AI 字幕获取

use anyhow::Result;

use super::types::Subtitle;

/// 尝试获取视频字幕（CC → AI 降级）
pub async fn try_get_subtitle(bvid: &str, cid: i64, cookie: &str) -> Result<Option<Subtitle>> {
    // 1. 先尝试获取 CC 字幕
    // 2. 没有 CC 则尝试 AI 字幕
    // 3. 都没有返回 None
    todo!("实现字幕获取降级逻辑")
}

/// 解析 SRT 格式字幕为纯文本
pub fn parse_srt(srt_content: &str) -> String {
    let mut lines = Vec::new();
    for line in srt_content.lines() {
        // 跳过序号行和时间轴行
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
