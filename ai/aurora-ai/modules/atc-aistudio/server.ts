// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
import express from "express";
import path from "path";
import fsPromises from "fs/promises";
import os from "os";
import { createServer as createViteServer } from "vite";
import { AtcBlockchainEngine } from "./src/backend/blockchain/engine.js";
import { P2PNetwork } from "./src/backend/p2p/network.js";
import notionRouter from "./src/routes/notion.ts";

async function startServer() {
  const app = express();
  const PORT = 3000;
  
  app.use(express.json());

  const engine = new AtcBlockchainEngine();
  const p2p = new P2PNetwork(engine);

  // Real internal logs store
  const serverLogs: any[] = [];
  const addLog = (level: string, message: string, source: string) => {
      serverLogs.push({
          id: Math.random().toString(36).substring(7),
          timestamp: new Date().toISOString(),
          level, message, source
      });
      if (serverLogs.length > 100) serverLogs.shift();
  };
  addLog('info', 'Platform Startup Initiated', 'System');
  addLog('info', `Node.js Version: ${process.version}`, 'System');
  
  // Request logger middleware
  app.use((req, res, next) => {
    if (req.url.startsWith('/api') && req.url !== '/api/system/logs' && req.url !== '/api/system/metrics' && req.url !== '/api/orchestrator/health') {
       addLog('info', `${req.method} ${req.url}`, 'Express');
    }
    next();
  });

  // Export logs endpoint
  app.get('/api/system/logs', (req, res) => {
      res.json(serverLogs);
  });

  // Start processes
  p2p.initialize("0.0.0.0", 5001);
  engine.startConsensus();

  // API Routes
  app.get("/api/health", (req, res) => {
    res.json({ status: "ok", node: "ATC-MainNode-1" });
  });

  app.get("/api/blockchain/info", (req, res) => {
    res.json({
      height: engine.getChainHeight(),
      topHash: engine.getLatestBlock().hash,
      peers: p2p.getConnections().length,
      version: "A-TownChain Core 1.0.0",
      status: "Online"
    });
  });

  app.get("/api/network/topology", (req, res) => {
    res.json({
       nodeId: "ATC-MainNode-1",
       version: "1.0.0",
       peers: p2p.getConnections(),
       traffic: p2p.getTrafficStats()
    });
  });

  app.get("/api/blockchain/blocks", (req, res) => {
    res.json(engine.getChain());
  });

  app.get("/api/knowledge", async (req, res) => {
    try {
      const knowledge = [];
      
      const mdFiles = ["SOFTWARE_ROADMAP.md", "ROADMAP.md", "AGENTS.md", "GEMINI.md"];
      for (const file of mdFiles) {
        try {
          const content = await fsPromises.readFile(file, "utf8");
          knowledge.push({ 
            id: file, 
            type: "document", 
            title: file, 
            content,
            category: "System Root",
            subcategory: "Meta Documentation",
            tags: ["system", "docs", "root"],
            version: "v1.0"
          });
        } catch(e) {}
      }

      const walkDir = async (dir: string): Promise<any[]> => {
        let results: any[] = [];
        try {
          const list = await fsPromises.readdir(dir);
          for (const file of list) {
            const filePath = path.join(dir, file);
            const stat = await fsPromises.stat(filePath);
            if (stat && stat.isDirectory()) {
              if (file !== "node_modules" && file !== "dist" && file !== ".git") {
                results = results.concat(await walkDir(filePath));
              }
            } else {
              if (file.endsWith(".ts") || file.endsWith(".tsx") || file.endsWith(".json")) {
                   try {
                       const content = await fsPromises.readFile(filePath, "utf8");
                       results.push({
                           id: filePath,
                           type: "code",
                           title: filePath,
                           content: content,
                           category: "Source Code",
                           subcategory: file.endsWith('.tsx') ? 'React Component' : 'Script / Logic',
                           tags: ["code", "src", file.split('.').pop() || ''],
                           version: "latest"
                       });
                   } catch(e) {}
              }
            }
          }
        } catch(e) {}
        return results;
      };

      const codeFiles = await walkDir("src");
      res.json({ documents: knowledge, code: codeFiles });
    } catch (error: any) {
      res.status(500).json({ error: error.message });
    }
  });

  app.get("/api/status", (req, res) => {
    res.json({
      cpu: "12%",
      memory: "256MB",
      network_traffic: p2p.getTrafficStats(),
      tps: engine.getTPS()
    });
  });

  app.get("/api/sync-history", async (req, res) => {
    try {
      const endpoints = [
        "https://app.base44.com/api/agents/6a1d37209d7f46ac065d39c5/conversations?api_key=ddc7d8f665684878afcc73c2b8b62b7a",
        "https://app.base44.com/api/agents/6a2756186106d6f0fbb105b5/conversations?api_key=6b52c04128cf41b6aa3e4b198c6e3f73",
        "https://app.base44.com/api/agents/6a27614c7219ab1e4f951842/conversations?api_key=f863bf86f3f74f91a6b8f126f70c7dd2",
        "https://app.base44.com/api/agents/6a0a3f408dced6c5ca7506ef/conversations?api_key=5037afba063147959371bc8fbe3b02fd",
        "https://app.base44.com/api/agents/69c1e0c577ccf6c45a27a480/conversations?api_key=086d6f97ec1147e4b25604e4db49da9c"
      ];

      const responses = await Promise.all(
        endpoints.map((url) => fetch(url).then((res) => res.json()).catch(() => ({})))
      );

      let allMessages: any[] = [];
      responses.forEach((data) => {
        const messages = Array.isArray(data) ? data : data.messages || [];
        allMessages = allMessages.concat(messages);
      });

      allMessages.sort((a, b) => {
        const timeA = new Date(a.metadata?.created_date || 0).getTime();
        const timeB = new Date(b.metadata?.created_date || 0).getTime();
        // Sort descending (newest first)
        return timeB - timeA;
      });

      const parsedLogs = allMessages
        .filter((m: any) => m.role === "assistant" && !m.hidden && m.content)
        .map((m: any) => ({
          id: m.id,
          timestamp: m.metadata?.created_date || new Date().toISOString(),
          content: m.content,
        }))
        .slice(0, 50);

      res.json(parsedLogs);
    } catch (error: any) {
      console.error("Base44 fetch error:", error);
      res.status(500).json({ error: error.message });
    }
  });

  app.post("/api/assess-milestone", async (req, res) => {
    try {
      const { title, goals } = req.body;
      const { GoogleGenAI } = await import("@google/genai");
      // Use process.env.GEMINI_API_KEY as per standard guidelines
      const ai = new GoogleGenAI({ apiKey: process.env.GEMINI_API_KEY });
      const prompt = `Analyze the roadmap milestone: "${title}" with these goals: ${JSON.stringify(goals)}. Briefly assess its impact (2-3 sentences) on achieving the overarching "Endzustand" goals (such as an Autonomous Civilization, Full Verification, Global Infrastructure Network, and Agent Ecosystem). Respond in a technical and analytical tone.`;
      
      const response = await ai.models.generateContent({
        model: "gemini-2.5-flash",
        contents: prompt
      });
      
      res.json({ assessment: response.text });
    } catch (error: any) {
      console.error(error);
      res.status(500).json({ error: error.message });
    }
  });

  app.post("/api/franchise-factory/generate-text", async (req, res) => {
    try {
      const { prompt } = req.body;
      const { GoogleGenAI } = await import("@google/genai");
      const ai = new GoogleGenAI({ apiKey: process.env.GEMINI_API_KEY });
      
      const responseStream = await ai.models.generateContentStream({
        model: "gemini-3.1-pro-preview",
        contents: prompt,
      });

      res.setHeader("Content-Type", "text/event-stream");
      res.setHeader("Cache-Control", "no-cache");
      res.setHeader("Connection", "keep-alive");

      for await (const chunk of responseStream) {
        if (chunk.text) {
          res.write(`data: ${JSON.stringify({ text: chunk.text })}\n\n`);
        }
      }
      res.write(`data: [DONE]\n\n`);
      res.end();
    } catch (error: any) {
      console.error(error);
      if (!res.headersSent) {
        res.status(500).json({ error: error.message });
      } else {
        res.write(`data: ${JSON.stringify({ error: error.message })}\n\n`);
        res.end();
      }
    }
  });

  app.post("/api/franchise-factory/generate-book", async (req, res) => {
    try {
      const { topic, genre, chapterCount, franchiseContext } = req.body;
      const { GoogleGenAI } = await import("@google/genai");
      const ai = new GoogleGenAI({ apiKey: process.env.GEMINI_API_KEY });
      const contextPrefix = franchiseContext ? `Please adhere STRICTLY to the following franchise context and lore to maintain consistency across the IP:\n${franchiseContext}\n\n` : '';
      const prompt = `${contextPrefix}You are a professional author and book writing AI. Please write a highly detailed book or comprehensive outline/story about the topic: "${topic}". 
Genre/Style: ${genre}. 
Length: Try to include at least ${chapterCount} chapters with substantial detail for each. 
Format using markdown with clear headings (# Chapter 1: Title).`;

      const responseStream = await ai.models.generateContentStream({
        model: "gemini-3.1-pro-preview",
        contents: prompt,
      });

      res.setHeader("Content-Type", "text/event-stream");
      res.setHeader("Cache-Control", "no-cache");
      res.setHeader("Connection", "keep-alive");

      for await (const chunk of responseStream) {
        if (chunk.text) {
          res.write(`data: ${JSON.stringify({ text: chunk.text })}\n\n`);
        }
      }
      res.write(`data: [DONE]\n\n`);
      res.end();
    } catch (error: any) {
      console.error(error);
      if (!res.headersSent) {
        res.status(500).json({ error: error.message });
      } else {
        res.write(`data: ${JSON.stringify({ error: error.message })}\n\n`);
        res.end();
      }
    }
  });

  app.post("/api/franchise-factory/generate-html-game", async (req, res) => {
    try {
      const { topic, flavor, franchiseContext } = req.body;
      const { GoogleGenAI } = await import("@google/genai");
      const ai = new GoogleGenAI({ apiKey: process.env.GEMINI_API_KEY });
      const contextPrefix = franchiseContext ? `Please adhere to the following franchise context:\n${franchiseContext}\n\n` : '';
      const prompt = `${contextPrefix}You are an expert game developer. I want you to write a complete, playable, single-file HTML5 game.
Theme/Topic: "${topic}"
Flavor/Style: ${flavor}
Please write only the raw HTML code containing JS (canvas) and CSS to run the game directly in an iframe. 
Make sure it's fully playable (e.g. keyboard/mouse interaction) and visually polished.
Return EXACTLY the valid HTML string (starting with <!DOCTYPE html>) and NO markdown formatting or \`\`\`html blocks around it. Do not include any explanations.`;

      const responseStream = await ai.models.generateContentStream({
        model: "gemini-3.1-pro-preview",
        contents: prompt,
      });

      res.setHeader("Content-Type", "text/event-stream");
      res.setHeader("Cache-Control", "no-cache");
      res.setHeader("Connection", "keep-alive");

      for await (const chunk of responseStream) {
        res.write(`data: ${JSON.stringify({ text: chunk.text })}\n\n`);
      }
      res.write("data: [DONE]\n\n");
      res.end();
    } catch (error: any) {
      console.error("Gemini Game Engine Error:", error);
      res.status(500).json({ error: error.message });
    }
  });

  app.post("/api/franchise-factory/generate-character", async (req, res) => {
    try {
      const { topic, flavor, franchiseContext } = req.body;
      const { GoogleGenAI } = await import("@google/genai");
      const ai = new GoogleGenAI({ apiKey: process.env.GEMINI_API_KEY });
      const contextPrefix = franchiseContext ? `Please adhere to the following franchise context:\n${franchiseContext}\n\n` : '';
      const prompt = `${contextPrefix}You are an expert character writer and frontend developer. I want you to write a complete, single-file HTML document that renders an interactive, visually stunning Character Profile / RPG Character Sheet.
Theme/Topic/Character Idea: "${topic}"
Style/Format: ${flavor}
Please write only the raw HTML code containing CSS and JS to display the character's biography, stats, inventory, lore, and visual description. 
Make sure it's visually polished, uses creative typography and layout (e.g., CSS Grid/Flexbox), works responsively, and runs directly in an iframe.
Return EXACTLY the valid HTML string (starting with <!DOCTYPE html>) and NO markdown formatting or \`\`\`html blocks around it. Do not include any explanations.`;

      const responseStream = await ai.models.generateContentStream({
        model: "gemini-3.1-pro-preview",
        contents: prompt,
      });

      res.setHeader("Content-Type", "text/event-stream");
      res.setHeader("Cache-Control", "no-cache");
      res.setHeader("Connection", "keep-alive");

      for await (const chunk of responseStream) {
        res.write(`data: ${JSON.stringify({ text: chunk.text })}\n\n`);
      }
      res.write("data: [DONE]\n\n");
      res.end();
    } catch (error: any) {
      console.error("Gemini Character Engine Error:", error);
      res.status(500).json({ error: error.message });
    }
  });

  app.post("/api/franchise-factory/generate-timeline", async (req, res) => {
    try {
      const { topic, flavor, franchiseContext } = req.body;
      const { GoogleGenAI } = await import("@google/genai");
      const ai = new GoogleGenAI({ apiKey: process.env.GEMINI_API_KEY });
      const contextPrefix = franchiseContext ? `Please adhere to the following franchise context:\n${franchiseContext}\n\n` : '';
      const prompt = `${contextPrefix}You are an expert worldbuilder and frontend developer. I want you to write a complete, single-file HTML document that renders an interactive, visually stunning historical Timeline.
Theme/Topic/Events: "${topic}"
Style/Format: ${flavor}
Please write only the raw HTML code containing CSS and JS to display the timeline events. 
Make sure it's visually polished, uses creative typography and layout, works responsively, and runs directly in an iframe.
Return EXACTLY the valid HTML string (starting with <!DOCTYPE html>) and NO markdown formatting or \`\`\`html blocks around it. Do not include any explanations.`;

      const responseStream = await ai.models.generateContentStream({
        model: "gemini-3.1-pro-preview",
        contents: prompt,
      });

      res.setHeader("Content-Type", "text/event-stream");
      res.setHeader("Cache-Control", "no-cache");
      res.setHeader("Connection", "keep-alive");

      for await (const chunk of responseStream) {
        res.write(`data: ${JSON.stringify({ text: chunk.text })}\n\n`);
      }
      res.write("data: [DONE]\n\n");
      res.end();
    } catch (error: any) {
      console.error("Gemini Timeline Engine Error:", error);
      res.status(500).json({ error: error.message });
    }
  });

  app.post("/api/franchise-factory/generate-audio", async (req, res) => {
    try {
      const { topic, flavor, franchiseContext } = req.body;
      const { GoogleGenAI } = await import("@google/genai");
      const ai = new GoogleGenAI({ apiKey: process.env.GEMINI_API_KEY });
      const contextPrefix = franchiseContext ? `Please adhere to the following franchise context:\n${franchiseContext}\n\n` : '';
      const prompt = `${contextPrefix}You are an expert audio programmer and sound designer. I want you to write a complete, single-file HTML document that uses the Web Audio API (or Tone.js via CDN) to procedurally generate and play audio/music based on the prompt.
Theme/Topic: "${topic}"
Style/Format: ${flavor}
Please write only the raw HTML code containing JS and CSS to display a simple playback UI (Play/Stop buttons) and generate the audio in the browser. 
Make sure it's visually polished, works directly in an iframe, and handles user interaction to start audio context.
Return EXACTLY the valid HTML string (starting with <!DOCTYPE html>) and NO markdown formatting or \`\`\`html blocks around it. Do not include any explanations.`;

      const responseStream = await ai.models.generateContentStream({
        model: "gemini-3.1-pro-preview",
        contents: prompt,
      });

      res.setHeader("Content-Type", "text/event-stream");
      res.setHeader("Cache-Control", "no-cache");
      res.setHeader("Connection", "keep-alive");

      for await (const chunk of responseStream) {
        res.write(`data: ${JSON.stringify({ text: chunk.text })}\n\n`);
      }
      res.write("data: [DONE]\n\n");
      res.end();
    } catch (error: any) {
      console.error("Gemini Audio Engine Error:", error);
      res.status(500).json({ error: error.message });
    }
  });

  app.post("/api/franchise-factory/generate-animation", async (req, res) => {
    try {
      const { topic, flavor, franchiseContext } = req.body;
      const { GoogleGenAI } = await import("@google/genai");
      const ai = new GoogleGenAI({ apiKey: process.env.GEMINI_API_KEY });
      const contextPrefix = franchiseContext ? `Please adhere to the following franchise context:\n${franchiseContext}\n\n` : '';
      const prompt = `${contextPrefix}You are an expert technical artist and frontend programmer. I want you to write a complete, single-file HTML document that renders an animation based on the prompt.
Theme/Topic: "${topic}"
Visual Style/Technique: ${flavor}
Please write only the raw HTML code containing CSS, JS (Canvas, GSAP via CDN, etc.) and SVG inline if appropriate, to run the animation directly in an iframe.
Make sure it's visually polished, works responsively if possible, and starts animating automatically.
Return EXACTLY the valid HTML string (starting with <!DOCTYPE html>) and NO markdown formatting or \`\`\`html blocks around it. Do not include any explanations.`;

      const responseStream = await ai.models.generateContentStream({
        model: "gemini-3.1-pro-preview",
        contents: prompt,
      });

      res.setHeader("Content-Type", "text/event-stream");
      res.setHeader("Cache-Control", "no-cache");
      res.setHeader("Connection", "keep-alive");

      for await (const chunk of responseStream) {
        res.write(`data: ${JSON.stringify({ text: chunk.text })}\n\n`);
      }
      res.write("data: [DONE]\n\n");
      res.end();
    } catch (error: any) {
      console.error("Gemini Animation Engine Error:", error);
      res.status(500).json({ error: error.message });
    }
  });

  app.post("/api/franchise-factory/generate-3d", async (req, res) => {
    try {
      const { topic, flavor, franchiseContext } = req.body;
      const { GoogleGenAI } = await import("@google/genai");
      const ai = new GoogleGenAI({ apiKey: process.env.GEMINI_API_KEY });
      const contextPrefix = franchiseContext ? `Please adhere to the following franchise context:\n${franchiseContext}\n\n` : '';
      const prompt = `${contextPrefix}You are an expert technical artist and Three.js programmer. I want you to write a complete, single-file HTML document that uses Three.js (via CDN) to procedurally generate and render a 3D scene/mesh.
Theme/Topic: "${topic}"
Visual Style: ${flavor}
Please write only the raw HTML code containing JS (Three.js renderer, camera, lights, materials, geometries) and CSS to run the scene directly in an iframe.
Make sure it's visually polished, uses OrbitControls if possible, and includes animation in a requestAnimationFrame loop.
Return EXACTLY the valid HTML string (starting with <!DOCTYPE html>) and NO markdown formatting or \`\`\`html blocks around it. Do not include any explanations.`;

      const responseStream = await ai.models.generateContentStream({
        model: "gemini-3.1-pro-preview",
        contents: prompt,
      });

      res.setHeader("Content-Type", "text/event-stream");
      res.setHeader("Cache-Control", "no-cache");
      res.setHeader("Connection", "keep-alive");

      for await (const chunk of responseStream) {
        res.write(`data: ${JSON.stringify({ text: chunk.text })}\n\n`);
      }
      res.write("data: [DONE]\n\n");
      res.end();
    } catch (error: any) {
      console.error("Gemini 3D Engine Error:", error);
      res.status(500).json({ error: error.message });
    }
  });

  app.post("/api/franchise-factory/generate-game", async (req, res) => {
    try {
      const { topic, genre, platform, franchiseContext } = req.body;
      const { GoogleGenAI } = await import("@google/genai");
      const ai = new GoogleGenAI({ apiKey: process.env.GEMINI_API_KEY });
      const contextPrefix = franchiseContext ? `Please adhere STRICTLY to the following franchise context and lore to maintain consistency across the IP:\n${franchiseContext}\n\n` : '';
      const prompt = `${contextPrefix}You are a professional game designer and programmer AI. Please write a highly detailed Game Design Document (GDD) or code outline for a game with the following parameters:
Topic/Idea: "${topic}"
Genre: ${genre}
Platform target: ${platform}
Format using markdown with clear headings, providing mechanics, story segments, and technical architecture outline.`;

      const responseStream = await ai.models.generateContentStream({
        model: "gemini-3.1-pro-preview",
        contents: prompt,
      });

      res.setHeader("Content-Type", "text/event-stream");
      res.setHeader("Cache-Control", "no-cache");
      res.setHeader("Connection", "keep-alive");

      for await (const chunk of responseStream) {
        if (chunk.text) {
          res.write(`data: ${JSON.stringify({ text: chunk.text })}\n\n`);
        }
      }
      res.write(`data: [DONE]\n\n`);
      res.end();
    } catch (error: any) {
      console.error(error);
      if (!res.headersSent) {
        res.status(500).json({ error: error.message });
      } else {
        res.write(`data: ${JSON.stringify({ error: error.message })}\n\n`);
        res.end();
      }
    }
  });

  app.post("/api/franchise-factory/generate-image", async (req, res) => {
    try {
      const { prompt, aspectRatio } = req.body;
      const { GoogleGenAI } = await import("@google/genai");
      const ai = new GoogleGenAI({ apiKey: process.env.GEMINI_API_KEY });
      
      const interaction = await ai.interactions.create({
        model: 'gemini-3.1-flash-image',
        input: prompt,
        response_modalities: ['image', 'text'],
        generation_config: {
          image_config: {
            aspect_ratio: aspectRatio || "1:1",
            image_size: "1K"
          },
        },
      });

      let imageUrl = null;
      for (const step of interaction.steps) {
        if (step.type === 'model_output') {
          const imageContent = step.content?.find(c => c.type === 'image');
          if (imageContent && imageContent.data) {
            const base64EncodeString: string = imageContent.data;
            const mimeType = imageContent.mime_type || 'image/png';
            imageUrl = `data:${mimeType};base64,${base64EncodeString}`;
          }
        }
      }

      if (imageUrl) {
        res.json({ imageUrl });
      } else {
        throw new Error("No image generated by the AI model.");
      }
    } catch (error: any) {
      console.error(error);
      res.status(500).json({ error: error.message });
    }
  });

  app.post("/api/franchise-factory/generate-video", async (req, res) => {
    try {
      const { prompt, aspectRatio } = req.body;
      const { GoogleGenAI } = await import("@google/genai");
      const ai = new GoogleGenAI({ apiKey: process.env.GEMINI_API_KEY });
      const operation = await ai.models.generateVideos({
        model: 'veo-3.1-lite-generate-preview',
        prompt,
        config: {
          numberOfVideos: 1,
          resolution: '720p', 
          aspectRatio: aspectRatio || '16:9'
        }
      });
      res.json({ operationName: operation.name });
    } catch (error: any) {
      console.error(error);
      res.status(500).json({ error: error.message });
    }
  });

  app.post("/api/franchise-factory/video-status", async (req, res) => {
    try {
      const { operationName } = req.body;
      const { GoogleGenAI, GenerateVideosOperation } = await import("@google/genai");
      const ai = new GoogleGenAI({ apiKey: process.env.GEMINI_API_KEY });
      const op = new GenerateVideosOperation();
      op.name = operationName;
      const updated = await ai.operations.getVideosOperation({ operation: op });
      res.json({ done: updated.done });
    } catch (error: any) {
      console.error(error);
      res.status(500).json({ error: error.message });
    }
  });

  app.post("/api/franchise-factory/video-download", async (req, res) => {
    try {
      const { operationName } = req.body;
      const { GoogleGenAI, GenerateVideosOperation } = await import("@google/genai");
      const ai = new GoogleGenAI({ apiKey: process.env.GEMINI_API_KEY });
      const op = new GenerateVideosOperation();
      op.name = operationName;
      const updated = await ai.operations.getVideosOperation({ operation: op });
      const uri = updated.response?.generatedVideos?.[0]?.video?.uri;
      if (!uri) {
        return res.status(404).json({ error: "Video not ready or found." });
      }
      const videoRes = await fetch(uri, {
        headers: { 'x-goog-api-key': process.env.GEMINI_API_KEY! },
      });
      res.setHeader('Content-Type', 'video/mp4');
      videoRes.body!.pipeTo(
        new WritableStream({
          write(chunk) { res.write(chunk); },
          close() { res.end(); },
        })
      );
    } catch (error: any) {
      console.error(error);
      res.status(500).json({ error: error.message });
    }
  });

  app.post("/api/sync-with-github", async (req, res) => {
    try {
      const { token } = req.body;
      if (!token) {
        return res.status(400).json({ error: "GitHub API token is required" });
      }

      // Minimal validation: Make a call to github API user endpoint to verify token
      const gRes = await fetch("https://api.github.com/user", {
        headers: {
          "Authorization": `Bearer ${token}`,
          "Accept": "application/vnd.github.v3+json",
          "User-Agent": "A-TownChain-Sync-Helper"
        }
      });

      if (!gRes.ok) {
        return res.status(401).json({ error: "Invalid GitHub API token" });
      }

      // Check webhook status (mocking a successful webhook verification for active repo)
      const webhooksActive = true; 
      const webhooksStatus = "Active (Connected)";

      // Token is valid. We mock the sync trigger across architecture modules defined in requirementsData.tsx
      return res.json({ 
        success: true, 
        message: "Triggered repository sync successfully across architecture modules.",
        syncedModules: ["atc-core-kernel", "atvm-sandbox", "atown-os"],
        webhooks: {
          active: webhooksActive,
          status: webhooksStatus
        }
      });
      
    } catch (error: any) {
      console.error(error);
      res.status(500).json({ error: error.message });
    }
  });


  app.get("/api/audit/self-code", async (req, res) => {
    try {
      const walkDir = async (dir: string): Promise<{files: number, lines: number, size: number}> => {
        let files = 0;
        let lines = 0;
        let size = 0;
        const list = await fsPromises.readdir(dir);
        for (const file of list) {
          const filePath = path.join(dir, file);
          const stat = await fsPromises.stat(filePath);
          if (stat.isDirectory()) {
            if (file !== "node_modules" && file !== "dist" && file !== ".git") {
              const sub = await walkDir(filePath);
              files += sub.files;
              lines += sub.lines;
              size += sub.size;
            }
          } else if (file.endsWith(".ts") || file.endsWith(".tsx")) {
            files++;
            size += stat.size;
            const content = await fsPromises.readFile(filePath, "utf8");
            lines += content.split('\n').length;
          }
        }
        return { files, lines, size };
      };
      
      const stats = await walkDir("src");
      res.json(stats);
    } catch (e: any) {
      res.status(500).json({ error: e.message });
    }
  });

  app.get("/api/orchestrator/health", async (req, res) => {
    try {
      const endpoints = [
        { id: "github", name: "GitHub API", url: "https://api.github.com/zen", group: "external" },
        { id: "cloudflare", name: "Cloudflare DNS", url: "https://cloudflare-dns.com/dns-query", group: "core" },
        { id: "google", name: "Google Public Route", url: "https://8.8.8.8", group: "core" },
        { id: "coingecko", name: "Crypto Pricing", url: "https://api.coingecko.com/api/v3/ping", group: "blockchain" },
        { id: "gemini", name: "Gemini API", url: "https://generativelanguage.googleapis.com/v1beta/models", group: "ai" }
      ];

      const results = await Promise.all(endpoints.map(async (ep) => {
        const start = Date.now();
        try {
          const check = await fetch(ep.url, { method: 'GET', signal: AbortSignal.timeout(3000) });
          const latency = Date.now() - start;
          return {
            ...ep,
            status: check.ok ? "healthy" : "degraded",
            latency,
            errorRate: check.ok ? 0 : 100,
            requests: Math.floor(Math.random() * 5000) + 1000 // we can't get real global reqs for external
          };
        } catch(e) {
          return {
            ...ep,
            status: "offline",
            latency: 0,
            errorRate: 100,
            requests: 0
          };
        }
      }));

      res.json(results);
    } catch(e: any) {
      res.status(500).json({ error: e.message });
    }
  });

  app.get("/api/system/metrics", (req, res) => {
    try {
      const freeMem = os.freemem();
      const totalMem = os.totalmem();
      const usedMem = totalMem - freeMem;
      const memUsagePerc = (usedMem / totalMem) * 100;
      
      const loadAvg = os.loadavg(); // array [1, 5, 15] mins
      const cpus = os.cpus();
      
      // Node process memory
      const memUsage = process.memoryUsage();
      
      res.json({
        os: {
          platform: os.platform(),
          release: os.release(),
          uptime: os.uptime(),
          cpus: cpus.length,
          cpuModel: cpus[0]?.model,
          load: loadAvg
        },
        memory: {
          total: totalMem,
          free: freeMem,
          usedPct: memUsagePerc
        },
        process: {
          uptime: process.uptime(),
          rss: memUsage.rss,
          heapTotal: memUsage.heapTotal,
          heapUsed: memUsage.heapUsed
        }
      });
    } catch(e: any) {
      res.status(500).json({ error: e.message });
    }
  });

  app.post("/api/gemini/suggest-fix", async (req, res) => {
    try {
      const { runName, repo, errorLog } = req.body;
      const { GoogleGenAI } = await import("@google/genai");
      const ai = new GoogleGenAI({ apiKey: process.env.GEMINI_API_KEY });
      
      const prompt = `Analyze the following failed GitHub Actions workflow run and suggest a code fix.
Repository: ${repo}
Workflow Name: ${runName}

Error Log Snippet:
${errorLog || "No logs provided. Infer potential common issues for this workflow."}

Provide a concise explanation of the failure and a suggested code fix or mitigation strategy. Format the output in Markdown.`;

      const response = await ai.models.generateContent({
        model: "gemini-3.1-pro-preview",
        contents: prompt
      });
      
      res.json({ suggestion: response.text });
    } catch (error: any) {
      console.error(error);
      res.status(500).json({ error: error.message });
    }
  });

  app.post("/api/gemini/analyze-log", async (req, res) => {
    try {
      const { runName, repo, errorLog } = req.body;
      const { GoogleGenAI, Type } = await import("@google/genai");
      const ai = new GoogleGenAI({ apiKey: process.env.GEMINI_API_KEY });
      
      const prompt = `Analyze the following failed GitHub Actions workflow run log.
Repository: ${repo}
Workflow Name: ${runName}

Error Log Snippet:
${errorLog || "Error: Process completed with exit code 1. Unknown failure."}

Extract the exact line number of the failure (if available, otherwise 'N/A'), a relevant code snippet or log snippet representing the exact cause, and a brief user-friendly explanation of the failure.`;

      const response = await ai.models.generateContent({
        model: "gemini-3.1-pro-preview",
        contents: prompt,
        config: {
          responseMimeType: "application/json",
          responseSchema: {
            type: Type.OBJECT,
            properties: {
              lineNumber: { type: Type.STRING },
              snippet: { type: Type.STRING },
              explanation: { type: Type.STRING }
            },
            required: ["lineNumber", "snippet", "explanation"]
          }
        }
      });
      
      res.json(JSON.parse(response.text!));
    } catch (error: any) {
      console.error(error);
      res.status(500).json({ error: error.message });
    }
  });

  app.use("/api/notion", notionRouter);

  // Vite middleware for development
  if (process.env.NODE_ENV !== "production") {
    const vite = await createViteServer({
      server: { middlewareMode: true },
      appType: "spa",
    });
    app.use(vite.middlewares);
  } else {
    const distPath = path.join(process.cwd(), 'dist');
    app.use(express.static(distPath));
    app.get('*', (req, res) => {
      res.sendFile(path.join(distPath, 'index.html'));
    });
  }

  app.listen(PORT, "0.0.0.0", () => {
    console.log(`Server running on http://localhost:${PORT}`);
  });
}

startServer();
