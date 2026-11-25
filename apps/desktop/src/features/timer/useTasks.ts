import { useCallback, useEffect, useRef, useState } from "react";

import {
  createTask,
  listTasks,
  type TaskDto,
} from "../../lib/ipc/commands";
import { describeError } from "../../lib/ipc/errors";

export function useTasks() {
  const [tasks, setTasks] = useState<TaskDto[]>([]);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const mounted = useRef(true);

  useEffect(() => {
    return () => {
      mounted.current = false;
    };
  }, []);

  const refresh = useCallback(async () => {
    setBusy(true);
    try {
      const next = await listTasks();
      if (mounted.current) {
        setTasks(next);
        setError(null);
      }
    } catch (cause) {
      if (mounted.current) {
        setError(describeError(cause, "No se pudieron cargar las tareas"));
      }
    } finally {
      if (mounted.current) {
        setBusy(false);
      }
    }
  }, []);

  const create = useCallback(async (title: string): Promise<TaskDto> => {
    setBusy(true);
    try {
      const created = await createTask(title);
      if (mounted.current) {
        setTasks((current) => [
          created,
          ...current.filter((task) => task.id !== created.id),
        ]);
        setError(null);
      }
      return created;
    } catch (cause) {
      if (mounted.current) {
        setError(describeError(cause, "No se pudo crear la tarea"));
      }
      throw cause;
    } finally {
      if (mounted.current) {
        setBusy(false);
      }
    }
  }, []);

  return { tasks, busy, error, refresh, create };
}
