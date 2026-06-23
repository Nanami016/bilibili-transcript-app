import { useState, useEffect } from "react";
import InputBar from "../components/InputBar";
import VideoCard from "../components/VideoCard";
import TranscribeModal from "../components/TranscribeModal";
import type { TranscribeParams } from "../components/TranscribeModal";
import {
  parseVideo,
  fetchCover,
  getVideoFormats,
  startVideoDownload,
  startAudioDownload,
  startTranscribe,
} from "../lib/tauri";
import { Film } from "lucide-react";

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
  const [showTranscribeModal, setShowTranscribeModal] = useState(false);

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

      try {
        const coverData = await fetchCover(vi.cover_url);
        vi.cover_url = coverData as string;
      } catch (e) {
        console.error("封面获取失败:", e);
      }

      setVideoInfo(vi);

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

  const handleTranscribe = () => {
    if (!videoInfo) return;
    setShowTranscribeModal(true);
  };

  const handleTranscribeConfirm = async (params: TranscribeParams) => {
    if (!videoInfo) return;
    setShowTranscribeModal(false);
    setActionLoading("transcribe");
    try {
      const url = `https://www.bilibili.com/video/${videoInfo.bvid}`;
      await startTranscribe(url, params.language, params.whisperPrompt, params.aiPrompt, params.aiContext);
      setToast({ message: "转录任务已启动", type: "info" });
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

      <TranscribeModal
        visible={showTranscribeModal}
        videoTitle={videoInfo?.title || ""}
        onConfirm={handleTranscribeConfirm}
        onCancel={() => setShowTranscribeModal(false)}
      />

      <div className="content-area">
        {loading && (
          <div className="loading">
            <div className="spinner-circle" />
            <span>解析中...</span>
          </div>
        )}

        {error && (
          <div className="empty-state" style={{ color: "var(--status-error)" }}>
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
            />

            {actionLoading && (
              <div className="loading" style={{ height: "auto", marginTop: 16 }}>
                <div className="spinner-circle" />
                <span>
                  {actionLoading === "transcribe"
                    ? "正在启动转录任务..."
                    : actionLoading === "downloadVideo"
                    ? "正在启动视频下载..."
                    : "正在启动音频下载..."}
                </span>
              </div>
            )}
          </>
        )}

        {!videoInfo && !loading && !error && (
          <div className="empty-state">
            <Film size={48} className="empty-state-icon" />
            <p>输入 B站视频链接开始使用</p>
          </div>
        )}
      </div>
    </div>
  );
}

export default Home;
