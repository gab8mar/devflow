import { describe, expect, it } from "vitest";

import type { TaskDto } from "../../lib/ipc/commands";
import { filterTasks, groupTasksByProject, normalizeText } from "./taskFilter";

function task(overrides: Partial<TaskDto> = {}): TaskDto {
  return {
    id: "t1",
    projectId: "project-general",
    projectName: "General",
    projectCode: "GEN",
    projectColor: "#2F81F7",
    title: "Trabajo general",
    externalId: null,
    lastUsedAt: null,
    isLocal: true,
    ...overrides,
  };
}

describe("normalizeText", () => {
  it("quita acentos y pasa a minúsculas", () => {
    expect(normalizeText("  Revisión API ")).toBe("revision api");
  });
});

describe("filterTasks", () => {
  it("devuelve todas las tareas con consulta vacía", () => {
    const tasks = [task(), task({ id: "t2", title: "Otra" })];
    expect(filterTasks(tasks, "   ")).toHaveLength(2);
  });

  it("busca por título sin distinguir mayúsculas", () => {
    const tasks = [task(), task({ id: "t2", title: "Deploy" })];
    expect(filterTasks(tasks, "DEP").map((item) => item.id)).toEqual(["t2"]);
  });

  it("ignora acentos en la consulta", () => {
    const tasks = [task({ id: "t2", title: "Revisión de contrato" })];
    expect(filterTasks(tasks, "revision")).toHaveLength(1);
  });

  it("busca por nombre y código de proyecto", () => {
    const tasks = [
      task(),
      task({
        id: "t2",
        projectId: "project-activa",
        projectName: "Activa",
        projectCode: "ACT",
        title: "Tarea",
      }),
    ];
    expect(filterTasks(tasks, "activa").map((item) => item.id)).toEqual(["t2"]);
    expect(filterTasks(tasks, "act").map((item) => item.id)).toEqual(["t2"]);
  });

  it("devuelve vacío cuando nada coincide", () => {
    expect(filterTasks([task()], "zzzz")).toEqual([]);
  });
});

describe("groupTasksByProject", () => {
  it("agrupa por proyecto conservando el orden de llegada", () => {
    const tasks = [
      task({ id: "t1" }),
      task({
        id: "t2",
        projectId: "project-activa",
        projectName: "Activa",
        projectCode: "ACT",
        title: "Tarea A",
      }),
      task({ id: "t3", title: "Tarea B" }),
    ];

    const groups = groupTasksByProject(tasks);

    expect(groups).toHaveLength(2);
    expect(groups[0].projectCode).toBe("GEN");
    expect(groups[0].tasks.map((item) => item.id)).toEqual(["t1", "t3"]);
    expect(groups[1].projectCode).toBe("ACT");
    expect(groups[1].tasks.map((item) => item.id)).toEqual(["t2"]);
  });

  it("devuelve lista vacía sin tareas", () => {
    expect(groupTasksByProject([])).toEqual([]);
  });
});
