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

export interface TaskDto {
  id: string;
  projectId: string;
  projectName: string;
  projectCode: string;
  projectColor: string;
  title: string;
  externalId: string | null;
  lastUsedAt: number | null;
  isLocal: boolean;
}
