import { Download, FileText, Mic, Sparkles } from "lucide-react";

interface VideoCardProps {
  title: string;
  author: string;
  duration: string;
  coverUrl: string;
  onDownloadVideo: () => void;
  onDownloadAudio: () => void;
  onTranscribe: () => void;
  onSummarize: () => void;
}

function VideoCard({
  title,
  author,
  duration,
  coverUrl,
  onDownloadVideo,
  onDownloadAudio,
  onTranscribe,
  onSummarize,
}: VideoCardProps) {
  return (
    <div className="video-card">
      <div className="video-cover">
        <img
          src={coverUrl}
          alt={title}
          onLoad={() => console.log("封面加载成功:", coverUrl)}
          onError={(e) => {
            console.error("封面加载失败:", coverUrl);
            (e.target as HTMLImageElement).src = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='320' height='180' viewBox='0 0 320 180'%3E%3Crect fill='%23f0f0f0' width='320' height='180'/%3E%3Ctext fill='%23999' font-family='sans-serif' font-size='14' x='50%25' y='50%25' dominant-baseline='middle' text-anchor='middle'%3E封面加载失败%3C/text%3E%3C/svg%3E";
          }}
        />
        <span className="video-duration">{duration}</span>
      </div>
      <div className="video-info">
        <h3 className="video-title">{title}</h3>
        <p className="video-author">{author}</p>
      </div>
      <div className="video-actions">
        <button className="btn btn-secondary" onClick={onDownloadVideo} title="下载视频">
          <Download size={16} />
          <span>下载视频</span>
        </button>
        <button className="btn btn-secondary" onClick={onDownloadAudio} title="下载音频">
          <FileText size={16} />
          <span>下载音频</span>
        </button>
        <button className="btn btn-primary" onClick={onTranscribe} title="语音转录">
          <Mic size={16} />
          <span>语音转录</span>
        </button>
        <button className="btn btn-secondary" onClick={onSummarize} title="AI 摘要">
          <Sparkles size={16} />
          <span>AI 摘要</span>
        </button>
      </div>
    </div>
  );
}

export default VideoCard;
