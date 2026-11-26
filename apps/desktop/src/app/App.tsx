import { useState } from "react";

import { TimerPill } from "../features/timer/TimerPill";
import { useTasks } from "../features/timer/useTasks";
import { useTimer } from "../features/timer/useTimer";
import type { TaskDto } from "../lib/ipc/commands";

export function App() {
  const [pickerOpen, setPickerOpen] = useState(false);
  const { snapshot, busy, error, toggle, start } = useTimer();
  const {
    tasks,
    busy: tasksBusy,
    error: tasksError,
    refresh,
    create,
  } = useTasks();

  function openPicker() {
    setPickerOpen(true);
    void refresh();
  }

  async function applyTask(taskId: string) {
    if (taskId === snapshot?.active?.taskId) {
      setPickerOpen(false);
      return;
    }

    try {
      await start(taskId);
    } catch {
      return;
    }
    setPickerOpen(false);
  }

  async function createTask(title: string): Promise<TaskDto | null> {
    try {
      return await create(title);
    } catch {
      return null;
    }
  }

  return (
    <TimerPill
      snapshot={snapshot}
      busy={busy}
      error={error}
      pickerOpen={pickerOpen}
      tasks={tasks}
      tasksBusy={tasksBusy}
      tasksError={tasksError}
      onToggle={toggle}
      onOpenPicker={openPicker}
      onClosePicker={() => setPickerOpen(false)}
      onApplyTask={applyTask}
      onCreateTask={createTask}
    />
  );
}
