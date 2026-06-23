import { useState, useEffect, createContext, useContext } from "react";
import { Outlet } from "react-router-dom";
import { getCurrentWindow } from "@tauri-apps/api/window";
import Sidebar from "./Sidebar";
import TaskToast from "./TaskToast";

type Theme = "system" | "light" | "dark";

interface ThemeContextValue {
  theme: Theme;
  setTheme: (t: Theme) => void;
  resolved: "light" | "dark";
}

const ThemeContext = createContext<ThemeContextValue>({
  theme: "system",
  setTheme: () => {},
  resolved: "light",
});

export function useTheme() {
  return useContext(ThemeContext);
}

function resolveTheme(t: Theme): "light" | "dark" {
  if (t !== "system") return t;
  return window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
}

function Layout() {
  const [theme, setTheme] = useState<Theme>(
    () => (localStorage.getItem("theme") as Theme) || "system"
  );
  const [resolved, setResolved] = useState<"light" | "dark">(() => resolveTheme(theme));

  useEffect(() => {
    const r = resolveTheme(theme);
    setResolved(r);
    document.documentElement.setAttribute("data-theme", r);
    localStorage.setItem("theme", theme);

    // 同步 macOS 系统标题栏颜色
    try {
      const appWindow = getCurrentWindow();
      appWindow.setTheme(r === "dark" ? "dark" : "light");
    } catch (e) {
      console.error("setTheme failed:", e);
    }

    if (theme === "system") {
      const mq = window.matchMedia("(prefers-color-scheme: dark)");
      const handler = () => {
        const newResolved = mq.matches ? "dark" : "light";
        setResolved(newResolved);
        document.documentElement.setAttribute("data-theme", newResolved);
        try {
          getCurrentWindow().setTheme(newResolved === "dark" ? "dark" : "light");
        } catch {
          // ignore
        }
      };
      mq.addEventListener("change", handler);
      return () => mq.removeEventListener("change", handler);
    }
  }, [theme]);

  return (
    <ThemeContext.Provider value={{ theme, setTheme, resolved }}>
      <div className="app-layout">
        <Sidebar />
        <main className="main-content">
          <Outlet />
        </main>
        <TaskToast />
      </div>
    </ThemeContext.Provider>
  );
}

export default Layout;
