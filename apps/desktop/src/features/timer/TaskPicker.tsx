import {
  useMemo,
  useState,
  type FormEvent,
  type KeyboardEvent,
} from "react";

import type { TaskDto } from "../../lib/ipc/commands";
import { filterTasks, groupTasksByProject } from "./taskFilter";

interface TaskPickerProps {
  tasks: TaskDto[];
  activeTaskId: string | null;
  busy: boolean;
  error: string | null;
  onApply: (taskId: string) => void;
  onCreate: (title: string) => Promise<TaskDto | null>;
  onClose: () => void;
}

export function TaskPicker({
  tasks,
  activeTaskId,
  busy,
  error,
  onApply,
  onCreate,
  onClose,
}: TaskPickerProps) {
  const [query, setQuery] = useState("");
  const [creating, setCreating] = useState(false);
  const [title, setTitle] = useState("");
  const [submitting, setSubmitting] = useState(false);
  const [chosenId, setChosenId] = useState<string | null>(null);

  const visible = useMemo(() => filterTasks(tasks, query), [tasks, query]);
  const groups = useMemo(() => groupTasksByProject(visible), [visible]);

  const highlightedId = useMemo(() => {
    if (chosenId && visible.some((task) => task.id === chosenId)) {
      return chosenId;
    }
    if (activeTaskId && visible.some((task) => task.id === activeTaskId)) {
      return activeTaskId;
    }
    return visible[0]?.id ?? null;
  }, [chosenId, visible, activeTaskId]);

  function moveHighlight(delta: number) {
    if (visible.length === 0) {
      return;
    }
    const index = visible.findIndex((task) => task.id === highlightedId);
    const next =
      index < 0 ? 0 : (index + delta + visible.length) % visible.length;
    setChosenId(visible[next].id);
  }

  function handleKeyDown(event: KeyboardEvent<HTMLElement>) {
    if (event.key === "Escape") {
      event.preventDefault();
      if (creating) {
        setCreating(false);
        setTitle("");
        return;
      }
      onClose();
      return;
    }

    const target = event.target as HTMLElement;
    if (target.classList.contains("dt-new-task__input")) {
      return;
    }

    if (event.key === "ArrowDown") {
      event.preventDefault();
      moveHighlight(1);
      return;
    }

    if (event.key === "ArrowUp") {
      event.preventDefault();
      moveHighlight(-1);
      return;
    }

    if (event.key === "Enter") {
      event.preventDefault();
      if (highlightedId) {
        onApply(highlightedId);
      }
    }
  }

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
        setChosenId(created.id);
        setTitle("");
        setCreating(false);
      }
    } catch {
      return;
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <section
      className="dt-picker"
      aria-label="Elegir tarea"
      onKeyDown={handleKeyDown}
    >
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

      <input
        className="dt-picker__search"
        type="search"
        value={query}
        onChange={(event) => setQuery(event.target.value)}
        placeholder="Buscar tarea, proyecto o código"
        aria-label="Buscar tarea"
        spellCheck={false}
        autoFocus
      />

      <div className="dt-task-list" role="listbox" aria-label="Tareas disponibles">
        {visible.length === 0 && !busy ? (
          <p className="dt-picker__empty">
            {query.trim() ? "Sin coincidencias." : "No hay tareas todavía."}
          </p>
        ) : (
          groups.map((group) => (
            <div className="dt-task-group" key={group.projectId}>
              <p className="dt-task-group__title">
                <span
                  className="dt-task-group__dot"
                  style={{ background: group.projectColor }}
                  aria-hidden="true"
                />
                {group.projectCode} · {group.projectName}
              </p>
              {group.tasks.map((task) => {
                const isActive = task.id === activeTaskId;
                const isHighlighted = task.id === highlightedId;
                return (
                  <button
                    key={task.id}
                    className={`dt-task${isActive ? " dt-task--selected" : ""}${
                      isHighlighted ? " dt-task--highlight" : ""
                    }`}
                    type="button"
                    role="option"
                    aria-selected={isHighlighted}
                    onClick={() => setChosenId(task.id)}
                    onDoubleClick={() => onApply(task.id)}
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
                        {task.isLocal ? "Local" : task.externalId ?? "PWA"}
                      </span>
                    </span>
                    {isActive ? (
                      <span className="dt-task__check" aria-label="Tarea activa">
                        ✓
                      </span>
                    ) : null}
                  </button>
                );
              })}
            </div>
          ))
        )}
      </div>

      {error ? (
        <p className="dt-picker__error" role="alert">
          {error}
        </p>
      ) : null}

      <div className="dt-picker__actions">
        {creating ? (
          <form className="dt-new-task" onSubmit={submit}>
            <input
              className="dt-new-task__input"
              value={title}
              onChange={(event) => setTitle(event.target.value)}
              placeholder="Nombre de la tarea"
              maxLength={200}
              aria-label="Nombre de la nueva tarea"
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
              onClick={() => {
                setCreating(false);
                setTitle("");
              }}
              disabled={submitting}
            >
              Cancelar
            </button>
          </form>
        ) : (
          <>
            <button
              className="dt-new-task__open"
              type="button"
              onClick={() => setCreating(true)}
              disabled={busy}
            >
              + Nueva tarea local
            </button>
            <button
              className="dt-picker__apply"
              type="button"
              onClick={() => {
                if (highlightedId) {
                  onApply(highlightedId);
                }
              }}
              disabled={!highlightedId || busy || submitting}
            >
              Guardar y cambiar
            </button>
          </>
        )}
      </div>
    </section>
  );
}
