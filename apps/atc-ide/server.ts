import express from "express";
import path from "path";
import { createServer as createViteServer } from "vite";
import { GoogleGenAI } from "@google/genai";

async function startServer() {
  const app = express();
  const PORT = 3000;

  app.use(express.json({ limit: "50mb" }));

  // Initialize Gemini AI
  const ai = new GoogleGenAI({
    apiKey: process.env.GEMINI_API_KEY,
    httpOptions: {
      headers: {
        'User-Agent': 'aistudio-build',
      }
    }
  });

  // Chat API
  app.post("/api/chat", async (req, res) => {
    try {
      const { prompt, history, files } = req.body;
      
      const contents: any[] = [];
      
      if (history && history.length > 0) {
          // just taking the text from the previous requests for simplicity
          history.forEach((msg: any) => {
             // simplified message handling
             contents.push({ role: msg.role === 'user' ? 'user' : 'model', parts: [{ text: msg.text }] });
          });
      }
      
      const currentParts: any[] = [];
      if (files && files.length > 0) {
         files.forEach((file: any) => {
            currentParts.push({
               inlineData: {
                  mimeType: file.type || "text/plain",
                  data: file.data.split(',')[1] || file.data
               }
            });
         });
      }
      
      currentParts.push({ text: prompt });
      contents.push({ role: 'user', parts: currentParts });

      const response = await ai.models.generateContentStream({
        model: "gemini-3.5-flash",
        contents: contents,
        config: {
          systemInstruction: "Du bist ein autonomer KI-Softwareentwickler. Deine Aufgabe: Automatisch die vollständige Software schreiben, Unklarheiten klären, auf Fehler prüfen, Funktionen verbinden und testen, sowie Verbesserungen und Änderungsvorschläge machen. Du arbeitest den Projekt-Roadmap Schritt für Schritt ab.",
        }
      });
      
      res.setHeader('Content-Type', 'text/event-stream');
      res.setHeader('Cache-Control', 'no-cache');
      res.setHeader('Connection', 'keep-alive');

      for await (const chunk of response) {
        if (chunk.text) {
          res.write(`data: ${JSON.stringify({ text: chunk.text })}\n\n`);
        }
      }
      res.write('data: [DONE]\n\n');
      res.end();

    } catch (error: any) {
      console.error('Gemini API Error:', error);
      res.status(500).json({ error: error.message || 'Failed to generate content' });
    }
  });

  // ----------------------------------------------------
  // GitHub Integration APIs
  // ----------------------------------------------------

  // 1. Get GitHub OAuth Authorize URL
  app.get("/api/github/auth-url", (req, res) => {
    const clientId = process.env.GITHUB_CLIENT_ID || process.env.CLIENT_ID;
    if (!clientId) {
      return res.status(400).json({
        error: "GITHUB_CLIENT_ID is not configured in environment variables.",
      });
    }

    const host = req.headers.host || "localhost:3000";
    const protocol = req.headers["x-forwarded-proto"] || "https";
    const appUrl = process.env.APP_URL || `${protocol}://${host}`;
    const redirectUri = `${appUrl}/auth/callback`;

    const params = new URLSearchParams({
      client_id: clientId,
      redirect_uri: redirectUri,
      scope: "repo user workflow",
      allow_signup: "true",
    });

    const authUrl = `https://github.com/login/oauth/authorize?${params.toString()}`;
    res.json({ url: authUrl, redirectUri });
  });

  // 2. Exchange OAuth Code for Token
  app.post("/api/github/token", async (req, res) => {
    try {
      const { code } = req.body;
      const clientId = process.env.GITHUB_CLIENT_ID || process.env.CLIENT_ID;
      const clientSecret = process.env.GITHUB_CLIENT_SECRET || process.env.CLIENT_SECRET;

      if (!clientId || !clientSecret) {
        return res.status(400).json({
          error: "GitHub OAuth Client ID or Secret is not configured in the server environment.",
        });
      }

      const response = await fetch("https://github.com/login/oauth/access_token", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Accept: "application/json",
          "User-Agent": "ATOS-IDE-GitHub-Sync",
        },
        body: JSON.stringify({
          client_id: clientId,
          client_secret: clientSecret,
          code,
        }),
      });

      const data = await response.json();
      if (data.error) {
        return res.status(400).json({ error: data.error_description || data.error });
      }

      res.json(data);
    } catch (err: any) {
      console.error("OAuth Token Exchange Error:", err);
      res.status(500).json({ error: err.message || "Failed to exchange token" });
    }
  });

  // 3. GitHub OAuth Callback HTML Window Responder
  app.get(["/auth/callback", "/auth/callback/"], (req, res) => {
    const code = req.query.code;
    const error = req.query.error;

    res.send(`
      <!DOCTYPE html>
      <html>
        <head>
          <title>GitHub Authentication</title>
          <style>
            body { font-family: system-ui, -apple-system, sans-serif; background: #0f172a; color: #f8fafc; display: flex; align-items: center; justify-content: center; height: 100vh; margin: 0; }
            .box { background: #1e293b; padding: 2rem; border-radius: 1rem; border: 1px solid rgba(255,255,255,0.1); text-align: center; max-width: 400px; }
            .spinner { width: 36px; height: 36px; border: 3px solid rgba(99,102,241,0.2); border-top-color: #6366f1; border-radius: 50%; animation: spin 1s linear infinite; margin: 0 auto 1.5rem; }
            @keyframes spin { to { transform: rotate(360deg); } }
            h2 { margin: 0 0 0.5rem; font-size: 1.25rem; }
            p { color: #94a3b8; font-size: 0.875rem; margin: 0; }
          </style>
        </head>
        <body>
          <div class="box">
            <div class="spinner"></div>
            <h2>GitHub Authentifizierung...</h2>
            <p>Das Fenster schließt sich automatisch und überträgt das Token.</p>
          </div>
          <script>
            const code = ${JSON.stringify(code || "")};
            const error = ${JSON.stringify(error || "")};
            if (window.opener) {
              window.opener.postMessage({
                type: 'GITHUB_AUTH_CALLBACK',
                code: code,
                error: error
              }, '*');
              setTimeout(() => window.close(), 600);
            } else {
              window.location.href = '/';
            }
          </script>
        </body>
      </html>
    `);
  });

  // 4. Authenticated User Profile
  app.get("/api/github/user", async (req, res) => {
    const authHeader = req.headers.authorization;
    if (!authHeader) return res.status(401).json({ error: "No Authorization header" });

    try {
      const response = await fetch("https://api.github.com/user", {
        headers: {
          Authorization: authHeader,
          Accept: "application/vnd.github+json",
          "User-Agent": "ATOS-IDE-GitHub-Sync",
        },
      });

      if (!response.ok) {
        const err = await response.json();
        return res.status(response.status).json(err);
      }

      const userData = await response.json();
      res.json(userData);
    } catch (err: any) {
      res.status(500).json({ error: err.message });
    }
  });

  // 5. List User Repositories
  app.get("/api/github/repos", async (req, res) => {
    const authHeader = req.headers.authorization;
    if (!authHeader) return res.status(401).json({ error: "No Authorization header" });

    try {
      const response = await fetch("https://api.github.com/user/repos?sort=updated&per_page=100&type=all", {
        headers: {
          Authorization: authHeader,
          Accept: "application/vnd.github+json",
          "User-Agent": "ATOS-IDE-GitHub-Sync",
        },
      });

      if (!response.ok) {
        const err = await response.json();
        return res.status(response.status).json(err);
      }

      const repos = await response.json();
      res.json(repos);
    } catch (err: any) {
      res.status(500).json({ error: err.message });
    }
  });

  // 6. Create a New GitHub Repository
  app.post("/api/github/repos", async (req, res) => {
    const authHeader = req.headers.authorization;
    if (!authHeader) return res.status(401).json({ error: "No Authorization header" });

    const { name, description, isPrivate, autoInit } = req.body;
    if (!name) return res.status(400).json({ error: "Repository name is required" });

    try {
      const response = await fetch("https://api.github.com/user/repos", {
        method: "POST",
        headers: {
          Authorization: authHeader,
          Accept: "application/vnd.github+json",
          "Content-Type": "application/json",
          "User-Agent": "ATOS-IDE-GitHub-Sync",
        },
        body: JSON.stringify({
          name: name.trim().replace(/\s+/g, "-"),
          description: description || "Created via ATOS IDE",
          private: !!isPrivate,
          auto_init: autoInit !== undefined ? autoInit : true,
        }),
      });

      const repoData = await response.json();
      if (!response.ok) {
        return res.status(response.status).json(repoData);
      }

      res.status(201).json(repoData);
    } catch (err: any) {
      res.status(500).json({ error: err.message });
    }
  });

  // 7. Get Commits from a Repository
  app.get("/api/github/repos/:owner/:repo/commits", async (req, res) => {
    const authHeader = req.headers.authorization;
    if (!authHeader) return res.status(401).json({ error: "No Authorization header" });

    const { owner, repo } = req.params;

    try {
      const response = await fetch(`https://api.github.com/repos/${owner}/${repo}/commits?per_page=30`, {
        headers: {
          Authorization: authHeader,
          Accept: "application/vnd.github+json",
          "User-Agent": "ATOS-IDE-GitHub-Sync",
        },
      });

      if (!response.ok) {
        const err = await response.json();
        return res.status(response.status).json(err);
      }

      const commits = await response.json();
      res.json(commits);
    } catch (err: any) {
      res.status(500).json({ error: err.message });
    }
  });

  // 8. Commit & Push Changes (via GitHub Git Trees / Commits API)
  app.post("/api/github/repos/:owner/:repo/push", async (req, res) => {
    const authHeader = req.headers.authorization;
    if (!authHeader) return res.status(401).json({ error: "No Authorization header" });

    const { owner, repo } = req.params;
    const { branch = "main", message, files } = req.body;

    if (!message || !files || !Array.isArray(files) || files.length === 0) {
      return res.status(400).json({ error: "Commit message and at least one file are required" });
    }

    try {
      const headers = {
        Authorization: authHeader,
        Accept: "application/vnd.github+json",
        "Content-Type": "application/json",
        "User-Agent": "ATOS-IDE-GitHub-Sync",
      };

      // A. Get repository details to find default branch if needed
      let targetBranch = branch;
      const repoRes = await fetch(`https://api.github.com/repos/${owner}/${repo}`, { headers });
      if (repoRes.ok) {
        const repoInfo = await repoRes.json();
        if (!branch && repoInfo.default_branch) {
          targetBranch = repoInfo.default_branch;
        }
      }

      // B. Get latest commit SHA on the target branch
      let latestCommitSha: string | null = null;
      let baseTreeSha: string | null = null;

      const refRes = await fetch(`https://api.github.com/repos/${owner}/${repo}/git/ref/heads/${targetBranch}`, { headers });
      
      if (refRes.ok) {
        const refData = await refRes.json();
        latestCommitSha = refData.object.sha;

        // Get the commit to find its tree SHA
        const commitRes = await fetch(`https://api.github.com/repos/${owner}/${repo}/git/commits/${latestCommitSha}`, { headers });
        if (commitRes.ok) {
          const commitData = await commitRes.json();
          baseTreeSha = commitData.tree.sha;
        }
      }

      // C. Create blobs for each file
      const treeItems: any[] = [];
      for (const f of files) {
        // Create blob
        const blobRes = await fetch(`https://api.github.com/repos/${owner}/${repo}/git/blobs`, {
          method: "POST",
          headers,
          body: JSON.stringify({
            content: f.content,
            encoding: "utf-8",
          }),
        });

        if (!blobRes.ok) {
          const blobErr = await blobRes.json();
          return res.status(blobRes.status).json({ error: `Failed to create blob for ${f.path}: ${blobErr.message || "Unknown error"}` });
        }

        const blobData = await blobRes.json();
        treeItems.push({
          path: f.path.startsWith("/") ? f.path.substring(1) : f.path,
          mode: "100644",
          type: "blob",
          sha: blobData.sha,
        });
      }

      // D. Create a new Tree
      const treeBody: any = { tree: treeItems };
      if (baseTreeSha) {
        treeBody.base_tree = baseTreeSha;
      }

      const createTreeRes = await fetch(`https://api.github.com/repos/${owner}/${repo}/git/trees`, {
        method: "POST",
        headers,
        body: JSON.stringify(treeBody),
      });

      if (!createTreeRes.ok) {
        const treeErr = await createTreeRes.json();
        return res.status(createTreeRes.status).json({ error: `Failed to create Git tree: ${treeErr.message}` });
      }

      const newTreeData = await createTreeRes.json();

      // E. Create the Commit
      const commitBody: any = {
        message: message,
        tree: newTreeData.sha,
        parents: latestCommitSha ? [latestCommitSha] : [],
      };

      const createCommitRes = await fetch(`https://api.github.com/repos/${owner}/${repo}/git/commits`, {
        method: "POST",
        headers,
        body: JSON.stringify(commitBody),
      });

      if (!createCommitRes.ok) {
        const commitErr = await createCommitRes.json();
        return res.status(createCommitRes.status).json({ error: `Failed to create commit: ${commitErr.message}` });
      }

      const newCommitData = await createCommitRes.json();

      // F. Update or create the Branch Reference
      if (latestCommitSha) {
        // Update existing branch ref
        const updateRefRes = await fetch(`https://api.github.com/repos/${owner}/${repo}/git/refs/heads/${targetBranch}`, {
          method: "PATCH",
          headers,
          body: JSON.stringify({
            sha: newCommitData.sha,
            force: false,
          }),
        });

        if (!updateRefRes.ok) {
          const refErr = await updateRefRes.json();
          return res.status(updateRefRes.status).json({ error: `Failed to update branch reference: ${refErr.message}` });
        }
      } else {
        // Create new branch ref
        const createRefRes = await fetch(`https://api.github.com/repos/${owner}/${repo}/git/refs`, {
          method: "POST",
          headers,
          body: JSON.stringify({
            ref: `refs/heads/${targetBranch}`,
            sha: newCommitData.sha,
          }),
        });

        if (!createRefRes.ok) {
          const refErr = await createRefRes.json();
          return res.status(createRefRes.status).json({ error: `Failed to create branch reference: ${refErr.message}` });
        }
      }

      res.json({
        success: true,
        commitSha: newCommitData.sha,
        branch: targetBranch,
        url: newCommitData.html_url || `https://github.com/${owner}/${repo}/commit/${newCommitData.sha}`,
        message: `Successfully pushed commit "${message}" with ${files.length} file(s) to ${owner}/${repo}@${targetBranch}`,
      });

    } catch (err: any) {
      console.error("GitHub Push Error:", err);
      res.status(500).json({ error: err.message || "Failed to push to GitHub" });
    }
  });

  // 9. Pull / Fetch Files from a GitHub Repository
  app.get("/api/github/repos/:owner/:repo/contents", async (req, res) => {
    const authHeader = req.headers.authorization;
    if (!authHeader) return res.status(401).json({ error: "No Authorization header" });

    const { owner, repo } = req.params;
    const branch = (req.query.ref as string) || "main";

    try {
      const headers = {
        Authorization: authHeader,
        Accept: "application/vnd.github+json",
        "User-Agent": "ATOS-IDE-GitHub-Sync",
      };

      // Get repository tree recursively
      const repoRes = await fetch(`https://api.github.com/repos/${owner}/${repo}`, { headers });
      let targetBranch = branch;
      if (repoRes.ok) {
        const repoData = await repoRes.json();
        if (!req.query.ref && repoData.default_branch) {
          targetBranch = repoData.default_branch;
        }
      }

      // Fetch git tree recursively
      const treeRes = await fetch(`https://api.github.com/repos/${owner}/${repo}/git/trees/${targetBranch}?recursive=1`, { headers });
      if (!treeRes.ok) {
        const treeErr = await treeRes.json();
        return res.status(treeRes.status).json(treeErr);
      }

      const treeData = await treeRes.json();
      const files: { name: string; content: string; path: string }[] = [];

      // Filter blobs and fetch content for text files up to sensible limit
      const blobs = (treeData.tree || []).filter((item: any) => item.type === "blob").slice(0, 50);

      for (const blob of blobs) {
        try {
          const blobRes = await fetch(blob.url, { headers });
          if (blobRes.ok) {
            const blobContent = await blobRes.json();
            const decoded = Buffer.from(blobContent.content, "base64").toString("utf-8");
            files.push({
              name: blob.path,
              path: blob.path,
              content: decoded,
            });
          }
        } catch (e) {
          console.warn(`Could not read blob ${blob.path}:`, e);
        }
      }

      res.json({ files, branch: targetBranch, count: files.length });
    } catch (err: any) {
      res.status(500).json({ error: err.message || "Failed to fetch repository files" });
    }
  });

  // Vite middleware for development
  if (process.env.NODE_ENV !== "production" && process.env.DISABLE_HMR !== 'true') {
    const vite = await createViteServer({
      server: { middlewareMode: true },
      appType: "spa",
    });
    app.use(vite.middlewares);
  } else {
    // Basic static serving if we are not in Vite dev mode
    const distPath = path.join(process.cwd(), 'dist');
    app.use(express.static(distPath));
    app.get('*all', (req, res) => {
      res.sendFile(path.join(distPath, 'index.html'));
    });
  }

  app.listen(PORT, "0.0.0.0", () => {
    console.log(`Server running on http://localhost:${PORT}`);
  });
}

startServer();
