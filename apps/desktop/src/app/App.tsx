import { TimerPill } from "../features/timer/TimerPill";
import { useTimer } from "../features/timer/useTimer";

export function App() {
  const { snapshot, busy, error, toggle } = useTimer();

  return (
    <TimerPill
      snapshot={snapshot}
      busy={busy}
      error={error}
      onToggle={toggle}
    />
  );
}
