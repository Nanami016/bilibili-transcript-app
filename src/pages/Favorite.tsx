import { useState, useEffect } from "react";
import { getFavorites, getFavoriteVideos, transcribe } from "../lib/tauri";

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
  const [toast, setToast] = useState<{ message: string; type: "success" | "error" | "info" } | null>(null);

  // Toast 自动消失
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
    try {
      const result = await getFavoriteVideos(String(folder.id));
      setVideos(result as VideoInfo[]);
    } catch (err) {
      setToast({ message: `获取视频列表失败: ${err}`, type: "error" });
    } finally {
      setVideosLoading(false);
    }
  };

  const handleBack = () => {
    setSelectedFolder(null);
    setVideos([]);
  };

  const handleTranscribe = async (video: VideoInfo) => {
    try {
      const url = `https://www.bilibili.com/video/${video.bvid}`;
      await transcribe(url);
      setToast({ message: `「${video.title}」转录完成`, type: "success" });
    } catch (err) {
      setToast({ message: `转录失败: ${err}`, type: "error" });
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

      {selectedFolder ? (
        <>
          <div style={{ display: "flex", alignItems: "center", gap: 12, marginBottom: 24 }}>
            <button className="btn btn-secondary" onClick={handleBack}>← 返回</button>
            <h2>{selectedFolder.title}</h2>
            <span style={{ color: "#666" }}>{selectedFolder.media_count} 个视频</span>
          </div>

          {videosLoading ? (
            <div className="loading">加载视频列表...</div>
          ) : videos.length === 0 ? (
            <div className="empty-state">
              <p>收藏夹中没有视频</p>
            </div>
          ) : (
            <div className="favorites-list">
              {videos.map((video) => (
                <div key={video.bvid} className="favorite-item" style={{ cursor: "default" }}>
                  <div style={{ display: "flex", gap: 12 }}>
                    <img
                      src={video.cover_url}
                      alt={video.title}
                      style={{ width: 120, height: 68, objectFit: "cover", borderRadius: 4 }}
                    />
                    <div style={{ flex: 1 }}>
                      <h4 style={{ fontSize: 14, marginBottom: 4 }}>{video.title}</h4>
                      <p style={{ fontSize: 12, color: "#666" }}>
                        {video.author} · {formatDuration(video.duration)}
                      </p>
                    </div>
                  </div>
                  <div style={{ marginTop: 8, display: "flex", gap: 8 }}>
                    <button
                      className="btn btn-primary"
                      style={{ fontSize: 12, padding: "6px 12px" }}
                      onClick={() => handleTranscribe(video)}
                    >
                      语音转录
                    </button>
                  </div>
                </div>
              ))}
            </div>
          )}
        </>
      ) : (
        <>
          <h2>我的收藏夹</h2>

          {loading ? (
            <div className="loading">加载中...</div>
          ) : error ? (
            <div className="empty-state" style={{ color: "#ff4d4f" }}>
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
                  <h3>{folder.title}</h3>
                  <span>{folder.media_count} 个视频</span>
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
