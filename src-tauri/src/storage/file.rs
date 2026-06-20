// TXT 文件渲染输出
// 从数据库记录生成格式化的 TXT 文件

use anyhow::Result;
use std::path::PathBuf;

use super::database::TranscriptRecord;

/// 渲染 TXT 文件
pub fn render_txt(record: &TranscriptRecord, output_dir: &PathBuf) -> Result<PathBuf> {
    // 按发布年月组织目录
    let year = &record.upload_date[..4.min(record.upload_date.len())];
    let month = &record.upload_date[5..7.min(record.upload_date.len())];
    let dir = output_dir.join(year).join(month);
    std::fs::create_dir_all(&dir)?;

    // 生成文件名
    let filename = format!(
        "{}_{}_{}_{}.txt",
        sanitize_filename(&record.title),
        sanitize_filename(&record.author),
        record.upload_date,
        record.bvid
    );
    let filepath = dir.join(filename);

    // 渲染内容
    let summary = record.summary.as_deref().unwrap_or("【AI待处理：请阅读全文后，替换此行，写结构化摘要】");
    let content = format!(
        r#"================================================================================
B站视频转录文档
================================================================================

📹 视频标题：{}
🔗 B站链接：{}
👤 作者：{}
📅 发布时间：{}
⏱️  视频时长：{}
📝 转录来源：{}
⏰ 转录时间：{}

================================================================================
第一部分：视频摘要（AI生成）
================================================================================

{}

================================================================================
第二部分：完整原文
================================================================================

{}
================================================================================
文档结束
================================================================================
"#,
        record.title, record.url, record.author, record.upload_date,
        record.duration, record.transcript_source, record.created_at,
        summary, record.transcript_text
    );

    std::fs::write(&filepath, content)?;
    Ok(filepath)
}

/// 清理文件名中的非法字符
fn sanitize_filename(s: &str) -> String {
    s.chars()
        .filter(|c| !matches!(c, '\\' | '/' | ':' | '*' | '?' | '"' | '<' | '>' | '|'))
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("-")
}
