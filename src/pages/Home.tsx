import { useState, useEffect } from "react";
import InputBar from "../components/InputBar";
import VideoCard from "../components/VideoCard";
import {
  parseVideo,
  fetchCover,
  getVideoFormats,
  startVideoDownload,
  startAudioDownload,
  startTranscribe,
  startAiSummary,
} from "../lib/tauri";

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

interface VideoFormat {
  format_id: string;
  quality: string;
  description: string;
  filesize: number | null;
}

function Home() {
  const [videoInfo, setVideoInfo] = useState<VideoInfo | null>(null);
  const [formats, setFormats] = useState<VideoFormat[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
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
    setFormats([]);

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

      // 获取可用格式列表
      try {
        const videoUrl = `https://www.bilibili.com/video/${vi.bvid}`;
        const fmts = await getVideoFormats(videoUrl);
        setFormats(fmts as VideoFormat[]);
      } catch (e) {
        console.error("获取格式列表失败:", e);
      }
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  };

  const handleDownloadVideo = async (formatId: string = "best") => {
    if (!videoInfo) return;
    setActionLoading("downloadVideo");
    try {
      const url = `https://www.bilibili.com/video/${videoInfo.bvid}`;
      await startVideoDownload(url, formatId);
      setToast({ message: "视频下载任务已启动", type: "info" });
    } catch (err) {
      setToast({ message: `启动失败: ${err}`, type: "error" });
    } finally {
      setActionLoading(null);
    }
  };

  const handleDownloadAudio = async () => {
    if (!videoInfo) return;
    setActionLoading("downloadAudio");
    try {
      const url = `https://www.bilibili.com/video/${videoInfo.bvid}`;
      await startAudioDownload(url);
      setToast({ message: "音频下载任务已启动", type: "info" });
    } catch (err) {
      setToast({ message: `启动失败: ${err}`, type: "error" });
    } finally {
      setActionLoading(null);
    }
  };

  const handleTranscribe = async () => {
    if (!videoInfo) return;
    setActionLoading("transcribe");
    try {
      const url = `https://www.bilibili.com/video/${videoInfo.bvid}`;
      await startTranscribe(url);
      setToast({ message: "转录任务已启动", type: "info" });
    } catch (err) {
      setToast({ message: `启动失败: ${err}`, type: "error" });
    } finally {
      setActionLoading(null);
    }
  };

  const handleSummarize = async () => {
    if (!videoInfo) return;
    setActionLoading("aiSummary");
    try {
      await startAiSummary(videoInfo.bvid);
      setToast({ message: "AI 摘要任务已启动", type: "info" });
    } catch (err) {
      setToast({ message: `启动失败: ${err}`, type: "error" });
    } finally {
      setActionLoading(null);
    }
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
              formats={formats}
              onDownloadVideo={handleDownloadVideo}
              onDownloadAudio={handleDownloadAudio}
              onTranscribe={handleTranscribe}
              onSummarize={handleSummarize}
            />

            {actionLoading && (
              <div className="loading">
                {actionLoading === "transcribe"
                  ? "正在启动转录任务..."
                  : actionLoading === "downloadVideo"
                  ? "正在启动视频下载..."
                  : actionLoading === "aiSummary"
                  ? "正在启动 AI 分析..."
                  : "正在启动音频下载..."}
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
