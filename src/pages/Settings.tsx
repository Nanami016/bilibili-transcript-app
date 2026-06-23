import { useState, useEffect, useRef } from "react";
import { getConfig, updateConfig, importCookie, importCookieFromBrowser, getCookieStatus, testWhisperConnection, testAiSummaryConnection } from "../lib/tauri";
import { useTheme } from "../components/Layout";

function Settings() {
  const [config, setConfig] = useState<any>(null);
  const [loading, setLoading] = useState(true);
  const [cookieStatus, setCookieStatus] = useState(false);
  const [saving, setSaving] = useState(false);
  const [autoSaved, setAutoSaved] = useState(false);
  const [toast, setToast] = useState<{ message: string; type: "success" | "error" | "info" } | null>(null);
  const [manualCookie, setManualCookie] = useState("");
  const [browserLoading, setBrowserLoading] = useState(false);
  const skipAutoSaveRef = useRef(true);
  const { theme, setTheme } = useTheme();

  useEffect(() => { loadConfig(); }, []);

  useEffect(() => {
    if (toast) {
      const timer = setTimeout(() => setToast(null), 4000);
      return () => clearTimeout(timer);
    }
  }, [toast]);

  useEffect(() => {
    if (skipAutoSaveRef.current) {
      skipAutoSaveRef.current = false;
      return;
    }
    if (!config) return;
    const timer = setTimeout(async () => {
      try {
        await updateConfig(config);
        setAutoSaved(true);
        setTimeout(() => setAutoSaved(false), 2000);
      } catch (error) {
        console.error("自动保存失败:", error);
      }
    }, 800);
    return () => clearTimeout(timer);
  }, [config]);

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

  const handleTestAiSummary = async () => {
    try {
      const result = await testAiSummaryConnection();
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
    return (
      <div className="page">
        <div className="loading">
          <div className="spinner-circle" />
          <span>加载中...</span>
        </div>
      </div>
    );
  }

  return (
    <div className="page settings-page">
      <div className="task-page-header">
        <h2>设置</h2>
      </div>

      {toast && (
        <div className={`toast toast-${toast.type}`}>
          <span>{toast.message}</span>
          <button className="toast-close" onClick={() => setToast(null)}>×</button>
        </div>
      )}

      {/* 外观 */}
      <div className="settings-section">
        <h3>🎨 外观</h3>
        <div className="theme-switcher">
          <button
            className={`theme-btn ${theme === "system" ? "active" : ""}`}
            onClick={() => setTheme("system")}
          >
            🌓 跟随系统
          </button>
          <button
            className={`theme-btn ${theme === "light" ? "active" : ""}`}
            onClick={() => setTheme("light")}
          >
            ☀️ 浅色
          </button>
          <button
            className={`theme-btn ${theme === "dark" ? "active" : ""}`}
            onClick={() => setTheme("dark")}
          >
            🌙 深色
          </button>
        </div>
      </div>

      {/* B站配置 */}
      <div className="settings-section">
        <h3>
          📺 B站账号
          {cookieStatus
            ? <span className="status-badge status-success" style={{ marginLeft: 8 }}>✅ 已配置</span>
            : <span className="status-badge status-error" style={{ marginLeft: 8 }}>⚠️ 未配置</span>
          }
        </h3>

        <div style={{ marginBottom: 24 }}>
          <h4 className="settings-subtitle">方式一：从浏览器读取（推荐）</h4>
          <p className="form-hint" style={{ marginBottom: 12 }}>
            确保你已在浏览器中登录 B站，然后点击对应按钮读取 Cookie（包含 SESSDATA）
          </p>
          {browserLoading ? (
            <div className="loading" style={{ height: "auto", padding: 16 }}>
              <div className="spinner-circle" />
              <span>正在读取浏览器 Cookie...</span>
            </div>
          ) : (
            <div className="browser-btn-group">
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

        <hr className="settings-divider" />

        <div>
          <h4 className="settings-subtitle">方式二：手动导入 Cookie</h4>
          <div className="tutorial-steps">
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

        <div style={{ marginTop: 20 }}>
          <h4 className="settings-subtitle">📁 输出目录</h4>
          <div className="form-group">
            <label>视频下载路径</label>
            <input
              type="text"
              value={config?.bilibili?.video_dir || ""}
              onChange={(e) => updateField("bilibili", "video_dir", e.target.value)}
              placeholder="~/Downloads/bilibili-transcript-app/bilibili-video"
            />
          </div>
          <div className="form-group">
            <label>音频下载路径</label>
            <input
              type="text"
              value={config?.bilibili?.audio_dir || ""}
              onChange={(e) => updateField("bilibili", "audio_dir", e.target.value)}
              placeholder="~/Downloads/bilibili-transcript-app/bilibili-audio"
            />
          </div>
          <div className="form-group">
            <label>转录结果路径</label>
            <input
              type="text"
              value={config?.bilibili?.transcript_dir || ""}
              onChange={(e) => updateField("bilibili", "transcript_dir", e.target.value)}
              placeholder="~/Downloads/bilibili-transcript-app/bilibili-transfer"
            />
          </div>
        </div>
      </div>

      {/* Whisper 配置 */}
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
          {config?.whisper?.mode === "local" && (
            <p className="form-hint" style={{ marginTop: 4 }}>
              💡 本地 Whisper 服务部署可参考项目：
              <a href="https://github.com/Nanami016/FunAudioLLM-Server" target="_blank" rel="noopener noreferrer" style={{ color: "var(--accent)", marginLeft: 4 }}>
                FunAudioLLM-Server
              </a>
            </p>
          )}
        </div>
        <div className="form-group">
          <label>API 地址 <span style={{ color: "var(--status-error)" }}>*</span></label>
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

      {/* AI 摘要 */}
      <div className="settings-section">
        <h3>🤖 AI 摘要</h3>
        <div className="form-group">
          <label style={{ display: "flex", alignItems: "center", gap: 8, cursor: "pointer" }}>
            <input
              type="checkbox"
              checked={config?.ai_summary?.enabled || false}
              onChange={(e) => updateField("ai_summary", "enabled", e.target.checked)}
            />
            启用 AI 摘要功能
          </label>
        </div>
        {config?.ai_summary?.enabled && (
          <>
            <div className="form-group">
              <label>API 地址 <span style={{ color: "var(--status-error)" }}>*</span></label>
              <input type="text" value={config?.ai_summary?.api_url || ""} onChange={(e) => updateField("ai_summary", "api_url", e.target.value)} placeholder="https://api.openai.com/v1" />
              <p className="form-hint">填写 base URL，自动追加 /chat/completions</p>
            </div>
            <div className="form-group">
              <label>API Key</label>
              <input type="password" value={config?.ai_summary?.api_key || ""} onChange={(e) => updateField("ai_summary", "api_key", e.target.value)} placeholder="sk-xxx" />
            </div>
            <div className="form-group">
              <label>模型名称</label>
              <input type="text" value={config?.ai_summary?.model || ""} onChange={(e) => updateField("ai_summary", "model", e.target.value)} placeholder="gpt-4o-mini" />
            </div>
            <button className="btn btn-secondary" onClick={handleTestAiSummary}>🔌 测试连接</button>
          </>
        )}
      </div>

      <div className="settings-actions">
        <button className="btn btn-primary" onClick={handleSave} disabled={saving}>
          {saving ? "保存中..." : "💾 保存设置"}
        </button>
        {autoSaved && (
          <span className="auto-save-indicator">✅ 已自动保存</span>
        )}
      </div>
    </div>
  );
}

export default Settings;
