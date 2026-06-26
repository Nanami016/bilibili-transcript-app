// 工具函数

use anyhow::{bail, Result};
use std::path::PathBuf;

/// 解析 yt-dlp 的完整路径
///
/// App bundle 启动时 PATH 不包含 /opt/homebrew/bin/，需要手动查找
pub fn resolve_ytdlp_path() -> Result<PathBuf> {
    // 1. 优先使用环境变量中配置的路径
    if let Ok(path) = std::env::var("YT_DLP_PATH") {
        let p = PathBuf::from(&path);
        if p.exists() {
            return Ok(p);
        }
    }

    // 2. 尝试 which 查找（开发环境有效）
    if let Ok(output) = std::process::Command::new("which").arg("yt-dlp").output() {
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
        "/opt/homebrew/bin/yt-dlp",      // macOS Apple Silicon (Homebrew)
        "/usr/local/bin/yt-dlp",          // macOS Intel (Homebrew) / Linux
        "/usr/bin/yt-dlp",                // 系统包管理器
        "/home/linuxbrew/.linuxbrew/bin/yt-dlp", // Linux Homebrew
    ];

    for candidate in &candidates {
        let p = PathBuf::from(candidate);
        if p.exists() {
            return Ok(p);
        }
    }

    // 4. 未找到，给出明确的安装提示
    bail!("未找到 yt-dlp，请先安装: brew install yt-dlp")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_ytdlp_path() {
        let path = resolve_ytdlp_path().expect("yt-dlp should be installed");
        assert!(path.exists(), "yt-dlp not found at {:?}", path);
    }
}
