import { describe, expect, it } from "vitest";

import type { ActiveEntryDto } from "../ipc/commands";
import { elapsedSeconds } from "./elapsed";

const T0 = 1_700_000_000_000;

function entry(overrides: Partial<ActiveEntryDto> = {}): ActiveEntryDto {
  return {
    id: "e1",
    projectId: "project-general",
    projectName: "General",
    projectCode: "GEN",
    projectColor: "#2F81F7",
    taskId: "task-general",
    taskTitle: "Trabajo general",
    status: "running",
    startedAt: T0,
    lastResumedAt: T0,
    accumulatedSeconds: 0,
    elapsedSeconds: 0,
    ...overrides,
  };
}

describe("elapsedSeconds", () => {
  it("suma el tramo en curso al tiempo acumulado", () => {
    expect(elapsedSeconds(entry(), T0 + 90_000)).toBe(90);
    expect(
      elapsedSeconds(
        entry({ accumulatedSeconds: 30, lastResumedAt: T0 + 60_000 }),
        T0 + 120_000,
      ),
    ).toBe(90);
  });

  it("congela el tiempo cuando está en pausa", () => {
    expect(
      elapsedSeconds(
        entry({
          status: "paused",
          lastResumedAt: null,
          accumulatedSeconds: 42,
          elapsedSeconds: 42,
        }),
        T0 + 600_000,
      ),
    ).toBe(42);
  });

  it("no resta tiempo si el reloj retrocede", () => {
    expect(elapsedSeconds(entry(), T0 - 5_000)).toBe(0);
  });

  it("devuelve cero sin entrada activa o con reloj inválido", () => {
    expect(elapsedSeconds(null, T0)).toBe(0);
    expect(elapsedSeconds(entry(), Number.NaN)).toBe(0);
  });
});
