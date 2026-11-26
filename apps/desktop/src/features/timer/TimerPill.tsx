import { LogicalSize } from "@tauri-apps/api/dpi";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useEffect, useState } from "react";

import { formatDuration } from "../../lib/format/duration";
import type { TaskDto, TimerSnapshotDto } from "../../lib/ipc/commands";
import { elapsedSeconds } from "../../lib/timer/elapsed";
import { TaskPicker } from "./TaskPicker";
import "./timer.css";

interface TimerPillProps {
  snapshot: TimerSnapshotDto | null;
  busy: boolean;
  error: string | null;
  pickerOpen: boolean;
  tasks: TaskDto[];
  tasksBusy: boolean;
  tasksError: string | null;
  onToggle: () => void;
  onOpenPicker: () => void;
  onClosePicker: () => void;
  onApplyTask: (taskId: string) => void;
  onCreateTask: (title: string) => Promise<TaskDto | null>;
}

export function TimerPill({
  snapshot,
  busy,
  error,
  pickerOpen,
  tasks,
  tasksBusy,
  tasksError,
  onToggle,
  onOpenPicker,
  onClosePicker,
  onApplyTask,
  onCreateTask,
}: TimerPillProps) {
  const [nowMs, setNowMs] = useState(() => Date.now());

  useEffect(() => {
    const interval = window.setInterval(() => setNowMs(Date.now()), 1000);
    return () => window.clearInterval(interval);
  }, []);

  useEffect(() => {
    const size = pickerOpen
      ? new LogicalSize(320, 440)
      : new LogicalSize(132, 28);
    void getCurrentWindow()
      .setSize(size)
      .catch(() => undefined);
  }, [pickerOpen]);

  const active = snapshot?.active ?? null;
  const running = active?.status === "running";
  const paused = active?.status === "paused";
  const state = running ? "running" : paused ? "paused" : "idle";
  const stateLabel = running
    ? "En curso"
    : paused
      ? "En pausa"
      : "Sin tarea activa";
  const seconds = elapsedSeconds(active, nowMs);
  const actionLabel = running
    ? "Pausar"
    : paused
      ? "Reanudar"
      : "Iniciar tarea general";

  const tooltip = active
    ? `${active.projectName} · ${active.taskTitle} · ${stateLabel}`
    : error ?? `DevTime · ${stateLabel}`;

  return (
    <div className="dt-shell">
      <div
        className={`dt-pill${error && !active ? " dt-pill--error" : ""}`}
        data-tauri-drag-region
        title={tooltip}
      >
        <span className="dt-icon" data-tauri-drag-region aria-hidden="true">
          <svg viewBox="0 0 16 16" width="13" height="13">
            <circle
              cx="8"
              cy="8"
              r="6"
              fill="none"
              stroke="currentColor"
              strokeWidth="1.4"
            />
            <path
              d="M8 4.5V8l2.5 1.5"
              fill="none"
              stroke="currentColor"
              strokeWidth="1.4"
              strokeLinecap="round"
            />
          </svg>
        </span>
        <span className="dt-time" data-tauri-drag-region>
          {formatDuration(seconds)}
        </span>
        <button
          className="dt-caret"
          type="button"
          onClick={pickerOpen ? onClosePicker : onOpenPicker}
          aria-label={pickerOpen ? "Cerrar selector de tareas" : "Elegir tarea"}
          aria-expanded={pickerOpen}
          title="Elegir tarea"
        >
          <svg viewBox="0 0 10 10" width="9" height="9" aria-hidden="true">
            <path
              d="M2.5 4 5 6.5 7.5 4"
              fill="none"
              stroke="currentColor"
              strokeWidth="1.4"
              strokeLinecap="round"
              strokeLinejoin="round"
            />
          </svg>
        </button>
        <span
          className={`dt-state dt-state--${state}`}
          data-tauri-drag-region
          aria-label={stateLabel}
        />
        <button
          className="dt-toggle"
          type="button"
          onClick={onToggle}
          disabled={busy}
          aria-label={actionLabel}
          title={actionLabel}
        >
          {running ? (
            <svg viewBox="0 0 16 16" width="12" height="12" aria-hidden="true">
              <rect
                x="4"
                y="3.5"
                width="3"
                height="9"
                rx="1"
                fill="currentColor"
              />
              <rect
                x="9"
                y="3.5"
                width="3"
                height="9"
                rx="1"
                fill="currentColor"
              />
            </svg>
          ) : (
            <svg viewBox="0 0 16 16" width="12" height="12" aria-hidden="true">
              <path d="M5 3.5 12 8l-7 4.5z" fill="currentColor" />
            </svg>
          )}
        </button>
      </div>

      {pickerOpen ? (
        <TaskPicker
          tasks={tasks}
          activeTaskId={active?.taskId ?? null}
          busy={tasksBusy}
          error={tasksError ?? error}
          onApply={onApplyTask}
          onCreate={onCreateTask}
          onClose={onClosePicker}
        />
      ) : null}
    </div>
  );
}
