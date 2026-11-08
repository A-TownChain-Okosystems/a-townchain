import { describe, expect, it } from "vitest";
import { buildGenerationPrompt, planGraphics } from "./graphicsAi";

describe("Graphics AI planner", () => {
  it("infers a deterministic 3D graphics pipeline", () => {
    const plan = planGraphics({
      prompt: "Futuristische 3D Szene mit Metall, Neonlicht und VFX",
      style: "cyberpunk",
      backend: "threejs",
    });

    expect(plan.domains).toEqual(["material", "model3d", "scene", "lighting", "vfx"]);
    expect(plan.pipeline.map((stage) => stage.id)).toEqual([
      "analyze",
      "generate-material",
      "generate-model3d",
      "generate-scene",
      "generate-lighting",
      "generate-vfx",
      "render",
    ]);
    expect(plan.safety.executableCode).toBe(false);
  });

  it("rejects empty or oversized prompts", () => {
    expect(() => planGraphics({ prompt: "   " })).toThrow(/must not be empty/);
    expect(() => planGraphics({ prompt: "x".repeat(8_193) })).toThrow(/8192/);
  });

  it("rejects invalid budgets", () => {
    expect(() => planGraphics({ prompt: "test", budget: { targetFps: 0 } })).toThrow(/budget/);
  });

  it("builds a bounded model prompt", () => {
    const plan = planGraphics({
      prompt: "Raumschiff rendern",
      domains: ["model3d", "render", "optimization"],
      budget: { maxTriangles: 100_000 },
    });
    const prompt = buildGenerationPrompt(plan);
    expect(prompt).toContain("100000 triangles");
    expect(prompt).toContain("do not execute arbitrary host commands");
  });
});
