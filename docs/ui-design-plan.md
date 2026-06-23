# Phase 7: UI 美化方案 — 毛玻璃 + B站粉 + 深色模式

## 一、设计方向

### 1.1 风格定义

| 关键词 | 说明 |
|--------|------|
| **毛玻璃 (Glassmorphism)** | `backdrop-filter: blur()` + 半透明背景 + 微妙边框，营造通透层次感 |
| **B站品牌感** | 以 B站粉 `#FB7299` 为主强调色，贴合 B站用户认知；背景用 Apple 原生白/黑，干净克制 |
| **现代感** | 流畅过渡动画、一致间距体系、干净排版、圆角卡片 |
| **深色模式** | CSS 变量双套主题，支持跟随系统 + 手动切换 |

> **性能说明**: 目标平台为 Apple Silicon (M 系列 Mac)，不考虑 Intel Mac 的渲染性能问题。

### 1.2 配色体系

| 角色 | 色名 | 浅色模式 | 深色模式 | 说明 |
|------|------|----------|----------|------|
| 主强调色 | B站粉 | `#FB7299` | `#FB7299` | 主按钮、active 状态、链接、收藏 |
| 主强调色悬浮 | — | `#e85a83` | `#ff8db5` | hover 状态 |
| 主背景 | Apple 白 | `#ffffff` | `#1c1c1e` | 页面底色（Apple 原生白/黑） |
| 次背景 | — | `#f5f5f7` | `#2c2c2e` | 卡片、面板 |
| 毛玻璃背景 | — | `rgba(255,255,255,0.72)` | `rgba(44,44,46,0.78)` | 侧边栏、浮层 |
| 毛玻璃悬浮 | — | `rgba(255,255,255,0.88)` | `rgba(58,58,60,0.88)` | hover 状态 |
| 主文字 | — | `#1d1d1f` | `#f5f5f7` | 标题、正文 |
| 次文字 | — | `#86868b` | `#98989d` | 辅助信息 |
| 边框 | — | `rgba(0,0,0,0.06)` | `rgba(255,255,255,0.08)` | 分割线、卡片边框 |
| 阴影-小 | — | `0 1px 3px rgba(0,0,0,0.06)` | `0 1px 3px rgba(0,0,0,0.3)` | 默认卡片 |
| 阴影-中 | — | `0 4px 12px rgba(0,0,0,0.08)` | `0 4px 12px rgba(0,0,0,0.4)` | 悬浮状态 |
| 阴影-大 | — | `0 8px 30px rgba(0,0,0,0.12)` | `0 8px 30px rgba(0,0,0,0.5)` | 弹窗、下拉 |

### 1.3 圆角体系

| 级别 | 值 | 用途 |
|------|-----|------|
| `--radius-sm` | `8px` | 按钮、输入框、小标签 |
| `--radius-md` | `12px` | 卡片、面板 |
| `--radius-lg` | `16px` | 弹窗、大容器 |
| `--radius-full` | `9999px` | pill 按钮、badge |

---

## 二、关键问题解决方案

### 2.1 窗口标题栏与 Sidebar 衔接

**问题**: macOS 系统标题栏是独立渲染的，和毛玻璃 sidebar 之间会有色差。

**解决方案**: 保留系统标题栏（红绿灯按钮），sidebar 背景色与标题栏接近以减少割裂感。

- 浅色模式：sidebar 毛玻璃背景 `rgba(255,255,255,0.72)` 与系统标题栏白色接近
- 深色模式：sidebar 毛玻璃背景 `rgba(44,44,46,0.78)` 与系统标题栏深色接近
- 不隐藏系统装饰，保留原生红绿灯按钮

### 2.2 动态内联样式处理策略

**问题**: Favorite.tsx、TaskPanel.tsx 等组件有大量条件内联样式（如 `style={{ color: status === "completed" ? "#52c41a" : "#ff4d4f" }}`），如果不转换，深色模式下颜色对不上。

**解决方案: CSS 变量 + 内联引用，而非 class 切换。**

```css
/* global.css 定义语义化变量 */
--status-success: #52c41a;
--status-error: #ff4d4f;
--status-warning: #faad14;
--status-info: #FB7299;
```

```tsx
// 内联样式改用 CSS 变量
style={{ color: task.status === "completed" ? "var(--status-success)" : "var(--status-error)" }}
```

**风险评估**:
- ✅ 无需改组件逻辑，只需替换色值字符串
- ✅ 深色模式自动适配（变量值在 `[data-theme="dark"]` 下可覆盖）
- ⚠️ 唯一风险：如果某些状态色在深色模式下需要完全不同的色值（如 success 绿在深色背景需要更亮），需要在 dark theme 下覆盖对应变量

**结论**: 保留条件内联样式，但把硬编码色值替换为 CSS 变量引用。这样既不改组件逻辑，又能适配深色模式。

### 2.3 GPU 加速策略

**问题**: 收藏夹页面 20+ 卡片同时 hover 触发 repaint。

**解决方案**:

```css
/* 所有会触发 transform 的卡片元素 */
.card, .favorite-item, .task-history-item {
  will-change: transform;
  transform: translateZ(0);  /* 强制 GPU 层 */
  transition: transform 0.2s ease, box-shadow 0.2s ease;
}

.card:hover {
  transform: translateY(-2px) translateZ(0);
  box-shadow: var(--shadow-md);
}
```

注意事项:
- `will-change` 只加在确实会动画的元素上，不要全局滥用
- `translateZ(0)` 创建独立合成层，M 系列 Mac 的 GPU 处理毫无压力
- 动画结束后浏览器会自动回收 GPU 层

### 2.4 弹窗动画文字模糊问题

**问题**: `scale(0.95→1)` 动画过程中文字出现亚像素模糊。

**解决方案**: 动画结束后再渲染文字内容。

```tsx
function Modal({ visible, children }) {
  const [showContent, setShowContent] = useState(false);

  useEffect(() => {
    if (visible) {
      // 动画结束后显示内容
      const timer = setTimeout(() => setShowContent(true), 200);
      return () => clearTimeout(timer);
    } else {
      setShowContent(false);
    }
  }, [visible]);

  if (!visible) return null;

  return (
    <div className="modal-overlay">
      <div className="modal-content">
        {showContent ? children : <div className="modal-placeholder" />}
      </div>
    </div>
  );
}
```

```css
.modal-content {
  animation: modalIn 0.2s ease-out forwards;
}

@keyframes modalIn {
  from {
    opacity: 0;
    transform: scale(0.95);
  }
  to {
    opacity: 1;
    transform: scale(1);
  }
}

/* 占位符保持容器尺寸，避免弹窗大小跳动 */
.modal-placeholder {
  min-height: 200px;
}
```

### 2.5 深色模式毛玻璃层次感

**问题**: 深色背景下毛玻璃效果不明显。

**解决方案**: 深色模式下用更亮的边框 + 微妙的内发光。

```css
[data-theme="dark"] .glass-card {
  background: var(--bg-glass);
  backdrop-filter: blur(20px) saturate(180%);
  border: 1px solid rgba(255, 255, 255, 0.08);
  box-shadow:
    0 4px 12px rgba(0, 0, 0, 0.4),
    inset 0 1px 0 rgba(255, 255, 255, 0.04);  /* 顶部内发光，模拟光边 */
}
```

### 2.6 滚动条样式

**解决方案**: 使用 `::-webkit-scrollbar` 自定义，适配双主题。

```css
::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

::-webkit-scrollbar-track {
  background: transparent;
}

::-webkit-scrollbar-thumb {
  background: rgba(0, 0, 0, 0.15);
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: rgba(0, 0, 0, 0.25);
}

[data-theme="dark"] ::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.15);
}

[data-theme="dark"] ::-webkit-scrollbar-thumb:hover {
  background: rgba(255, 255, 255, 0.25);
}
```

### 2.7 焦点状态 (focus-visible)

**问题**: 深色模式下 focus ring 颜色需要调整。

**解决方案**:

```css
:focus-visible {
  outline: 2px solid var(--accent);
  outline-offset: 2px;
  border-radius: var(--radius-sm);
}

/* 深色模式下 focus ring 需要更亮才能可见 */
[data-theme="dark"] :focus-visible {
  outline-color: var(--accent-hover);  /* #ff8db5，比 #FB7299 更亮 */
  box-shadow: 0 0 0 4px rgba(251, 114, 153, 0.2);  /* 额外光晕 */
}
```

### 2.8 深色模式图片处理

**解决方案**: 封面图片加微妙边框，防止与深色背景融合。

```css
.video-cover img,
.favorite-video-cover {
  border: 1px solid var(--border);
  border-radius: var(--radius-sm);
}

/* 深色模式下略微降低亮度，减少刺眼感 */
[data-theme="dark"] .video-cover img {
  filter: brightness(0.92);
}
```

---

## 三、组件改造方案

### 3.1 Sidebar — 侧边栏

| 项目 | 改前 | 改后 |
|------|------|------|
| 背景 | 纯白 `#fff` | 毛玻璃 `var(--bg-glass)` + `backdrop-filter: blur(20px) saturate(180%)` |
| active 项 | 整块蓝色背景 | 药丸 `border-radius: 8px` + `rgba(251,114,153,0.12)` 背景 + 左侧 3px accent 色条 |
| hover 项 | 灰色背景 | `var(--bg-glass-hover)` |
| Logo 区 | 纯文本 | lucide `Tv` 图标 + 标题，accent 色高亮 |
| 分隔线 | 实线 `#e0e0e0` | `var(--border)` 半透明线 |
| 顶部拖拽区 | 无 | 保留系统标题栏，不自定义拖拽区 |

### 3.2 InputBar — 输入框

| 项目 | 改前 | 改后 |
|------|------|------|
| 背景 | 纯白 | 毛玻璃 `var(--bg-glass)` |
| 边框 | `1px solid #e0e0e0` | `1px solid var(--border)` |
| focus | 边框变粉 | 边框变粉 + `box-shadow: 0 0 0 3px rgba(251,114,153,0.15)` 光晕 |
| 解析按钮 | 纯色 | B站粉渐变 `linear-gradient(135deg, #FB7299, #e85a83)` |

### 3.3 VideoCard — 视频卡片

| 项目 | 改前 | 改后 |
|------|------|------|
| 卡片背景 | 纯白 | 毛玻璃 + `will-change: transform` GPU 加速 |
| hover | 无 | `translateY(-2px)` + 阴影加深 |
| 封面 hover | 无 | `scale(1.02)` + overflow hidden |
| 时长标签 | 黑底白字 | 毛玻璃 pill `backdrop-filter: blur(8px)` |
| 操作按钮 | 基础 btn | pill 样式，紧凑间距 |
| 转录按钮 | 纯色 primary | B站粉渐变 |
| 深色模式封面 | 无处理 | `filter: brightness(0.92)` + `border: 1px solid var(--border)` |

### 3.4 TranscribeModal — 转录弹窗

| 项目 | 改前 | 改后 |
|------|------|------|
| 弹窗背景 | 纯白 | 毛玻璃 + 更大模糊 `blur(40px)` |
| 入场动画 | 无 | `scale(0.95→1) + opacity(0→1)` 200ms，动画结束后渲染文字 |
| 遮罩 | `rgba(0,0,0,0.45)` | `rgba(0,0,0,0.3)` + `backdrop-filter: blur(4px)` |
| 语言标签 | 蓝色边框 | pill：选中时 accent 背景 + 白字，未选时半透明背景 |
| 高级选项 | 文字按钮 | chevron icon + slide-down 动画 |

### 3.5 TaskPanel — 任务面板

| 项目 | 改前 | 改后 |
|------|------|------|
| 卡片背景 | 纯白 | 毛玻璃 |
| 进度条 | 纯色 | B站粉渐变 `linear-gradient(90deg, #FB7299, #ff8db5)` |
| 进度条 glow | 无 | 浅色模式不加，深色模式加 `box-shadow: 0 0 8px rgba(251,114,153,0.4)` |
| 历史行 hover | `#f5f5f5` | `var(--bg-glass-hover)` |
| 状态 badge | emoji + 文字 | 彩色 pill：success 绿 / error 红 / warning 黄 / info B站粉 |
| 操作按钮 | 文字按钮 | icon button（圆形，hover accent 色） |

### 3.6 TaskToast — 任务通知

| 项目 | 改前 | 改后 |
|------|------|------|
| 背景 | 纯白 | 毛玻璃 |
| 进度条 | 纯色 | B站粉渐变 + 进行中时脉冲动画 |
| 入场 | `translateX(40px→0)` | 保持 + `backdrop-filter` |

### 3.7 Favorite — 收藏夹页

**所有内联样式迁移为 CSS class，动态色值改用 CSS 变量。**

| 内联样式 | 改为 |
|----------|------|
| `style={{ display: "flex", gap: 12 }}` | `className="favorite-video-row"` |
| `style={{ width: 120, height: 68 }}` | `className="favorite-video-cover"` |
| `style={{ fontSize: 14, marginBottom: 4 }}` | `className="favorite-video-title"` |
| `style={{ fontSize: 12, color: "#666" }}` | `className="favorite-video-meta"` |
| `style={{ fontSize: 12, padding: "6px 12px" }}` | `className="btn btn-sm"` |
| `style={{ gridColumn: "1 / -1" }}` | `className="favorites-load-more"` |

### 3.8 Settings — 设置页

**内联样式迁移 + 新增主题切换控件。**

| 内联样式 | 改为 |
|----------|------|
| `style={{ borderTop: "1px solid #e0e0e0" }}` | `className="settings-divider"` |
| `style={{ fontSize: 12, color: "#52c41a" }}` | `className="status-badge status-success"` |
| `style={{ fontSize: 14, marginBottom: 12 }}` | `className="settings-subtitle"` |
| `style={{ display: "flex", gap: 8 }}` | `className="browser-btn-group"` |

新增「外观」分区:

```tsx
<div className="settings-section">
  <h3>🎨 外观</h3>
  <div className="theme-switcher">
    <button className={`theme-btn ${theme === "system" ? "active" : ""}`}>
      🌓 跟随系统
    </button>
    <button className={`theme-btn ${theme === "light" ? "active" : ""}`}>
      ☀️ 浅色
    </button>
    <button className={`theme-btn ${theme === "dark" ? "active" : ""}`}>
      🌙 深色
    </button>
  </div>
</div>
```

### 3.9 Logs — 日志页

| 内联样式 | 改为 |
|----------|------|
| `style={{ display: "flex", justifyContent: "space-between" }}` | `className="logs-header"` |
| `style={{ background: "#1e1e1e", borderRadius: 8 }}` | `className="logs-terminal"` |
| `style={{ color: "#666", minWidth: 70 }}` | `className="log-timestamp"` |
| `style={{ color: levelColor(...) }}` | `className={`log-level log-level-${level}`}` |
| `style={{ padding: "4px 8px" }}` | `className="logs-filter-select"` |

---

## 四、深色模式实现方案

### 4.1 主题存储

```
localStorage key: "theme"
值: "system" | "light" | "dark"
```

### 4.2 主题应用

- `data-theme` 属性挂在 `<html>` 上
- CSS 选择器: `[data-theme="dark"] { ... }`
- `system` 模式下监听 `prefers-color-scheme` 变化实时切换

### 4.3 Layout.tsx 主题初始化

```tsx
function Layout() {
  const [theme, setTheme] = useState(() => localStorage.getItem("theme") || "system");

  useEffect(() => {
    const applyTheme = (t: string) => {
      const resolved = t === "system"
        ? (window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light")
        : t;
      document.documentElement.setAttribute("data-theme", resolved);
    };

    applyTheme(theme);
    localStorage.setItem("theme", theme);

    if (theme === "system") {
      const mq = window.matchMedia("(prefers-color-scheme: dark)");
      const handler = () => applyTheme("system");
      mq.addEventListener("change", handler);
      return () => mq.removeEventListener("change", handler);
    }
  }, [theme]);

  return (
    <div className="app-layout">
      <Sidebar />
      <main className="main-content"><Outlet /></main>
      <TaskToast />
    </div>
  );
}
```

---

## 五、CSS 变量迁移映射表

一次性替换，以下是旧变量 → 新变量的完整映射:

| 旧变量 | 新变量 | 说明 |
|--------|--------|------|
| `--primary-color` | `--accent` | 主强调色（B站粉 `#FB7299`） |
| `--primary-hover` | `--accent-hover` | 强调色悬浮（`#e85a83` / `#ff8db5`） |
| `--bg-color` | `--bg-primary` | 主背景 |
| `--sidebar-bg` | `--bg-glass` | 侧边栏改用毛玻璃 |
| `--card-bg` | `--bg-secondary` | 卡片背景 |
| `--text-color` | `--text-primary` | 主文字 |
| `--text-secondary` | `--text-secondary` | 次文字（变量名不变） |
| `--border-color` | `--border` | 边框 |
| `--shadow` | `--shadow-md` | 中等阴影 |
| `--radius` | `--radius-sm` | 小圆角 |

**迁移步骤**:
1. 在 `global.css` 中先定义新变量
2. 保留旧变量作为 alias: `--primary-color: var(--accent);`
3. 逐文件替换旧变量引用为新变量
4. 全部替换完成后删除旧变量 alias

---

## 六、实施顺序

| 步骤 | 文件 | 内容 |
|------|------|------|
| 1 | `global.css` | 新变量体系 + 旧变量 alias + 滚动条 + focus-visible + 所有组件样式 |
| 2 | `Layout.tsx` | 主题初始化逻辑 |
| 3 | `Sidebar.tsx` | 侧边栏重构（drag-region + 毛玻璃 + active 样式） |
| 4 | `InputBar.tsx` + `VideoCard.tsx` | 首页核心组件 |
| 5 | `Home.tsx` | 空状态/loading 美化 |
| 6 | `TranscribeModal.tsx` | 弹窗美化 + 动画延迟渲染 |
| 7 | `TaskPanel.tsx` + `TaskToast.tsx` | 任务系统组件 |
| 8 | `Favorite.tsx` | 内联样式迁移 + CSS class |
| 9 | `Settings.tsx` | 内联样式迁移 + 主题切换控件 |
| 10 | `Logs.tsx` | 内联样式迁移 |
| 11 | `global.css` | 删除旧变量 alias，最终清理 |

---

## 七、验证方式

1. `npm run dev` 启动应用
2. 浅色模式逐页检查视觉效果
3. 切换深色模式，检查所有组件适配（重点：毛玻璃层次、文字可读性、边框可见性）
4. 测试 hover/focus/transition 动画
5. 搜索代码确认无硬编码色值残留（`grep -r "#fff\|#333\|#666\|#999\|#e0e0e0" src/`）
6. 测试弹窗动画文字渲染时机
7. 测试滚动条在两种主题下的显示
