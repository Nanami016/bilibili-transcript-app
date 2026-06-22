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
    pub prompt: Option<String>,
    pub language: Option<String>,
}

impl OpenAIWhisperClient {
    pub fn new(api_url: String, api_key: Option<String>, model: String, prompt: Option<String>, language: Option<String>) -> Self {
        Self {
            api_url,
            api_key,
            model,
            prompt,
            language,
        }
    }
}

#[async_trait::async_trait]
impl WhisperClient for OpenAIWhisperClient {
    async fn transcribe(&self, audio_path: &Path) -> Result<String> {
        log::info!("Whisper 转录: {:?}", audio_path);
        log::debug!("API URL: {}, Model: {}", self.api_url, self.model);
        let client = Client::new();

        // 读取音频文件
        let audio_data = tokio::fs::read(audio_path).await?;
        log::debug!("音频文件大小: {} bytes", audio_data.len());

        // 根据文件扩展名确定 MIME 类型
        let mime = match audio_path.extension().and_then(|e| e.to_str()) {
            Some("wav") => "audio/wav",
            Some("mp3") => "audio/mpeg",
            Some("m4a") => "audio/mp4",
            Some("ogg") => "audio/ogg",
            Some("flac") => "audio/flac",
            _ => "audio/wav",
        };

        let filename = audio_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("audio.wav");

        // 构建 multipart 请求
        let file_part = reqwest::multipart::Part::bytes(audio_data)
            .file_name(filename.to_string())
            .mime_str(mime)?;

        let mut form = reqwest::multipart::Form::new()
            .part("file", file_part)
            .text("model", self.model.clone());

        if let Some(ref prompt) = self.prompt {
            if !prompt.is_empty() {
                form = form.text("prompt", prompt.clone());
            }
        }

        if let Some(ref lang) = self.language {
            if !lang.is_empty() {
                form = form.text("language", lang.clone());
            }
        }

        let mut request = client.post(&self.api_url).multipart(form);

        if let Some(key) = &self.api_key {
            if !key.is_empty() {
                request = request.header("Authorization", format!("Bearer {}", key));
            }
        }

        let resp = request.send().await?;
        let status = resp.status();
        log::debug!("Whisper API 响应: status={}", status);

        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            log::error!("Whisper API 请求失败 ({}): {}", status, body);
            anyhow::bail!("Whisper API 请求失败 ({}): {}", status, body);
        }

        let data: serde_json::Value = resp.json().await?;

        let text = data["text"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid response format: missing 'text' field"))?;

        log::info!("Whisper 转录成功: 文本长度={}", text.len());
        Ok(text.to_string())
    }

    async fn test_connection(&self) -> Result<bool> {
        // 创建一个极小的测试 WAV 文件 (静音，0.1秒)
        let test_audio = create_test_wav();

        let client = Client::new();

        let file_part = reqwest::multipart::Part::bytes(test_audio)
            .file_name("test.wav")
            .mime_str("audio/wav")?;

        let form = reqwest::multipart::Form::new()
            .part("file", file_part)
            .text("model", self.model.clone());

        let mut request = client.post(&self.api_url).multipart(form);

        if let Some(key) = &self.api_key {
            if !key.is_empty() {
                request = request.header("Authorization", format!("Bearer {}", key));
            }
        }

        let resp = request.send().await?;

        // 只要 API 能响应就认为连接成功
        Ok(resp.status().is_success())
    }
}

/// 创建一个极小的测试 WAV 文件 (16kHz, 16bit, mono, ~0.1秒静音)
fn create_test_wav() -> Vec<u8> {
    let sample_rate: u32 = 16000;
    let num_samples: u32 = 1600; // 0.1 秒
    let data_size = num_samples * 2; // 16bit = 2 bytes per sample
    let file_size = 36 + data_size;

    let mut wav = Vec::new();

    // RIFF header
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&file_size.to_le_bytes());
    wav.extend_from_slice(b"WAVE");

    // fmt chunk
    wav.extend_from_slice(b"fmt ");
    wav.extend_from_slice(&16u32.to_le_bytes()); // chunk size
    wav.extend_from_slice(&1u16.to_le_bytes()); // PCM
    wav.extend_from_slice(&1u16.to_le_bytes()); // mono
    wav.extend_from_slice(&sample_rate.to_le_bytes());
    wav.extend_from_slice(&(sample_rate * 2).to_le_bytes()); // byte rate
    wav.extend_from_slice(&2u16.to_le_bytes()); // block align
    wav.extend_from_slice(&16u16.to_le_bytes()); // bits per sample

    // data chunk
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&data_size.to_le_bytes());
    // 静音数据
    wav.extend(vec![0u8; data_size as usize]);

    wav
}
