import { useState, useEffect, useRef } from "react";
import {
  onTaskProgress,
  onTaskCompleted,
  type TaskProgressEvent,
  type TaskCompletedEvent,
} from "../lib/tauri";

interface ActiveToast {
  task_id: number;
  task_type: string;
  title: string;
  progress: number;
  speed: string;
  eta: string;
  status: string; // "running" | "completed" | "failed"
  fadingOut?: boolean;
}

const TASK_TYPE_LABELS: Record<string, string> = {
  video_download: "📹 视频下载",
  audio_download: "🎵 音频下载",
  transcribe: "📝 音频转录",
  ai_summary: "🤖 AI 分析",
};

function TaskToast() {
  const [toasts, setToasts] = useState<Map<number, ActiveToast>>(new Map());
  const dismissTimers = useRef<Map<number, ReturnType<typeof setTimeout>>>(new Map());

  // 清理定时器
  useEffect(() => {
    return () => {
      dismissTimers.current.forEach((timer) => clearTimeout(timer));
    };
  }, []);

  // 自动消失
  const scheduleDismiss = (taskId: number) => {
    const timer = setTimeout(() => {
      setToasts((prev) => {
        const next = new Map(prev);
        next.delete(taskId);
        return next;
      });
      dismissTimers.current.delete(taskId);
    }, 3000);
    dismissTimers.current.set(taskId, timer);
  };

  useEffect(() => {
    const unlistenFns: (() => void)[] = [];

    onTaskProgress((event: TaskProgressEvent) => {
      // 取消之前的消失定时器（如果任务重新活跃）
      const existing = dismissTimers.current.get(event.task_id);
      if (existing) {
        clearTimeout(existing);
        dismissTimers.current.delete(event.task_id);
      }

      setToasts((prev) => {
        const next = new Map(prev);
        const label = TASK_TYPE_LABELS[event.task_type] || event.task_type;
        const existing = next.get(event.task_id);
        next.set(event.task_id, {
          task_id: event.task_id,
          task_type: event.task_type,
          title: existing?.title || label,
          progress: event.progress,
          speed: event.speed,
          eta: event.eta,
          status: "running",
        });
        return next;
      });
    }).then((fn) => unlistenFns.push(fn));

    onTaskCompleted((event: TaskCompletedEvent) => {
      const label = TASK_TYPE_LABELS[event.task_type] || event.task_type;
      setToasts((prev) => {
        const next = new Map(prev);
        next.set(event.task_id, {
          task_id: event.task_id,
          task_type: event.task_type,
          title: event.title || label,
          progress: event.status === "completed" ? 100 : 0,
          speed: "",
          eta: "",
          status: event.status,
        });
        return next;
      });

      // 3 秒后自动消失
      scheduleDismiss(event.task_id);
    }).then((fn) => unlistenFns.push(fn));

    return () => unlistenFns.forEach((fn) => fn());
  }, []);

  if (toasts.size === 0) return null;

  return (
    <div className="task-toast-container">
      {Array.from(toasts.values()).map((toast) => (
        <div
          key={toast.task_id}
          className={`task-toast task-toast-${toast.status}`}
        >
          <div className="task-toast-header">
            <span className="task-toast-title">
              {toast.status === "completed"
                ? `✅ ${toast.title}`
                : toast.status === "failed"
                ? `❌ ${toast.title}`
                : `${TASK_TYPE_LABELS[toast.task_type] || toast.task_type}`}
            </span>
            <button
              className="task-toast-close"
              onClick={() => {
                const timer = dismissTimers.current.get(toast.task_id);
                if (timer) clearTimeout(timer);
                dismissTimers.current.delete(toast.task_id);
                setToasts((prev) => {
                  const next = new Map(prev);
                  next.delete(toast.task_id);
                  return next;
                });
              }}
            >
              ×
            </button>
          </div>

          {toast.status === "running" && (
            <>
              <div className="task-toast-progress-wrapper">
                <div
                  className="task-toast-progress-bar"
                  style={{ width: `${toast.progress}%` }}
                />
              </div>
              <div className="task-toast-meta">
                <span>{Math.round(toast.progress)}%</span>
                {toast.speed && <span>{toast.speed}</span>}
                {toast.eta && <span>{toast.eta}</span>}
              </div>
            </>
          )}

          {toast.status === "completed" && (
            <div className="task-toast-meta">
              <span style={{ color: "#52c41a" }}>已完成</span>
            </div>
          )}

          {toast.status === "failed" && (
            <div className="task-toast-meta">
              <span style={{ color: "#ff4d4f" }}>下载失败</span>
            </div>
          )}
        </div>
      ))}
    </div>
  );
}

export default TaskToast;
