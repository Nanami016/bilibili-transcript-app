import { useState, useEffect } from "react";

function Favorite() {
  const [favorites, setFavorites] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadFavorites();
  }, []);

  const loadFavorites = async () => {
    try {
      // TODO: 调用 Tauri 命令获取收藏夹列表
      setLoading(false);
    } catch (error) {
      console.error("Failed to load favorites:", error);
      setLoading(false);
    }
  };

  return (
    <div className="page favorite-page">
      <h2>我的收藏夹</h2>

      {loading ? (
        <div className="loading">加载中...</div>
      ) : favorites.length === 0 ? (
        <div className="empty-state">
          <p>请先在设置中登录 B站账号</p>
        </div>
      ) : (
        <div className="favorites-list">
          {favorites.map((folder) => (
            <div key={folder.id} className="favorite-item">
              <h3>{folder.title}</h3>
              <span>{folder.media_count} 个视频</span>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}

export default Favorite;
