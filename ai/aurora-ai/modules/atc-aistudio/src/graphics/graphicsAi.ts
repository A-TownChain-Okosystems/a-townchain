// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems.
// Apache-2.0

export type GraphicsDomain =
  | "image" | "concept" | "texture" | "material" | "mesh" | "model3d"
  | "scene" | "lighting" | "shader" | "vfx" | "animation" | "render"
  | "ui" | "optimization";

export type GraphicsBackend = "image-model" | "threejs" | "webgl" | "webgpu" | "external";

export interface GraphicsBudget {
  maxTriangles: number;
  maxTexturePixels: number;
  maxDrawCalls: number;
  targetFps: number;
  maxVramMb: number;
}

export interface GraphicsRequest {
  prompt: string;
  domains?: GraphicsDomain[];
  style?: string;
  aspectRatio?: "1:1" | "4:3" | "16:9" | "9:16";
  backend?: GraphicsBackend;
  budget?: Partial<GraphicsBudget>;
}

export interface GraphicsPlan {
  normalizedPrompt: string;
  domains: GraphicsDomain[];
  style: string;
  aspectRatio: NonNullable<GraphicsRequest["aspectRatio"]>;
  backend: GraphicsBackend;
  budget: GraphicsBudget;
  pipeline: GraphicsStage[];
  safety: {
    executableCode: false;
    externalNetworkRequired: boolean;
  };
}

export interface GraphicsStage {
  id: string;
  domain: GraphicsDomain;
  action: "analyze" | "generate" | "compile" | "optimize" | "render";
  inputs: string[];
  outputs: string[];
}

const DEFAULT_BUDGET: GraphicsBudget = {
  maxTriangles: 250_000,
  maxTexturePixels: 67_108_864,
  maxDrawCalls: 2_000,
  targetFps: 60,
  maxVramMb: 2_048,
};

const KEYWORDS: ReadonlyArray<[GraphicsDomain, RegExp]> = [
  ["concept", /concept|konzept|skizze|design/i],
  ["texture", /texture|textur|pbr|albedo|normal map|roughness/i],
  ["material", /material|metall|glas|holz|stein/i],
  ["mesh", /mesh|polygon|retopology|topologie/i],
  ["model3d", /3d|objekt|asset/i],
  ["scene", /szene|scene|level|umgebung|environment/i],
  ["lighting", /licht|lighting|beleuchtung|shadow|schatten|illumination/i],
  ["shader", /shader|glsl|hlsl|wgsl|material graph/i],
  ["vfx", /vfx|partikel|particle|explosion|rauch|feuer|fog|nebel/i],
  ["animation", /animation|rig|skelett|bewegung|motion/i],
  ["render", /render|ray tracing|path tracing|denoise|upscale/i],
  ["ui", /ui|hud|interface|icon|menü|menu/i],
  ["optimization", /optimier|optimi|performance|fps|vram|draw call|lod/i],
  ["image", /bild|image|illustration|foto|portrait|poster/i],
];

function mergeBudget(input?: Partial<GraphicsBudget>): GraphicsBudget {
  const budget = { ...DEFAULT_BUDGET, ...(input ?? {}) };
  for (const [key, value] of Object.entries(budget)) {
    if (!Number.isFinite(value) || value <= 0) {
      throw new Error(`Invalid graphics budget: ${key}`);
    }
  }
  if (budget.targetFps > 240) throw new Error("targetFps must be <= 240");
  return budget;
}

function inferDomains(prompt: string): GraphicsDomain[] {
  const domains = KEYWORDS.filter(([, pattern]) => pattern.test(prompt)).map(([domain]) => domain);
  return domains.length ? [...new Set(domains)] : ["image"];
}

function normalizePrompt(prompt: string): string {
  const normalized = prompt.trim().replace(/\s+/g, " ");
  if (!normalized) throw new Error("Graphics prompt must not be empty");
  if (normalized.length > 8_192) throw new Error("Graphics prompt exceeds 8192 characters");
  return normalized;
}

function buildPipeline(domains: GraphicsDomain[], backend: GraphicsBackend): GraphicsStage[] {
  const stages: GraphicsStage[] = [
    { id: "analyze", domain: domains[0], action: "analyze", inputs: ["prompt"], outputs: ["graphics-intent"] },
  ];

  const generationDomains = domains.filter((domain) => domain !== "optimization" && domain !== "render");
  for (const domain of generationDomains) {
    stages.push({
      id: `generate-${domain}`,
      domain,
      action: domain === "shader" ? "compile" : "generate",
      inputs: ["graphics-intent"],
      outputs: [`${domain}-asset`],
    });
  }

  if (domains.includes("optimization")) {
    stages.push({
      id: "optimize-scene",
      domain: "optimization",
      action: "optimize",
      inputs: generationDomains.map((domain) => `${domain}-asset`),
      outputs: ["optimized-scene"],
    });
  }

  if (domains.includes("render") || backend === "threejs" || backend === "webgl" || backend === "webgpu") {
    stages.push({
      id: "render",
      domain: "render",
      action: "render",
      inputs: [domains.includes("optimization") ? "optimized-scene" : "graphics-intent"],
      outputs: ["frame"],
    });
  }

  return stages;
}

export function planGraphics(request: GraphicsRequest): GraphicsPlan {
  const normalizedPrompt = normalizePrompt(request.prompt);
  const domains = request.domains?.length ? [...new Set(request.domains)] : inferDomains(normalizedPrompt);
  const backend = request.backend ?? (domains.includes("image") && domains.length === 1 ? "image-model" : "threejs");

  return {
    normalizedPrompt,
    domains,
    style: request.style?.trim() || "adaptive",
    aspectRatio: request.aspectRatio ?? "16:9",
    backend,
    budget: mergeBudget(request.budget),
    pipeline: buildPipeline(domains, backend),
    safety: {
      executableCode: false,
      externalNetworkRequired: backend === "image-model" || backend === "external",
    },
  };
}

export function buildGenerationPrompt(plan: GraphicsPlan): string {
  const stages = plan.pipeline.map((stage) => stage.id).join(" -> ");
  return [
    "You are the Aurora Graphics AI planner.",
    `Prompt: ${plan.normalizedPrompt}`,
    `Style: ${plan.style}`,
    `Aspect ratio: ${plan.aspectRatio}`,
    `Domains: ${plan.domains.join(", ")}`,
    `Pipeline: ${stages}`,
    `Budget: ${plan.budget.maxTriangles} triangles, ${plan.budget.maxDrawCalls} draw calls, ${plan.budget.maxVramMb} MiB VRAM, ${plan.budget.targetFps} FPS.`,
    "Return only the requested graphics artifact or structured graphics output; do not execute arbitrary host commands.",
  ].join("\n");
}
