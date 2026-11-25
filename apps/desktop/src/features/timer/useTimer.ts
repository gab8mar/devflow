import { useCallback, useEffect, useRef, useState } from "react";
import {
  getTimerSnapshot,
  pauseCurrent,
  resumeCurrent,
  startTask as requestStartTask,
  type TimerSnapshotDto,
} from "../../lib/ipc/commands";
import { describeError } from "../../lib/ipc/errors";

const POLL_INTERVAL_MS = 15_000;

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
        setError(describeError(cause, "No se pudo actualizar el cronómetro"));
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
        ? await requestStartTask()
        : current.status === "running"
          ? await pauseCurrent()
          : await resumeCurrent();
      apply(next);
    } catch (cause) {
      setError(describeError(cause, "No se pudo actualizar el cronómetro"));
    } finally {
      if (mounted.current) {
        setBusy(false);
      }
    }
  }, [snapshot, apply]);

  const start = useCallback(
    async (taskId: string) => {
      setBusy(true);
      try {
        apply(await requestStartTask(taskId));
      } catch (cause) {
        setError(describeError(cause, "No se pudo actualizar el cronómetro"));
        throw cause;
      } finally {
        if (mounted.current) {
          setBusy(false);
        }
      }
    },
    [apply],
  );

  return { snapshot, busy, error, refresh, toggle, start };
}
