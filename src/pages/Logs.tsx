import { useState, useEffect, useRef } from "react";
import { getRunLogs, clearRunLogs } from "../lib/tauri";

interface LogEntry {
  timestamp: string;
  level: string;
  message: string;
}

function Logs() {
  const [logs, setLogs] = useState<LogEntry[]>([]);
  const [autoScroll, setAutoScroll] = useState(true);
  const [filter, setFilter] = useState<string>("ALL");
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

  const levelColor = (level: string) => {
    switch (level) {
      case "ERROR":
        return "#ff4d4f";
      case "WARN":
        return "#faad14";
      case "INFO":
        return "#52c41a";
      case "DEBUG":
        return "#1890ff";
      case "TRACE":
        return "#999";
      default:
        return "#999";
    }
  };

  const filteredLogs = filter === "ALL" ? logs : logs.filter((l) => l.level === filter);

  return (
    <div className="page logs-page">
      <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", marginBottom: 16 }}>
        <h2>📋 运行日志</h2>
        <div style={{ display: "flex", gap: 8, alignItems: "center" }}>
          <select
            value={filter}
            onChange={(e) => setFilter(e.target.value)}
            style={{ padding: "4px 8px", borderRadius: 4, border: "1px solid #e0e0e0", fontSize: 13 }}
          >
            <option value="ALL">全部</option>
            <option value="ERROR">ERROR</option>
            <option value="WARN">WARN</option>
            <option value="INFO">INFO</option>
            <option value="DEBUG">DEBUG</option>
            <option value="TRACE">TRACE</option>
          </select>
          <button className="btn btn-secondary" style={{ fontSize: 12, padding: "4px 12px" }} onClick={refreshLogs}>
            🔄 刷新
          </button>
          <button className="btn btn-secondary" style={{ fontSize: 12, padding: "4px 12px" }} onClick={handleClear}>
            🗑️ 清空
          </button>
          <label style={{ display: "flex", alignItems: "center", gap: 4, fontSize: 12, color: "#666" }}>
            <input type="checkbox" checked={autoScroll} onChange={(e) => setAutoScroll(e.target.checked)} />
            自动滚动
          </label>
        </div>
      </div>

      <div
        style={{
          background: "#1e1e1e",
          borderRadius: 8,
          padding: 16,
          height: "calc(100vh - 160px)",
          overflow: "auto",
          fontFamily: "'SF Mono', Monaco, Consolas, monospace",
          fontSize: 13,
          lineHeight: 1.7,
        }}
      >
        {filteredLogs.length === 0 ? (
          <div style={{ color: "#666", textAlign: "center", padding: 40 }}>暂无日志</div>
        ) : (
          filteredLogs.map((log, i) => (
            <div key={i} style={{ display: "flex", gap: 12 }}>
              <span style={{ color: "#666", minWidth: 70 }}>{log.timestamp}</span>
              <span
                style={{
                  color: levelColor(log.level),
                  minWidth: 45,
                  fontWeight: log.level === "ERROR" ? 600 : 400,
                }}
              >
                {log.level}
              </span>
              <span style={{ color: "#d4d4d4", wordBreak: "break-all" }}>{log.message}</span>
            </div>
          ))
        )}
        <div ref={logEndRef} />
      </div>

      <div style={{ marginTop: 8, fontSize: 12, color: "#999", textAlign: "right" }}>
        共 {filteredLogs.length} 条日志
      </div>
    </div>
  );
}

export default Logs;
