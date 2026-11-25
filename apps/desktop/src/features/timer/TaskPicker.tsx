import { useState, type FormEvent } from "react";

import type { TaskDto } from "../../lib/ipc/commands";

interface TaskPickerProps {
  tasks: TaskDto[];
  activeTaskId: string | null;
  busy: boolean;
  error: string | null;
  onSelect: (taskId: string) => void;
  onCreate: (title: string) => Promise<boolean>;
  onClose: () => void;
}

export function TaskPicker({
  tasks,
  activeTaskId,
  busy,
  error,
  onSelect,
  onCreate,
  onClose,
}: TaskPickerProps) {
  const [creating, setCreating] = useState(false);
  const [title, setTitle] = useState("");
  const [submitting, setSubmitting] = useState(false);

  async function submit(event: FormEvent) {
    event.preventDefault();
    const trimmed = title.trim();
    if (!trimmed || submitting) {
      return;
    }

    setSubmitting(true);
    try {
      const created = await onCreate(trimmed);
      if (created) {
        setTitle("");
      }
    } catch {
      return;
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <section className="dt-picker" aria-label="Elegir tarea">
      <header className="dt-picker__header">
        <span>Tareas</span>
        <button
          className="dt-picker__close"
          type="button"
          onClick={onClose}
          aria-label="Cerrar selector de tareas"
          title="Cerrar"
        >
          ×
        </button>
      </header>

      <div className="dt-task-list">
        {tasks.length === 0 && !busy ? (
          <p className="dt-picker__empty">No hay tareas todavía.</p>
        ) : (
          tasks.map((task) => {
            const selected = task.id === activeTaskId;
            return (
              <button
                key={task.id}
                className={`dt-task${selected ? " dt-task--selected" : ""}`}
                type="button"
                onClick={() => onSelect(task.id)}
                disabled={busy || submitting}
              >
                <span
                  className="dt-task__dot"
                  style={{ background: task.projectColor }}
                  aria-hidden="true"
                />
                <span className="dt-task__text">
                  <span className="dt-task__title">{task.title}</span>
                  <span className="dt-task__meta">
                    {task.projectCode} · {task.isLocal ? "Local" : "PWA"}
                  </span>
                </span>
                {selected ? (
                  <span className="dt-task__check" aria-label="Tarea activa">
                    ✓
                  </span>
                ) : null}
              </button>
            );
          })
        )}
      </div>

      {error ? (
        <p className="dt-picker__error" role="alert">
          {error}
        </p>
      ) : null}

      {creating ? (
        <form className="dt-new-task" onSubmit={submit}>
          <input
            className="dt-new-task__input"
            value={title}
            onChange={(event) => setTitle(event.target.value)}
            placeholder="Nombre de la tarea"
            maxLength={200}
            autoFocus
            disabled={submitting}
          />
          <button
            className="dt-new-task__submit"
            type="submit"
            disabled={!title.trim() || submitting}
          >
            Crear
          </button>
          <button
            className="dt-new-task__cancel"
            type="button"
            onClick={() => setCreating(false)}
            disabled={submitting}
          >
            Cancelar
          </button>
        </form>
      ) : (
        <button
          className="dt-new-task__open"
          type="button"
          onClick={() => setCreating(true)}
          disabled={busy}
        >
          + Nueva tarea local
        </button>
      )}
    </section>
  );
}
