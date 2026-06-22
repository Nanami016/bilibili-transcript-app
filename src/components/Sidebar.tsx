import { Link, useLocation } from "react-router-dom";
import { Home, Star, Download, Music, Mic, ScrollText, Settings } from "lucide-react";

function Sidebar() {
  const location = useLocation();

  return (
    <aside className="sidebar">
      <div className="sidebar-header">
        <h1 className="app-title">📺 Bili Transcript</h1>
      </div>

      <nav className="sidebar-nav">
        <Link
          to="/"
          className={`nav-item ${location.pathname === "/" ? "active" : ""}`}
        >
          <Home size={20} />
          <span>首页</span>
        </Link>
        <Link
          to="/favorite"
          className={`nav-item ${location.pathname === "/favorite" ? "active" : ""}`}
        >
          <Star size={20} />
          <span>收藏夹</span>
        </Link>

        <div className="nav-divider" />
        <div className="nav-group-label">任务中心</div>

        <Link
          to="/tasks/video"
          className={`nav-item nav-item-sub ${location.pathname === "/tasks/video" ? "active" : ""}`}
        >
          <Download size={18} />
          <span>视频下载</span>
        </Link>
        <Link
          to="/tasks/audio"
          className={`nav-item nav-item-sub ${location.pathname === "/tasks/audio" ? "active" : ""}`}
        >
          <Music size={18} />
          <span>音频下载</span>
        </Link>
        <Link
          to="/tasks/transcribe"
          className={`nav-item nav-item-sub ${location.pathname === "/tasks/transcribe" ? "active" : ""}`}
        >
          <Mic size={18} />
          <span>音频转录</span>
        </Link>
      </nav>

      <div className="sidebar-footer">
        <Link
          to="/logs"
          className={`nav-item ${location.pathname === "/logs" ? "active" : ""}`}
        >
          <ScrollText size={20} />
          <span>运行日志</span>
        </Link>
        <Link
          to="/settings"
          className={`nav-item ${location.pathname === "/settings" ? "active" : ""}`}
        >
          <Settings size={20} />
          <span>设置</span>
        </Link>
      </div>
    </aside>
  );
}

export default Sidebar;
