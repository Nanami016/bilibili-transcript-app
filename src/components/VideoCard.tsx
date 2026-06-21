import { useState, useRef, useEffect } from "react";
import { Download, FileText, Mic, Sparkles, ChevronDown } from "lucide-react";

interface VideoFormat {
  format_id: string;
  quality: string;
  description: string;
  filesize: number | null;
}

interface VideoCardProps {
  title: string;
  author: string;
  duration: string;
  coverUrl: string;
  formats: VideoFormat[];
  onDownloadVideo: (formatId: string) => void;
  onDownloadAudio: () => void;
  onTranscribe: () => void;
  onSummarize: () => void;
}

function VideoCard({
  title,
  author,
  duration,
  coverUrl,
  formats,
  onDownloadVideo,
  onDownloadAudio,
  onTranscribe,
  onSummarize,
}: VideoCardProps) {
  const [showFormats, setShowFormats] = useState(false);
  const [selectedFormat, setSelectedFormat] = useState<string>("best");
  const dropdownRef = useRef<HTMLDivElement>(null);

  // 点击外部关闭下拉框
  useEffect(() => {
    const handleClickOutside = (e: MouseEvent) => {
      if (dropdownRef.current && !dropdownRef.current.contains(e.target as Node)) {
        setShowFormats(false);
      }
    };
    document.addEventListener("mousedown", handleClickOutside);
    return () => document.removeEventListener("mousedown", handleClickOutside);
  }, []);

  // 格式列表更新时自动选中第一个
  useEffect(() => {
    if (formats.length > 0 && !formats.find((f) => f.format_id === selectedFormat)) {
      setSelectedFormat(formats[0].format_id);
    }
  }, [formats, selectedFormat]);

  const currentFormat = formats.find((f) => f.format_id === selectedFormat);
  const buttonLabel = currentFormat ? currentFormat.quality : "下载视频";

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
        <div className="video-download-wrapper" ref={dropdownRef}>
          <button
            className="btn btn-secondary"
            onClick={() => {
              if (formats.length === 0) {
                // 没有格式列表时直接下载
                onDownloadVideo("best");
              } else {
                setShowFormats(!showFormats);
              }
            }}
            title="下载视频"
          >
            <Download size={16} />
            <span>{buttonLabel}</span>
            {formats.length > 0 && <ChevronDown size={14} />}
          </button>
          {showFormats && formats.length > 0 && (
            <div className="format-dropdown">
              {formats.map((f) => (
                <div
                  key={f.format_id}
                  className={`format-option ${selectedFormat === f.format_id ? "selected" : ""}`}
                  onClick={() => {
                    setSelectedFormat(f.format_id);
                    setShowFormats(false);
                    onDownloadVideo(f.format_id);
                  }}
                >
                  <span className="format-quality">{f.quality}</span>
                  <span className="format-desc">{f.description}</span>
                </div>
              ))}
              {!formats.some((f) => {
                const h = parseInt(f.quality);
                return h >= 720;
              }) && (
                <div className="format-hint">
                  💡 720P 及以上需要 B站大会员
                </div>
              )}
            </div>
          )}
        </div>
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
