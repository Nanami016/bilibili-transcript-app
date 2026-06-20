import { useState, useEffect } from "react";

function Settings() {
  const [config, setConfig] = useState<any>(null);
  const [cookieStatus, setCookieStatus] = useState(false);

  useEffect(() => {
    loadConfig();
  }, []);

  const loadConfig = async () => {
    try {
      // TODO: 调用 Tauri 命令获取配置
      setLoading(false);
    } catch (error) {
      console.error("Failed to load config:", error);
    }
  };

  const handleSave = async () => {
    try {
      // TODO: 调用 Tauri 命令保存配置
    } catch (error) {
      console.error("Failed to save config:", error);
    }
  };

  return (
    <div className="page settings-page">
      <h2>设置</h2>

      <div className="settings-section">
        <h3>🎤 Whisper 配置</h3>
        <div className="form-group">
          <label>模式</label>
          <select value={config?.whisper?.mode || "openai"}>
            <option value="openai">OpenAI 格式</option>
            <option value="local">本地 REST API</option>
          </select>
        </div>
        <div className="form-group">
          <label>API 地址</label>
          <input type="text" placeholder="https://api.openai.com/v1/audio/transcriptions" />
        </div>
        <div className="form-group">
          <label>API Key（可选）</label>
          <input type="password" placeholder="sk-xxx" />
        </div>
        <div className="form-group">
          <label>模型名称</label>
          <input type="text" placeholder="whisper-1" />
        </div>
        <button className="btn btn-secondary">测试连接</button>
      </div>

      <div className="settings-section">
        <h3>📺 B站配置</h3>
        <div className="form-group">
          <label>Cookie</label>
          <textarea placeholder="从浏览器导出的 Cookie..." rows={4} />
          <p className="form-hint">
            使用浏览器插件 Cookie-Editor 导出 Cookie 字符串
          </p>
        </div>
        <div className="form-group">
          <label>输出目录</label>
          <input type="text" placeholder="~/bilibili-voice-only" />
        </div>
      </div>

      <div className="settings-section">
        <h3>🤖 AI 摘要配置</h3>
        <div className="form-group">
          <label>
            <input type="checkbox" /> 启用 AI 摘要
          </label>
        </div>
        <div className="form-group">
          <label>API 地址</label>
          <input type="text" placeholder="https://api.openai.com/v1/chat/completions" />
        </div>
        <div className="form-group">
          <label>API Key</label>
          <input type="password" placeholder="sk-xxx" />
        </div>
        <div className="form-group">
          <label>模型名称</label>
          <input type="text" placeholder="gpt-4o-mini" />
        </div>
      </div>

      <div className="settings-actions">
        <button className="btn btn-primary" onClick={handleSave}>保存设置</button>
      </div>
    </div>
  );
}

export default Settings;
