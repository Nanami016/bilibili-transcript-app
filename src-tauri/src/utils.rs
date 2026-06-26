// 工具函数

use anyhow::{bail, Result};
use std::path::PathBuf;

/// 解析 yt-dlp 的完整路径
///
/// App bundle 启动时 PATH 不包含 /opt/homebrew/bin/，需要手动查找
pub fn resolve_ytdlp_path() -> Result<PathBuf> {
    resolve_tool_path("yt-dlp", "YT_DLP_PATH", "brew install yt-dlp")
}

/// 解析 ffmpeg 的完整路径
///
/// App bundle 启动时 PATH 不包含 /opt/homebrew/bin/，需要手动查找
pub fn resolve_ffmpeg_path() -> Result<PathBuf> {
    resolve_tool_path("ffmpeg", "FFMPEG_PATH", "brew install ffmpeg")
}

/// 通用工具路径解析
///
/// App bundle 启动时 PATH 不包含 /opt/homebrew/bin/，需要手动查找
fn resolve_tool_path(name: &str, env_var: &str, install_hint: &str) -> Result<PathBuf> {
    // 1. 优先使用环境变量中配置的路径
    if let Ok(path) = std::env::var(env_var) {
        let p = PathBuf::from(&path);
        if p.exists() {
            return Ok(p);
        }
    }

    // 2. 尝试 which 查找（开发环境有效）
    if let Ok(output) = std::process::Command::new("which").arg(name).output() {
        if output.status.success() {
            let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path_str.is_empty() {
                let p = PathBuf::from(&path_str);
                if p.exists() {
                    return Ok(p);
                }
            }
        }
    }

    // 3. 尝试常见安装路径（App bundle 启动时 PATH 不完整）
    let candidates = [
        format!("/opt/homebrew/bin/{}", name),      // macOS Apple Silicon (Homebrew)
        format!("/usr/local/bin/{}", name),          // macOS Intel (Homebrew) / Linux
        format!("/usr/bin/{}", name),                // 系统包管理器
        format!("/home/linuxbrew/.linuxbrew/bin/{}", name), // Linux Homebrew
    ];

    for candidate in &candidates {
        let p = PathBuf::from(candidate);
        if p.exists() {
            return Ok(p);
        }
    }

    // 4. 未找到，给出明确的安装提示
    bail!("未找到 {}，请先安装: {}", name, install_hint)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_ytdlp_path() {
        let path = resolve_ytdlp_path().expect("yt-dlp should be installed");
        assert!(path.exists(), "yt-dlp not found at {:?}", path);
    }

    #[test]
    fn test_resolve_ffmpeg_path() {
        let path = resolve_ffmpeg_path().expect("ffmpeg should be installed");
        assert!(path.exists(), "ffmpeg not found at {:?}", path);
    }
}
