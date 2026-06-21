import { useState, useEffect } from "react";
import InputBar from "../components/InputBar";
import VideoCard from "../components/VideoCard";
import { parseVideo, downloadVideo, downloadAudio, transcribe, fetchCover } from "../lib/tauri";

interface VideoInfo {
  bvid: string;
  aid: number;
  cid: number;
  title: string;
  author: string;
  duration: number;
  description: string;
  cover_url: string;
  upload_date: string;
}

interface TranscriptResult {
  text: string;
  source: string;
  language: string | null;
}

function Home() {
  const [videoInfo, setVideoInfo] = useState<VideoInfo | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [transcriptResult, setTranscriptResult] = useState<TranscriptResult | null>(null);
  const [actionLoading, setActionLoading] = useState<string | null>(null);
  const [toast, setToast] = useState<{ message: string; type: "success" | "error" | "info" } | null>(null);

  // Toast 自动消失
  useEffect(() => {
    if (toast) {
      const timer = setTimeout(() => setToast(null), 4000);
      return () => clearTimeout(timer);
    }
  }, [toast]);

  const handleSearch = async (url: string) => {
    setLoading(true);
    setError(null);
    setVideoInfo(null);
    setTranscriptResult(null);

    try {
      const info = await parseVideo(url);
      const vi = info as VideoInfo;
      console.log("Video info:", vi);
      console.log("Cover URL:", vi.cover_url);

      // 通过 Rust 代理获取封面（避免 WebView 外部图片限制）
      try {
        const coverData = await fetchCover(vi.cover_url);
        vi.cover_url = coverData as string;
        console.log("封面获取成功");
      } catch (e) {
        console.error("封面获取失败:", e);
      }

      setVideoInfo(vi);
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  };

  const handleDownloadVideo = async () => {
    if (!videoInfo) return;
    setActionLoading("downloadVideo");
    try {
      const url = `https://www.bilibili.com/video/${videoInfo.bvid}`;
      const result = await downloadVideo(url, "best");
      setToast({ message: `视频已下载: ${result}`, type: "success" });
    } catch (err) {
      setToast({ message: `下载失败: ${err}`, type: "error" });
    } finally {
      setActionLoading(null);
    }
  };

  const handleDownloadAudio = async () => {
    if (!videoInfo) return;
    setActionLoading("downloadAudio");
    try {
      const url = `https://www.bilibili.com/video/${videoInfo.bvid}`;
      const result = await downloadAudio(url);
      setToast({ message: `音频已下载: ${result}`, type: "success" });
    } catch (err) {
      setToast({ message: `下载失败: ${err}`, type: "error" });
    } finally {
      setActionLoading(null);
    }
  };

  const handleTranscribe = async () => {
    if (!videoInfo) return;
    setActionLoading("transcribe");
    setTranscriptResult(null);
    try {
      const url = `https://www.bilibili.com/video/${videoInfo.bvid}`;
      const result = (await transcribe(url)) as TranscriptResult;
      setTranscriptResult(result);
      setToast({
        message: `转录完成 (${result.source === "cc" ? "CC字幕" : result.source === "ai" ? "AI字幕" : "Whisper"})`,
        type: "success",
      });
    } catch (err) {
      setToast({ message: `转录失败: ${err}`, type: "error" });
    } finally {
      setActionLoading(null);
    }
  };

  const handleSummarize = () => {
    setToast({ message: "AI 摘要功能需要先在设置中配置", type: "info" });
  };

  const formatDuration = (seconds: number) => {
    const m = Math.floor(seconds / 60);
    const s = seconds % 60;
    return `${m}:${s.toString().padStart(2, "0")}`;
  };

  return (
    <div className="page home-page">
      <InputBar onSearch={handleSearch} />

      {toast && (
        <div className={`toast toast-${toast.type}`}>
          <span>{toast.message}</span>
          <button className="toast-close" onClick={() => setToast(null)}>×</button>
        </div>
      )}

      <div className="content-area">
        {loading && <div className="loading">解析中...</div>}

        {error && (
          <div className="empty-state" style={{ color: "#ff4d4f" }}>
            <p>{error}</p>
          </div>
        )}

        {videoInfo && (
          <>
            <VideoCard
              title={videoInfo.title}
              author={videoInfo.author}
              duration={formatDuration(videoInfo.duration)}
              coverUrl={videoInfo.cover_url}
              onDownloadVideo={handleDownloadVideo}
              onDownloadAudio={handleDownloadAudio}
              onTranscribe={handleTranscribe}
              onSummarize={handleSummarize}
            />

            {actionLoading && (
              <div className="loading">
                {actionLoading === "transcribe"
                  ? "正在转录，请稍候..."
                  : actionLoading === "downloadVideo"
                  ? "正在下载视频..."
                  : "正在下载音频..."}
              </div>
            )}

            {transcriptResult && (
              <div className="settings-section" style={{ marginTop: 16 }}>
                <h3>
                  📝 转录结果
                  <span style={{ fontSize: 12, color: "#666", marginLeft: 8 }}>
                    来源: {transcriptResult.source === "cc" ? "CC字幕" : transcriptResult.source === "ai" ? "AI字幕" : "Whisper"}
                  </span>
                </h3>
                <div
                  style={{
                    whiteSpace: "pre-wrap",
                    lineHeight: 1.8,
                    maxHeight: 400,
                    overflow: "auto",
                    padding: 16,
                    background: "#f9f9f9",
                    borderRadius: 8,
                    marginTop: 12,
                  }}
                >
                  {transcriptResult.text}
                </div>
              </div>
            )}
          </>
        )}

        {!videoInfo && !loading && !error && (
          <div className="empty-state">
            <p>输入 B站视频链接开始使用</p>
          </div>
        )}
      </div>
    </div>
  );
}

export default Home;
