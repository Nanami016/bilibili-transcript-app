import { useState, useEffect } from "react";

interface TranscribeModalProps {
  visible: boolean;
  videoTitle: string;
  onConfirm: (params: TranscribeParams) => void;
  onCancel: () => void;
}

export interface TranscribeParams {
  language: string;
  whisperPrompt: string;
  aiPrompt: string;
  aiContext: string;
  skipBilibiliSubtitle: boolean;
}

const LANGUAGES = [
  { code: "zh", label: "中文" },
  { code: "ja", label: "日文" },
  { code: "en", label: "英文" },
  { code: "ko", label: "韩文" },
  { code: "fr", label: "法文" },
  { code: "de", label: "德文" },
  { code: "ru", label: "俄文" },
  { code: "es", label: "西班牙文" },
];

function TranscribeModal({ visible, videoTitle, onConfirm, onCancel }: TranscribeModalProps) {
  const [language, setLanguage] = useState("zh");
  const [whisperPrompt, setWhisperPrompt] = useState("");
  const [aiPrompt, setAiPrompt] = useState("");
  const [aiContext, setAiContext] = useState("");
  const [skipBilibiliSubtitle, setSkipBilibiliSubtitle] = useState(false);
  const [showAdvanced, setShowAdvanced] = useState(false);
  const [showContent, setShowContent] = useState(false);

  // 动画结束后再渲染文字内容，避免 scale 动画导致的亚像素模糊
  useEffect(() => {
    if (visible) {
      const timer = setTimeout(() => setShowContent(true), 200);
      return () => clearTimeout(timer);
    } else {
      setShowContent(false);
    }
  }, [visible]);

  if (!visible) return null;

  const handleConfirm = () => {
    onConfirm({ language, whisperPrompt, aiPrompt, aiContext, skipBilibiliSubtitle });
  };

  return (
    <div className="modal-overlay" onClick={onCancel}>
      <div className="modal-content" onClick={(e) => e.stopPropagation()}>
        {showContent ? (
          <>
            <div className="modal-header">
              <h3>📝 语音转录</h3>
              <button className="modal-close" onClick={onCancel}>×</button>
            </div>

            <div className="modal-body">
              <p className="modal-video-title" title={videoTitle}>
                {videoTitle.length > 40 ? videoTitle.slice(0, 40) + "..." : videoTitle}
              </p>

              <div className="form-group">
                <label>视频主要语言</label>
                <div style={{ display: "flex", flexWrap: "wrap", gap: 6, marginTop: 4 }}>
                  {LANGUAGES.map((lang) => (
                    <button
                      key={lang.code}
                      type="button"
                      onClick={() => setLanguage(lang.code)}
                      className={`btn ${language === lang.code ? "btn-primary" : "btn-secondary"} btn-sm`}
                    >
                      {lang.label}
                    </button>
                  ))}
                </div>
              </div>

              <div className="form-group">
                <label style={{ display: "flex", alignItems: "center", gap: 8, cursor: "pointer" }}>
                  <input
                    type="checkbox"
                    checked={skipBilibiliSubtitle}
                    onChange={(e) => setSkipBilibiliSubtitle(e.target.checked)}
                    style={{ width: 16, height: 16, cursor: "pointer" }}
                  />
                  跳过 B站字幕，直接使用本地模型转录
                </label>
                <p className="form-hint" style={{ marginLeft: 24 }}>忽略 B站 AI/CC 字幕，强制使用 Whisper 转录</p>
              </div>

              <div className="form-group">
                <label>转录提示词（可选）</label>
                <input
                  type="text"
                  value={whisperPrompt}
                  onChange={(e) => setWhisperPrompt(e.target.value)}
                  placeholder="如：AI、GPT、B站、UP主 等专有名词"
                />
                <p className="form-hint">提高特定词汇的识别准确度</p>
              </div>

              <button
                className="btn-link"
                onClick={() => setShowAdvanced(!showAdvanced)}
                style={{
                  background: "none",
                  border: "none",
                  color: "var(--accent)",
                  cursor: "pointer",
                  fontSize: 13,
                  padding: "4px 0",
                  marginBottom: showAdvanced ? 8 : 0,
                  display: "flex",
                  alignItems: "center",
                  gap: 4,
                }}
              >
                {showAdvanced ? "▼ 收起高级选项" : "▶ 高级选项（AI 摘要）"}
              </button>

              {showAdvanced && (
                <>
                  <div className="form-group">
                    <label>AI 摘要指令（可选）</label>
                    <textarea
                      value={aiPrompt}
                      onChange={(e) => setAiPrompt(e.target.value)}
                      placeholder="如：重点关注技术细节，输出要点列表格式"
                      rows={2}
                      style={{ fontSize: 13 }}
                    />
                    <p className="form-hint">追加到默认摘要指令之后</p>
                  </div>
                  <div className="form-group">
                    <label>上下文文本（可选）</label>
                    <textarea
                      value={aiContext}
                      onChange={(e) => setAiContext(e.target.value)}
                      placeholder="如：这是一个科技评测频道，主要讨论数码产品"
                      rows={2}
                      style={{ fontSize: 13 }}
                    />
                    <p className="form-hint">为 AI 提供额外背景信息</p>
                  </div>
                </>
              )}
            </div>

            <div className="modal-footer">
              <button className="btn btn-secondary" onClick={onCancel}>取消</button>
              <button className="btn btn-primary" onClick={handleConfirm}>开始转录</button>
            </div>
          </>
        ) : (
          <div className="modal-placeholder" />
        )}
      </div>
    </div>
  );
}

export default TranscribeModal;
