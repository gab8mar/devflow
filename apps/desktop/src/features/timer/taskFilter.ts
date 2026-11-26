import type { TaskDto } from "../../lib/ipc/commands";

export interface TaskGroup {
  projectId: string;
  projectName: string;
  projectCode: string;
  projectColor: string;
  tasks: TaskDto[];
}

export function normalizeText(value: string): string {
  return value
    .normalize("NFD")
    .replace(/[\u0300-\u036f]/g, "")
    .toLowerCase()
    .trim();
}

export function filterTasks(tasks: TaskDto[], query: string): TaskDto[] {
  const needle = normalizeText(query);
  if (!needle) {
    return tasks;
  }

  return tasks.filter(
    (task) =>
      normalizeText(task.title).includes(needle) ||
      normalizeText(task.projectName).includes(needle) ||
      normalizeText(task.projectCode).includes(needle),
  );
}

export function groupTasksByProject(tasks: TaskDto[]): TaskGroup[] {
  const groups = new Map<string, TaskGroup>();

  for (const task of tasks) {
    let group = groups.get(task.projectId);
    if (!group) {
      group = {
        projectId: task.projectId,
        projectName: task.projectName,
        projectCode: task.projectCode,
        projectColor: task.projectColor,
        tasks: [],
      };
      groups.set(task.projectId, group);
    }
    group.tasks.push(task);
  }

  return [...groups.values()];
}
