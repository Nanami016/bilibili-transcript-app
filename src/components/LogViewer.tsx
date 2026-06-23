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
    refreshLogs();
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

  return (
    <div className="settings-section">
      <div
        style={{ display: "flex", justifyContent: "space-between", alignItems: "center", cursor: "pointer" }}
        onClick={() => setExpanded(!expanded)}
      >
        <h3>
          📋 运行日志
          <span style={{ fontSize: 12, color: "var(--text-secondary)", marginLeft: 8 }}>
            {logs.length} 条
          </span>
        </h3>
        <span style={{ fontSize: 14, color: "var(--text-secondary)" }}>{expanded ? "▲ 收起" : "▼ 展开"}</span>
      </div>

      {expanded && (
        <>
          <div style={{ display: "flex", gap: 8, marginTop: 12, marginBottom: 12 }}>
            <button className="btn btn-secondary btn-sm" onClick={refreshLogs}>
              🔄 刷新
            </button>
            <button className="btn btn-secondary btn-sm" onClick={handleClear}>
              🗑️ 清空
            </button>
            <label style={{ display: "flex", alignItems: "center", gap: 4, fontSize: 12, color: "var(--text-secondary)", marginLeft: "auto" }}>
              <input type="checkbox" checked={autoScroll} onChange={(e) => setAutoScroll(e.target.checked)} />
              自动滚动
            </label>
          </div>

          <div className="logs-terminal" style={{ maxHeight: 300 }}>
            {logs.length === 0 ? (
              <div className="logs-empty">暂无日志</div>
            ) : (
              logs.map((log, i) => (
                <div key={i} className="log-line">
                  <span className="log-timestamp">{log.timestamp}</span>
                  <span className={`log-level log-level-${log.level}`}>{log.level}</span>
                  <span className="log-message">{log.message}</span>
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
