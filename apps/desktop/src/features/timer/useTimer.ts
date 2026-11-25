import { useCallback, useEffect, useRef, useState } from "react";

import {
  getTimerSnapshot,
  pauseCurrent,
  resumeCurrent,
  startTask,
  type TimerSnapshotDto,
} from "../../lib/ipc/commands";

const POLL_INTERVAL_MS = 15_000;

function describeError(error: unknown): string {
  if (typeof error === "string") {
    return error;
  }
  if (
    error &&
    typeof error === "object" &&
    "message" in error &&
    typeof error.message === "string"
  ) {
    return error.message;
  }
  return "No se pudo actualizar el cronómetro";
}

export function useTimer() {
  const [snapshot, setSnapshot] = useState<TimerSnapshotDto | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const mounted = useRef(true);

  const apply = useCallback((next: TimerSnapshotDto) => {
    if (mounted.current) {
      setSnapshot(next);
      setError(null);
    }
  }, []);

  const refresh = useCallback(async () => {
    try {
      apply(await getTimerSnapshot());
    } catch (cause) {
      if (mounted.current) {
        setError(describeError(cause));
      }
    }
  }, [apply]);

  useEffect(() => {
    mounted.current = true;
    void refresh();
    const interval = window.setInterval(() => void refresh(), POLL_INTERVAL_MS);
    return () => {
      mounted.current = false;
      window.clearInterval(interval);
    };
  }, [refresh]);

  const toggle = useCallback(async () => {
    setBusy(true);
    try {
      const current = snapshot?.active;
      const next = !current
        ? await startTask()
        : current.status === "running"
          ? await pauseCurrent()
          : await resumeCurrent();
      apply(next);
    } catch (cause) {
      setError(describeError(cause));
    } finally {
      if (mounted.current) {
        setBusy(false);
      }
    }
  }, [snapshot, apply]);

  return { snapshot, busy, error, refresh, toggle };
}
