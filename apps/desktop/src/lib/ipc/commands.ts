import { invoke } from "@tauri-apps/api/core";

export interface HealthDto {
  appVersion: string;
  dbPath: string;
  schemaVersion: number;
  sqliteVersion: string;
  startedAtMs: number;
  nowMs: number;
}

export type EntryStatus = "running" | "paused" | "closed";
export type IdleState = "active" | "paused" | "idle";

export interface ActiveEntryDto {
  id: string;
  projectId: string;
  projectName: string;
  projectCode: string;
  projectColor: string;
  taskId: string;
  taskTitle: string;
  status: EntryStatus;
  startedAt: number;
  lastResumedAt: number | null;
  accumulatedSeconds: number;
  elapsedSeconds: number;
}

export interface TimerSnapshotDto {
  active: ActiveEntryDto | null;
  idleState: IdleState;
  serverNowMs: number;
}

export interface DailyTotalDto {
  projectId: string;
  projectName: string;
  projectCode: string;
  taskId: string;
  taskTitle: string;
  totalSeconds: number;
  entryCount: number;
}

export interface CommandError {
  code: string;
  message: string;
}

export function getHealth(): Promise<HealthDto> {
  return invoke<HealthDto>("health_check");
}

export function getPreference(key: string): Promise<string | null> {
  return invoke<string | null>("get_preference", { key });
}

export function setPreference(key: string, valueJson: string): Promise<void> {
  return invoke<void>("set_preference", { key, valueJson });
}

export function getTimerSnapshot(): Promise<TimerSnapshotDto> {
  return invoke<TimerSnapshotDto>("get_timer_snapshot");
}

export function startTask(taskId?: string | null): Promise<TimerSnapshotDto> {
  return invoke<TimerSnapshotDto>("start_task", { taskId: taskId ?? null });
}

export function pauseCurrent(): Promise<TimerSnapshotDto> {
  return invoke<TimerSnapshotDto>("pause_current");
}

export function resumeCurrent(): Promise<TimerSnapshotDto> {
  return invoke<TimerSnapshotDto>("resume_current");
}

export function closeCurrent(): Promise<TimerSnapshotDto> {
  return invoke<TimerSnapshotDto>("close_current");
}

export function getTodayTotals(): Promise<DailyTotalDto[]> {
  return invoke<DailyTotalDto[]>("get_today_totals");
}
