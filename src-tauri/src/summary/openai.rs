// AI 摘要生成
// 调用 OpenAI 兼容 API 生成结构化摘要

use anyhow::Result;
use reqwest::Client;

/// 生成视频摘要
/// title: 视频标题
/// transcript: 转录文本
/// api_url: API 地址
/// api_key: API Key
/// model: 模型名称
pub async fn generate_summary(
    title: &str,
    transcript: &str,
    api_url: &str,
    api_key: &str,
    model: &str,
) -> Result<String> {
    let client = Client::new();

    let system_prompt = "你是一个视频摘要助手。请对以下转录文本生成结构化摘要，包含：1) 核心观点 2) 主要论点 3) 关键结论。用中文回复，简洁明了。";
    let user_prompt = format!("视频标题：{}\n\n转录文本：\n{}", title, &transcript[..transcript.len().min(30000)]);

    let payload = serde_json::json!({
        "model": model,
        "messages": [
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": user_prompt}
        ],
        "max_tokens": 1024
    });

    let resp = client
        .post(api_url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&payload)
        .send()
        .await?;

    let data: serde_json::Value = resp.json().await?;

    let summary = data["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid API response format"))?;

    Ok(summary.to_string())
}
