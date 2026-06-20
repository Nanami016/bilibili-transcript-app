import { useState } from "react";
import InputBar from "../components/InputBar";
import VideoCard from "../components/VideoCard";

function Home() {
  const [videoInfo, setVideoInfo] = useState<any>(null);
  const [loading, setLoading] = useState(false);

  const handleSearch = async (url: string) => {
    setLoading(true);
    try {
      // TODO: 调用 Tauri 命令解析视频
      console.log("Searching:", url);
    } catch (error) {
      console.error("Failed to parse video:", error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="page home-page">
      <InputBar onSearch={handleSearch} />

      <div className="content-area">
        {loading && <div className="loading">加载中...</div>}

        {videoInfo && (
          <VideoCard
            title={videoInfo.title}
            author={videoInfo.author}
            duration={videoInfo.duration}
            coverUrl={videoInfo.coverUrl}
            onDownloadVideo={() => {}}
            onDownloadAudio={() => {}}
            onTranscribe={() => {}}
            onSummarize={() => {}}
          />
        )}

        {!videoInfo && !loading && (
          <div className="empty-state">
            <p>输入 B站视频链接开始使用</p>
          </div>
        )}
      </div>
    </div>
  );
}

export default Home;
