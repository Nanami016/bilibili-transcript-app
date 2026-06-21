import { useState, useEffect, useRef } from "react";
import { getRunLogs, clearRunLogs } from "../lib/tauri";

interface LogEntry {
  timestamp: string;
  level: string;
  message: string;
}

function LogViewer() {
  const [logs, setLogs] = useState<LogEntry[]>([]);
  const [expanded, setExpanded] = useState(false);
  const [autoScroll, setAutoScroll] = useState(true);
  const logEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    // 初始加载
    refreshLogs();

    // 定时刷新
    const timer = setInterval(refreshLogs, 2000);
    return () => clearInterval(timer);
  }, []);

  useEffect(() => {
    if (autoScroll && logEndRef.current) {
      logEndRef.current.scrollIntoView({ behavior: "smooth" });
    }
  }, [logs, autoScroll]);

  const refreshLogs = async () => {
    try {
      const result = (await getRunLogs()) as LogEntry[];
      setLogs(result);
    } catch {
      // 忽略
    }
  };

  const handleClear = async () => {
    try {
      await clearRunLogs();
      setLogs([]);
    } catch {
      // 忽略
    }
  };

  const levelColor = (level: string) => {
    switch (level) {
      case "ERROR":
        return "#ff4d4f";
      case "WARN":
        return "#faad14";
      case "INFO":
        return "#52c41a";
      default:
        return "#999";
    }
  };

  return (
    <div className="settings-section">
      <div
        style={{
          display: "flex",
          justifyContent: "space-between",
          alignItems: "center",
          cursor: "pointer",
        }}
        onClick={() => setExpanded(!expanded)}
      >
        <h3>
          📋 运行日志
          <span style={{ fontSize: 12, color: "#999", marginLeft: 8 }}>
            {logs.length} 条
          </span>
        </h3>
        <span style={{ fontSize: 14, color: "#666" }}>{expanded ? "▲ 收起" : "▼ 展开"}</span>
      </div>

      {expanded && (
        <>
          <div
            style={{
              display: "flex",
              gap: 8,
              marginTop: 12,
              marginBottom: 12,
            }}
          >
            <button className="btn btn-secondary" style={{ fontSize: 12, padding: "4px 12px" }} onClick={refreshLogs}>
              🔄 刷新
            </button>
            <button className="btn btn-secondary" style={{ fontSize: 12, padding: "4px 12px" }} onClick={handleClear}>
              🗑️ 清空
            </button>
            <label
              style={{
                display: "flex",
                alignItems: "center",
                gap: 4,
                fontSize: 12,
                color: "#666",
                marginLeft: "auto",
              }}
            >
              <input
                type="checkbox"
                checked={autoScroll}
                onChange={(e) => setAutoScroll(e.target.checked)}
              />
              自动滚动
            </label>
          </div>

          <div
            style={{
              background: "#1e1e1e",
              borderRadius: 8,
              padding: 12,
              maxHeight: 300,
              overflow: "auto",
              fontFamily: "'SF Mono', Monaco, Consolas, monospace",
              fontSize: 12,
              lineHeight: 1.6,
            }}
          >
            {logs.length === 0 ? (
              <div style={{ color: "#666", textAlign: "center", padding: 20 }}>暂无日志</div>
            ) : (
              logs.map((log, i) => (
                <div key={i} style={{ display: "flex", gap: 8 }}>
                  <span style={{ color: "#666", minWidth: 60 }}>{log.timestamp}</span>
                  <span
                    style={{
                      color: levelColor(log.level),
                      minWidth: 40,
                      fontWeight: log.level === "ERROR" ? 600 : 400,
                    }}
                  >
                    {log.level}
                  </span>
                  <span style={{ color: "#d4d4d4", wordBreak: "break-all" }}>
                    {log.message}
                  </span>
                </div>
              ))
            )}
            <div ref={logEndRef} />
          </div>
        </>
      )}
    </div>
  );
}

export default LogViewer;
