import { useState, useEffect } from "react";
import { getConfig, updateConfig, importCookie, importCookieFromBrowser, getCookieStatus, testWhisperConnection } from "../lib/tauri";

function Settings() {
  const [config, setConfig] = useState<any>(null);
  const [loading, setLoading] = useState(true);
  const [cookieStatus, setCookieStatus] = useState(false);
  const [saving, setSaving] = useState(false);
  const [toast, setToast] = useState<{ message: string; type: "success" | "error" | "info" } | null>(null);
  const [manualCookie, setManualCookie] = useState("");
  const [browserLoading, setBrowserLoading] = useState(false);

  useEffect(() => { loadConfig(); }, []);

  useEffect(() => {
    if (toast) {
      const timer = setTimeout(() => setToast(null), 4000);
      return () => clearTimeout(timer);
    }
  }, [toast]);

  const loadConfig = async () => {
    try {
      const cfg = await getConfig();
      setConfig(cfg);
      const status = await getCookieStatus();
      setCookieStatus(status as boolean);
    } catch (error) {
      console.error("Failed to load config:", error);
    } finally {
      setLoading(false);
    }
  };

  const handleSave = async () => {
    if (!config) return;
    setSaving(true);
    try {
      await updateConfig(config);
      setToast({ message: "设置已保存", type: "success" });
    } catch (error) {
      setToast({ message: "保存失败: " + String(error), type: "error" });
    } finally {
      setSaving(false);
    }
  };

  const handleImportFromBrowser = async (browser: string) => {
    setBrowserLoading(true);
    try {
      const result = await importCookieFromBrowser(browser);
      setCookieStatus(true);
      setToast({ message: String(result), type: "success" });
      await loadConfig();
    } catch (error) {
      setToast({ message: String(error), type: "error" });
    } finally {
      setBrowserLoading(false);
    }
  };

  const handleManualImport = async () => {
    if (!manualCookie.trim()) return;
    try {
      await importCookie(manualCookie.trim());
      setCookieStatus(true);
      setToast({ message: "Cookie 导入成功", type: "success" });
      setManualCookie("");
    } catch (error) {
      setToast({ message: "导入失败: " + String(error), type: "error" });
    }
  };

  const handleTestWhisper = async () => {
    try {
      const result = await testWhisperConnection();
      setToast({ message: result ? "连接成功 ✅" : "连接失败", type: result ? "success" : "error" });
    } catch (error) {
      setToast({ message: "测试失败: " + String(error), type: "error" });
    }
  };

  const updateField = (section: string, field: string, value: any) => {
    setConfig((prev: any) => ({
      ...prev,
      [section]: { ...prev[section], [field]: value },
    }));
  };

  if (loading) {
    return <div className="page"><div className="loading">加载中...</div></div>;
  }

  return (
    <div className="page settings-page">
      <h2>设置</h2>

      {toast && (
        <div className={`toast toast-${toast.type}`}>
          <span>{toast.message}</span>
          <button className="toast-close" onClick={() => setToast(null)}>×</button>
        </div>
      )}

      {/* ============ B站配置 ============ */}
      <div className="settings-section">
        <h3>
          📺 B站账号
          {cookieStatus
            ? <span style={{ fontSize: 12, color: "#52c41a", marginLeft: 8 }}>✅ 已配置</span>
            : <span style={{ fontSize: 12, color: "#ff4d4f", marginLeft: 8 }}>⚠️ 未配置</span>
          }
        </h3>

        {/* 方式一：从浏览器读取 */}
        <div style={{ marginBottom: 24 }}>
          <h4 style={{ fontSize: 14, marginBottom: 12, color: "#333" }}>
            方式一：从浏览器读取（推荐）
          </h4>
          <p style={{ fontSize: 13, color: "#666", marginBottom: 12 }}>
            确保你已在浏览器中登录 B站，然后点击对应按钮读取 Cookie（包含 SESSDATA）
          </p>
          {browserLoading ? (
            <div className="loading" style={{ height: "auto", padding: 16 }}>正在读取浏览器 Cookie...</div>
          ) : (
            <div style={{ display: "flex", gap: 8, flexWrap: "wrap" }}>
              <button className="btn btn-primary" onClick={() => handleImportFromBrowser("chrome")}>
                🌐 从 Chrome 读取
              </button>
              <button className="btn btn-secondary" onClick={() => handleImportFromBrowser("safari")}>
                🧭 从 Safari 读取
              </button>
              <button className="btn btn-secondary" onClick={() => handleImportFromBrowser("firefox")}>
                🦊 从 Firefox 读取
              </button>
              <button className="btn btn-secondary" onClick={() => handleImportFromBrowser("edge")}>
                📐 从 Edge 读取
              </button>
            </div>
          )}
          <p className="form-hint" style={{ marginTop: 8 }}>
            读取前请关闭对应浏览器，否则可能无法读取 Cookie
          </p>
        </div>

        {/* 分隔线 */}
        <div style={{ borderTop: "1px solid #e0e0e0", margin: "20px 0" }}></div>

        {/* 方式二：手动导入 */}
        <div>
          <h4 style={{ fontSize: 14, marginBottom: 12, color: "#333" }}>
            方式二：手动导入 Cookie
          </h4>
          <div className="tutorial-steps" style={{ marginBottom: 16 }}>
            <div className="step">
              <span className="step-num">1</span>
              <div className="step-content">
                <strong>在浏览器中登录 B站</strong>
                <p>访问 <code>https://www.bilibili.com</code> 并登录</p>
              </div>
            </div>
            <div className="step">
              <span className="step-num">2</span>
              <div className="step-content">
                <strong>F12 打开开发者工具</strong>
                <p>按 <kbd>F12</kbd> 或 <kbd>Cmd</kbd>+<kbd>Option</kbd>+<kbd>I</kbd></p>
              </div>
            </div>
            <div className="step">
              <span className="step-num">3</span>
              <div className="step-content">
                <strong>复制 Cookie</strong>
                <p>
                  <strong>方法 A：</strong>Application → Cookies → <code>bilibili.com</code> → 复制 <code>SESSDATA</code>、<code>DedeUserID</code>、<code>bili_jct</code> 的值<br />
                  <strong>方法 B：</strong>Network → 任意请求 → 右键 → Copy as cURL → 从命令中提取 Cookie 头
                </p>
              </div>
            </div>
          </div>
          <div className="form-group">
            <textarea
              value={manualCookie}
              onChange={(e) => setManualCookie(e.target.value)}
              placeholder="SESSDATA=xxx; DedeUserID=xxx; bili_jct=xxx"
              rows={3}
              style={{ fontFamily: "monospace", fontSize: 13 }}
            />
          </div>
          <button className="btn btn-secondary" onClick={handleManualImport}>导入 Cookie</button>
        </div>

        {/* 输出目录 */}
        <div style={{ marginTop: 20 }}>
          <h4 style={{ fontSize: 14, marginBottom: 12, color: "#333" }}>📁 输出目录</h4>
          <div className="form-group">
            <label>视频下载路径</label>
            <input
              type="text"
              value={config?.bilibili?.video_dir || ""}
              onChange={(e) => updateField("bilibili", "video_dir", e.target.value)}
              placeholder="~/Downloads/bilibili-download/video"
            />
          </div>
          <div className="form-group">
            <label>音频下载路径</label>
            <input
              type="text"
              value={config?.bilibili?.audio_dir || ""}
              onChange={(e) => updateField("bilibili", "audio_dir", e.target.value)}
              placeholder="~/Downloads/bilibili-download/audio"
            />
          </div>
          <div className="form-group">
            <label>转录结果路径</label>
            <input
              type="text"
              value={config?.bilibili?.transcript_dir || ""}
              onChange={(e) => updateField("bilibili", "transcript_dir", e.target.value)}
              placeholder="~/Downloads/bilibili-download/transcript"
            />
          </div>
        </div>
      </div>

      {/* ============ Whisper 配置 ============ */}
      <div className="settings-section">
        <h3>🎤 Whisper 语音转文字</h3>
        <div className="form-group">
          <label>模式</label>
          <select
            value={config?.whisper?.mode || "openai"}
            onChange={(e) => updateField("whisper", "mode", e.target.value)}
          >
            <option value="openai">OpenAI 格式（远程 API）</option>
            <option value="local">本地 REST API（whisper.cpp 等）</option>
          </select>
        </div>
        <div className="form-group">
          <label>API 地址</label>
          <input
            type="text"
            value={config?.whisper?.api_url || ""}
            onChange={(e) => updateField("whisper", "api_url", e.target.value)}
            placeholder="https://api.openai.com/v1/audio/transcriptions"
          />
        </div>
        <div className="form-group">
          <label>API Key</label>
          <input
            type="password"
            value={config?.whisper?.api_key || ""}
            onChange={(e) => updateField("whisper", "api_key", e.target.value)}
            placeholder="sk-xxx"
          />
        </div>
        <div className="form-group">
          <label>模型名称</label>
          <input
            type="text"
            value={config?.whisper?.model || ""}
            onChange={(e) => updateField("whisper", "model", e.target.value)}
            placeholder="whisper-1"
          />
        </div>
        <button className="btn btn-secondary" onClick={handleTestWhisper}>🔌 测试连接</button>
      </div>

      {/* ============ AI 摘要 ============ */}
      <div className="settings-section">
        <h3>🤖 AI 摘要</h3>
        <div className="form-group">
          <label>
            <input
              type="checkbox"
              checked={config?.ai_summary?.enabled || false}
              onChange={(e) => updateField("ai_summary", "enabled", e.target.checked)}
            />{" "}
            启用 AI 摘要功能
          </label>
        </div>
        {config?.ai_summary?.enabled && (
          <>
            <div className="form-group">
              <label>API 地址</label>
              <input type="text" value={config?.ai_summary?.api_url || ""} onChange={(e) => updateField("ai_summary", "api_url", e.target.value)} placeholder="https://api.openai.com/v1/chat/completions" />
            </div>
            <div className="form-group">
              <label>API Key</label>
              <input type="password" value={config?.ai_summary?.api_key || ""} onChange={(e) => updateField("ai_summary", "api_key", e.target.value)} placeholder="sk-xxx" />
            </div>
            <div className="form-group">
              <label>模型名称</label>
              <input type="text" value={config?.ai_summary?.model || ""} onChange={(e) => updateField("ai_summary", "model", e.target.value)} placeholder="gpt-4o-mini" />
            </div>
          </>
        )}
      </div>

      <div className="settings-actions">
        <button className="btn btn-primary" onClick={handleSave} disabled={saving}>
          {saving ? "保存中..." : "💾 保存设置"}
        </button>
      </div>
    </div>
  );
}

export default Settings;
