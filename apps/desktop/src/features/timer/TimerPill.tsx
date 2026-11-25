import { useEffect, useState } from "react";

import { formatDuration } from "../../lib/format/duration";
import type { TimerSnapshotDto } from "../../lib/ipc/commands";
import { elapsedSeconds } from "../../lib/timer/elapsed";
import "./timer.css";

interface TimerPillProps {
  snapshot: TimerSnapshotDto | null;
  busy: boolean;
  error: string | null;
  onToggle: () => void;
}

export function TimerPill({ snapshot, busy, error, onToggle }: TimerPillProps) {
  const [nowMs, setNowMs] = useState(() => Date.now());

  useEffect(() => {
    const interval = window.setInterval(() => setNowMs(Date.now()), 1000);
    return () => window.clearInterval(interval);
  }, []);

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
            <rect x="4" y="3.5" width="3" height="9" rx="1" fill="currentColor" />
            <rect x="9" y="3.5" width="3" height="9" rx="1" fill="currentColor" />
          </svg>
        ) : (
          <svg viewBox="0 0 16 16" width="12" height="12" aria-hidden="true">
            <path d="M5 3.5 12 8l-7 4.5z" fill="currentColor" />
          </svg>
        )}
      </button>
    </div>
  );
}
