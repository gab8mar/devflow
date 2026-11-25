import type { ActiveEntryDto } from "../ipc/commands";

export function elapsedSeconds(
  active: ActiveEntryDto | null,
  nowMs: number,
): number {
  if (!active) {
    return 0;
  }

  if (active.status !== "running") {
    return Math.max(0, Math.floor(active.elapsedSeconds));
  }

  const resumedAt = active.lastResumedAt ?? active.startedAt;
  const live = Number.isFinite(nowMs)
    ? Math.max(0, nowMs - resumedAt) / 1000
    : 0;

  return active.accumulatedSeconds + Math.floor(live);
}
