// AI 摘要生成
// 调用 OpenAI 兼容 API 生成结构化摘要

use anyhow::Result;
use reqwest::Client;

/// 自动补全 API URL
/// 用户可只填 base URL（如 https://api.example.com/v1），代码自动追加 /chat/completions
fn resolve_api_url(url: &str) -> String {
    if url.ends_with("/chat/completions") {
        url.to_string()
    } else {
        format!("{}/chat/completions", url.trim_end_matches('/'))
    }
}

/// 生成视频摘要
/// title: 视频标题
/// transcript: 转录文本
/// api_url: API 地址（可只填 base URL，自动追加 /chat/completions）
/// api_key: API Key
/// model: 模型名称
/// prompt: 可选的 system prompt 补充指令
/// context: 可选的上下文文本
pub async fn generate_summary(
    title: &str,
    transcript: &str,
    api_url: &str,
    api_key: &str,
    model: &str,
    prompt: Option<&str>,
    context: Option<&str>,
) -> Result<String> {
    let client = Client::new();

    let base_system = "你是一个视频摘要助手。请对以下转录文本生成结构化摘要，包含：1) 核心观点 2) 主要论点 3) 关键结论。用中文回复，简洁明了。";
    let system_prompt = match prompt {
        Some(p) if !p.is_empty() => format!("{}\n\n额外要求：{}", base_system, p),
        _ => base_system.to_string(),
    };

    let truncated = &transcript[..transcript.len().min(30000)];
    let user_prompt = match context {
        Some(ctx) if !ctx.is_empty() => {
            format!("上下文信息：{}\n\n视频标题：{}\n\n转录文本：\n{}", ctx, title, truncated)
        }
        _ => format!("视频标题：{}\n\n转录文本：\n{}", title, truncated),
    };

    let payload = serde_json::json!({
        "model": model,
        "messages": [
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": user_prompt}
        ],
        "max_tokens": 1024
    });

    let url = resolve_api_url(api_url);
    log::debug!("AI 摘要请求 URL: {}", url);

    let resp = client
        .post(&url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&payload)
        .send()
        .await?;

    let status = resp.status();
    log::debug!("AI 摘要 API 响应: status={}", status);

    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        log::error!("AI 摘要 API 请求失败 ({}): {}", status, body);
        anyhow::bail!("AI 摘要 API 请求失败 ({}): {}", status, body);
    }

    let body = resp.text().await?;
    log::debug!("AI 摘要 API 响应体: {} bytes", body.len());

    let data: serde_json::Value = serde_json::from_str(&body)
        .map_err(|e| {
            log::error!("AI 摘要响应 JSON 解析失败: {}, 响应内容: {}", e, body);
            anyhow::anyhow!("AI 摘要响应 JSON 解析失败: {}", e)
        })?;

    let summary = data["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid API response format"))?;

    Ok(summary.to_string())
}
