import { useState, useEffect } from "react";
import TranscribeModal from "../components/TranscribeModal";
import type { TranscribeParams } from "../components/TranscribeModal";
import {
  getFavorites,
  getFavoriteVideos,
  fetchCover,
  startVideoDownload,
  startAudioDownload,
  startTranscribe,
} from "../lib/tauri";
import { ArrowLeft } from "lucide-react";

interface FavoriteFolder {
  id: number;
  title: string;
  media_count: number;
}

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

function Favorite() {
  const [favorites, setFavorites] = useState<FavoriteFolder[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [selectedFolder, setSelectedFolder] = useState<FavoriteFolder | null>(null);
  const [videos, setVideos] = useState<VideoInfo[]>([]);
  const [videosLoading, setVideosLoading] = useState(false);
  const [page, setPage] = useState(1);
  const [hasMore, setHasMore] = useState(false);
  const [loadingMore, setLoadingMore] = useState(false);
  const [toast, setToast] = useState<{ message: string; type: "success" | "error" | "info" } | null>(null);
  const [showTranscribeModal, setShowTranscribeModal] = useState(false);
  const [transcribeTarget, setTranscribeTarget] = useState<VideoInfo | null>(null);

  useEffect(() => {
    if (toast) {
      const timer = setTimeout(() => setToast(null), 4000);
      return () => clearTimeout(timer);
    }
  }, [toast]);

  useEffect(() => {
    loadFavorites();
  }, []);

  const loadFavorites = async () => {
    try {
      setLoading(true);
      setError(null);
      const result = await getFavorites();
      setFavorites(result as FavoriteFolder[]);
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  };

  const handleSelectFolder = async (folder: FavoriteFolder) => {
    setSelectedFolder(folder);
    setVideosLoading(true);
    setVideos([]);
    setPage(1);
    setHasMore(false);
    try {
      const result = await getFavoriteVideos(String(folder.id), 1);
      const data = result as { videos: VideoInfo[]; has_more: boolean; page: number };

      const videosWithCovers = await Promise.all(
        data.videos.map(async (video) => {
          try {
            const coverData = await fetchCover(video.cover_url);
            return { ...video, cover_url: coverData as string };
          } catch (e) {
            console.error("封面获取失败:", video.title, e);
            return video;
          }
        })
      );

      setVideos(videosWithCovers);
      setHasMore(data.has_more);
    } catch (err) {
      setToast({ message: `获取视频列表失败: ${err}`, type: "error" });
    } finally {
      setVideosLoading(false);
    }
  };

  const handleLoadMore = async () => {
    if (!selectedFolder || loadingMore) return;
    const nextPage = page + 1;
    setLoadingMore(true);
    try {
      const result = await getFavoriteVideos(String(selectedFolder.id), nextPage);
      const data = result as { videos: VideoInfo[]; has_more: boolean; page: number };

      const videosWithCovers = await Promise.all(
        data.videos.map(async (video) => {
          try {
            const coverData = await fetchCover(video.cover_url);
            return { ...video, cover_url: coverData as string };
          } catch (e) {
            console.error("封面获取失败:", video.title, e);
            return video;
          }
        })
      );

      setVideos((prev) => [...prev, ...videosWithCovers]);
      setPage(nextPage);
      setHasMore(data.has_more);
    } catch (err) {
      setToast({ message: `加载更多失败: ${err}`, type: "error" });
    } finally {
      setLoadingMore(false);
    }
  };

  const handleBack = () => {
    setSelectedFolder(null);
    setVideos([]);
    setPage(1);
    setHasMore(false);
  };

  const handleTranscribe = (video: VideoInfo) => {
    setTranscribeTarget(video);
    setShowTranscribeModal(true);
  };

  const handleTranscribeConfirm = async (params: TranscribeParams) => {
    if (!transcribeTarget) return;
    setShowTranscribeModal(false);
    try {
      const url = `https://www.bilibili.com/video/${transcribeTarget.bvid}`;
      await startTranscribe(url, params.language, params.whisperPrompt, params.aiPrompt, params.aiContext, params.skipBilibiliSubtitle);
      setToast({ message: `「${transcribeTarget.title}」转录任务已启动`, type: "info" });
    } catch (err) {
      setToast({ message: `转录失败: ${err}`, type: "error" });
    } finally {
      setTranscribeTarget(null);
    }
  };

  const handleDownloadVideo = async (video: VideoInfo) => {
    try {
      const url = `https://www.bilibili.com/video/${video.bvid}`;
      await startVideoDownload(url, "best");
      setToast({ message: `「${video.title}」视频下载任务已启动`, type: "info" });
    } catch (err) {
      setToast({ message: `下载失败: ${err}`, type: "error" });
    }
  };

  const handleDownloadAudio = async (video: VideoInfo) => {
    try {
      const url = `https://www.bilibili.com/video/${video.bvid}`;
      await startAudioDownload(url);
      setToast({ message: `「${video.title}」音频下载任务已启动`, type: "info" });
    } catch (err) {
      setToast({ message: `下载失败: ${err}`, type: "error" });
    }
  };

  const formatDuration = (seconds: number) => {
    const m = Math.floor(seconds / 60);
    const s = seconds % 60;
    return `${m}:${s.toString().padStart(2, "0")}`;
  };

  return (
    <div className="page favorite-page">
      {toast && (
        <div className={`toast toast-${toast.type}`}>
          <span>{toast.message}</span>
          <button className="toast-close" onClick={() => setToast(null)}>×</button>
        </div>
      )}

      <TranscribeModal
        visible={showTranscribeModal}
        videoTitle={transcribeTarget?.title || ""}
        onConfirm={handleTranscribeConfirm}
        onCancel={() => { setShowTranscribeModal(false); setTranscribeTarget(null); }}
      />

      {selectedFolder ? (
        <>
          <div className="task-page-header">
            <div style={{ display: "flex", alignItems: "center", gap: 12 }}>
              <button className="btn btn-secondary btn-sm" onClick={handleBack}>
                <ArrowLeft size={16} />
                返回
              </button>
              <h2>{selectedFolder.title}</h2>
              <span style={{ color: "var(--text-secondary)", fontSize: 14 }}>{selectedFolder.media_count} 个视频</span>
            </div>
          </div>

          {videosLoading ? (
            <div className="loading">
              <div className="spinner-circle" />
              <span>加载视频列表...</span>
            </div>
          ) : videos.length === 0 ? (
            <div className="empty-state">
              <p>收藏夹中没有视频</p>
            </div>
          ) : (
            <div className="favorites-list">
              {videos.map((video) => (
                <div key={video.bvid} className="favorite-item" style={{ cursor: "default" }}>
                  <div className="favorite-video-row">
                    <img
                      src={video.cover_url}
                      alt={video.title}
                      className="favorite-video-cover"
                      onError={(e) => {
                        (e.target as HTMLImageElement).src =
                          "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='120' height='68' viewBox='0 0 120 68'%3E%3Crect fill='%23f0f0f0' width='120' height='68'/%3E%3Ctext fill='%23999' font-family='sans-serif' font-size='10' x='50%25' y='50%25' dominant-baseline='middle' text-anchor='middle'%3E封面%3C/text%3E%3C/svg%3E";
                      }}
                    />
                    <div style={{ flex: 1, minWidth: 0 }}>
                      <h4 className="favorite-video-title">{video.title}</h4>
                      <p className="favorite-video-meta">
                        {video.author} · {formatDuration(video.duration)}
                      </p>
                    </div>
                  </div>
                  <div className="favorite-actions">
                    <button
                      className="btn btn-secondary btn-sm"
                      onClick={() => handleDownloadVideo(video)}
                      title="下载视频"
                    >
                      下载视频
                    </button>
                    <button
                      className="btn btn-secondary btn-sm"
                      onClick={() => handleDownloadAudio(video)}
                      title="下载音频"
                    >
                      下载音频
                    </button>
                    <button
                      className="btn btn-primary btn-sm"
                      onClick={() => handleTranscribe(video)}
                      title="语音转录"
                    >
                      语音转录
                    </button>
                  </div>
                </div>
              ))}
              {hasMore && (
                <div className="favorites-load-more">
                  <button
                    className="btn btn-secondary"
                    onClick={handleLoadMore}
                    disabled={loadingMore}
                    style={{ minWidth: 120 }}
                  >
                    {loadingMore ? (
                      <><div className="spinner-circle" style={{ width: 16, height: 16, borderWidth: 2 }} /> 加载中...</>
                    ) : "加载更多"}
                  </button>
                </div>
              )}
              {!hasMore && videos.length > 0 && (
                <div className="favorites-footer">
                  已加载全部 {videos.length} 个视频
                </div>
              )}
            </div>
          )}
        </>
      ) : (
        <>
          <div className="task-page-header">
            <h2>我的收藏夹</h2>
          </div>

          {loading ? (
            <div className="loading">
              <div className="spinner-circle" />
              <span>加载中...</span>
            </div>
          ) : error ? (
            <div className="empty-state" style={{ color: "var(--status-error)" }}>
              <p>{error}</p>
              <button className="btn btn-primary" style={{ marginTop: 16 }} onClick={loadFavorites}>
                重试
              </button>
            </div>
          ) : favorites.length === 0 ? (
            <div className="empty-state">
              <p>请先在设置中导入 B站 Cookie</p>
            </div>
          ) : (
            <div className="favorites-list">
              {favorites.map((folder) => (
                <div
                  key={folder.id}
                  className="favorite-item"
                  onClick={() => handleSelectFolder(folder)}
                >
                  <h3 style={{ fontSize: 16, marginBottom: 4 }}>{folder.title}</h3>
                  <span style={{ color: "var(--text-secondary)", fontSize: 13 }}>{folder.media_count} 个视频</span>
                </div>
              ))}
            </div>
          )}
        </>
      )}
    </div>
  );
}

export default Favorite;
