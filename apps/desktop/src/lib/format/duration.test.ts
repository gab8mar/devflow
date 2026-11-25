import { describe, expect, it } from "vitest";

import { formatDuration } from "./duration";

describe("formatDuration", () => {
  it("formatea ceros y valores de un dígito", () => {
    expect(formatDuration(0)).toBe("00:00:00");
    expect(formatDuration(59)).toBe("00:00:59");
    expect(formatDuration(61)).toBe("00:01:01");
  });

  it("formatea horas y mantiene horas de más de un día", () => {
    expect(formatDuration(3661)).toBe("01:01:01");
    expect(formatDuration(90_061)).toBe("25:01:01");
  });

  it("trunca fracciones de segundo", () => {
    expect(formatDuration(59.9)).toBe("00:00:59");
  });

  it("neutraliza valores inválidos o negativos", () => {
    expect(formatDuration(-5)).toBe("00:00:00");
    expect(formatDuration(Number.NaN)).toBe("00:00:00");
    expect(formatDuration(Number.POSITIVE_INFINITY)).toBe("00:00:00");
  });
});
