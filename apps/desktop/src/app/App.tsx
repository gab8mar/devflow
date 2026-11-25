import { useState } from "react";

import { TimerPill } from "../features/timer/TimerPill";
import { useTasks } from "../features/timer/useTasks";
import { useTimer } from "../features/timer/useTimer";

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

  async function selectTask(taskId: string) {
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

  async function createTask(title: string): Promise<boolean> {
    try {
      const task = await create(title);
      await start(task.id);
    } catch {
      return false;
    }
    setPickerOpen(false);
    return true;
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
      onSelectTask={selectTask}
      onCreateTask={createTask}
    />
  );
}
