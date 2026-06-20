import { Link, useLocation } from "react-router-dom";
import { Home, Star, Settings } from "lucide-react";

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
      </nav>

      <div className="sidebar-footer">
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
