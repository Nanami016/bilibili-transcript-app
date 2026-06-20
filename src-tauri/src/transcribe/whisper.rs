// Whisper API 客户端
// 支持 OpenAI 格式和本地 REST API

use anyhow::Result;
use reqwest::Client;
use std::path::Path;

/// Whisper 客户端 trait
#[async_trait::async_trait]
pub trait WhisperClient {
    async fn transcribe(&self, audio_path: &Path) -> Result<String>;
    async fn test_connection(&self) -> Result<bool>;
}

/// OpenAI 格式客户端（兼容远程 + 本地 OpenAI-compatible server）
pub struct OpenAIWhisperClient {
    pub api_url: String,
    pub api_key: Option<String>,
    pub model: String,
}

impl OpenAIWhisperClient {
    pub fn new(api_url: String, api_key: Option<String>, model: String) -> Self {
        Self {
            api_url,
            api_key,
            model,
        }
    }
}

#[async_trait::async_trait]
impl WhisperClient for OpenAIWhisperClient {
    async fn transcribe(&self, audio_path: &Path) -> Result<String> {
        let client = Client::new();

        // 读取音频文件
        let audio_data = tokio::fs::read(audio_path).await?;

        // 构建 multipart 请求
        let file_part = reqwest::multipart::Part::bytes(audio_data)
            .file_name("audio.wav")
            .mime_str("audio/wav")?;

        let form = reqwest::multipart::Form::new()
            .part("file", file_part)
            .text("model", self.model.clone());

        let mut request = client.post(&self.api_url).multipart(form);

        if let Some(key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", key));
        }

        let resp = request.send().await?;
        let data: serde_json::Value = resp.json().await?;

        // 解析响应
        let text = data["text"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid response format"))?;

        Ok(text.to_string())
    }

    async fn test_connection(&self) -> Result<bool> {
        // TODO: 发送测试请求验证连接
        todo!("实现连接测试")
    }
}
