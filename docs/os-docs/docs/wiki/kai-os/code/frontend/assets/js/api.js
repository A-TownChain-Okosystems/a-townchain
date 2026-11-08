// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// frontend/assets/js/api.js
// A-TownChain API Client v2.1
//
// ARCHITEKTUR:
//   Frontend → API Gateway (:4000) → Backend Services
//   Frontend spricht NIEMALS direkt mit dem Backend!
//
// BACKEND STRUKTUR:
//   backend/api/routes/blockchain.py  → Gateway :5001
//   backend/api/routes/wallet.py      → Gateway :5002
//   backend/api/server.py (core)      → Gateway :5000

const GATEWAY  = window.ATC_GATEWAY_URL || "http://localhost:4000/api";
const API_KEY  = window.ATC_API_KEY     || "atc-dev-key-2025";

const _headers = {
  "Content-Type": "application/json",
  "X-API-Key":    API_KEY
};

// ── HTTP Helpers ──────────────────────────────────────
async function _get(endpoint) {
  try {
    const res = await fetch(`${GATEWAY}/${endpoint}`, { headers: _headers });
    if (!res.ok) throw new Error(`HTTP ${res.status} on GET /${endpoint}`);
    return await res.json();
  } catch (e) {
    console.warn(`[ATC] GET /${endpoint}:`, e.message);
    return { error: e.message, offline: true };
  }
}

async function _post(endpoint, body = {}) {
  try {
    const res = await fetch(`${GATEWAY}/${endpoint}`, {
      method:  "POST",
      headers: _headers,
      body:    JSON.stringify(body)
    });
    if (!res.ok) throw new Error(`HTTP ${res.status} on POST /${endpoint}`);
    return await res.json();
  } catch (e) {
    console.warn(`[ATC] POST /${endpoint}:`, e.message);
    return { error: e.message, offline: true };
  }
}

// ── API Methoden ──────────────────────────────────────
const ATC_API = {

  // Gateway Health
  async health() {
    try {
      const res = await fetch(
        (window.ATC_GATEWAY_URL || "http://localhost:4000") + "/gateway/health",
        { headers: _headers }
      );
      return await res.json();
    } catch {
      return { gateway: "offline" };
    }
  },

  // ── Core (backend/api/server.py → :5000) ─────────
  async getStatus() {
    // Mapped auf: backend/api/server.py → GET /api/status
    return await _get("status");
  },

  async getModules() {
    // Mapped auf: backend/api/server.py → GET /api/modules
    return await _get("modules");
  },

  // ── Blockchain (backend/api/routes/blockchain.py → :5001) ──
  async getBlockchainInfo() {
    // Mapped auf: backend/api/routes/blockchain.py → GET /api/blockchain/info
    return await _get("blockchain/info");
  },

  async getBlocks() {
    // Mapped auf: backend/api/routes/blockchain.py → GET /api/blockchain/blocks
    return await _get("blockchain/blocks");
  },

  async getTx(txId) {
    // Mapped auf: backend/api/routes/blockchain.py → GET /api/blockchain/tx/:id
    return await _get(`blockchain/tx/${txId}`);
  },

  async getContract(name) {
    // Mapped auf: blockchain/contracts/ via backend
    return await _get(`blockchain/contract/${name}`);
  },

  // ── Wallet (backend/api/routes/wallet.py → :5002) ──
  async getBalance(address) {
    // Mapped auf: backend/api/routes/wallet.py → GET /api/wallet/balance/:address
    return await _get(`wallet/balance/${address}`);
  },

  async sendTransfer(from, to, amount) {
    // Mapped auf: backend/api/routes/wallet.py → POST /api/wallet/send
    return await _post("wallet/send", { from, to, amount });
  },

  // ── AI (backend/api/routes/ai.py → :5003) ──────────
  async queryAI(prompt, model = "gemini-2.0") {
    // Mapped auf: backend/api/routes/ai.py → POST /api/ai/query
    return await _post("ai/query", { prompt, model });
  },

  // ── Game (backend/api/routes/game.py → :5004) ──────
  async getShivamon(id) {
    return await _get(`game/shivamon/${id}`);
  },

  async battleShivamon(attacker, defender) {
    return await _post("game/battle", { attacker, defender });
  }
};

// ── Status-Anzeige beim Laden ─────────────────────────
window.addEventListener("DOMContentLoaded", async () => {
  const health = await ATC_API.health();
  const el = document.getElementById("backend-status");
  if (el) {
    const online = health.gateway === "online";
    el.textContent = online ? "● GATEWAY ONLINE" : "● GATEWAY OFFLINE";
    el.style.color  = online ? "#00ffb3" : "#ff2d78";
  }
  console.log("[ATC] Gateway health:", health);
});

export default ATC_API;
