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

  const filteredLogs = filter === "ALL" ? logs : logs.filter((l) => l.level === filter);

  return (
    <div className="page logs-page">
      <div className="logs-header">
        <h2>📋 运行日志</h2>
        <div className="logs-toolbar">
          <select
            className="logs-filter-select"
            value={filter}
            onChange={(e) => setFilter(e.target.value)}
          >
            <option value="ALL">全部</option>
            <option value="ERROR">ERROR</option>
            <option value="WARN">WARN</option>
            <option value="INFO">INFO</option>
            <option value="DEBUG">DEBUG</option>
            <option value="TRACE">TRACE</option>
          </select>
          <button className="btn btn-secondary btn-sm" onClick={refreshLogs}>
            🔄 刷新
          </button>
          <button className="btn btn-secondary btn-sm" onClick={handleClear}>
            🗑️ 清空
          </button>
          <label style={{ display: "flex", alignItems: "center", gap: 4, fontSize: 12, color: "var(--text-secondary)" }}>
            <input type="checkbox" checked={autoScroll} onChange={(e) => setAutoScroll(e.target.checked)} />
            自动滚动
          </label>
        </div>
      </div>

      <div className="logs-terminal">
        {filteredLogs.length === 0 ? (
          <div className="logs-empty">暂无日志</div>
        ) : (
          filteredLogs.map((log, i) => (
            <div key={i} className="log-line">
              <span className="log-timestamp">{log.timestamp}</span>
              <span className={`log-level log-level-${log.level}`}>{log.level}</span>
              <span className="log-message">{log.message}</span>
            </div>
          ))
        )}
        <div ref={logEndRef} />
      </div>

      <div className="logs-footer">
        共 {filteredLogs.length} 条日志
      </div>
    </div>
  );
}

export default Logs;
