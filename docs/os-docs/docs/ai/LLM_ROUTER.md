# Kapitel 35 — LLM-Router & Model-Registry

> Version: 1.0.0 | Stand: 2026-06-09 | KAI-OS Wiki
> Status: Geplant (Sprint 3.12)

---

## 35.1 Konzept

Der KAI-OS LLM-Router waehlt automatisch das optimale Modell fuer jede Aufgabe.

```
Agent -> LLM-Router -> [Modell-Selektion] -> Inferenz-Engine -> Antwort
                            |
                      Model-Registry (on-chain + IPFS)
```

---

## 35.2 Task-Typen

| TaskType | Beispiele | Bevorzugte Modelle |
|----------|-----------|-------------------|
| CODE_GENERATION | Code schreiben | deepseek-coder-7b, codellama-34b |
| CODE_ANALYSIS | Code reviewen | deepseek-coder-7b, claude-3.5 |
| TEXT_GENERATION | Texte verfassen | mistral-7b, llama3-8b |
| SUMMARIZATION | Zusammenfassen | phi-3-mini, mistral-7b |
| VISION | Bilder analysieren | llava-13b, gpt-4o |
| MATH_REASONING | Rechenaufgaben | llama3-70b, gpt-4o |
| FAST_REPLY | < 500ms Antwort | phi-3-mini, gemma-2b |
| CHAIN_ANALYSIS | Blockchain-Daten | mistral-7b, llama3-8b |

---

## 35.3 Modell-Katalog (Lokal + Remote)

| Modell | Groesse | Latenz | Kosten | Vision |
|--------|---------|--------|--------|--------|
| phi-3-mini | 2.4 GB | 200ms | kostenlos | Nein |
| gemma-2b | 1.6 GB | 150ms | kostenlos | Nein |
| mistral-7b | 4.1 GB | 800ms | kostenlos | Nein |
| llama3-8b | 4.7 GB | 900ms | kostenlos | Nein |
| llama3-70b | 39 GB | 8000ms | kostenlos | Nein |
| deepseek-coder-7b | 4.0 GB | 850ms | kostenlos | Nein |
| llava-13b | 7.5 GB | 2000ms | kostenlos | Ja |
| gemini-1.5-pro | remote | 2000ms | 0.00125/1k | Ja |
| gpt-4o | remote | 3000ms | 0.005/1k | Ja |
| claude-3.5-sonnet | remote | 2500ms | 0.003/1k | Ja |

---

## 35.4 Router-Implementierung

```python
# core/llm_router.py

class KAILLMRouter:
    def route(self, task, prompt, budget_atc=0.0, max_latency_ms=10000, require_local=True):
        candidates = self.TASK_ROUTING.get(task, ["mistral-7b"])
        for model_name in candidates:
            spec = self.MODELS[model_name]
            if require_local and spec.provider != "local":
                continue
            if spec.avg_latency_ms > max_latency_ms:
                continue
            if spec.provider == "local" and not self._model_fits_in_memory(spec):
                continue
            return spec, f"Optimal fuer {task}: {model_name}"
        # Fallback: Remote wenn Budget vorhanden
        if budget_atc > 0:
            for model_name in candidates:
                spec = self.MODELS[model_name]
                if spec.provider == "remote":
                    return spec, f"Remote-Fallback: {model_name}"
        return self.MODELS["phi-3-mini"], "Fallback: kleinstes Modell"

    def verify_model_integrity(self, model_name, local_path):
        import hashlib
        spec = self.MODELS[model_name]
        with open(local_path, 'rb') as f:
            h = hashlib.blake2b(f.read(), digest_size=32).hexdigest()
        return h == spec.hash_blake2b
```

---

## 35.5 On-Chain Model-Registry (pallet-ai-registry Erweiterung)

```rust
#[derive(Encode, Decode, Clone, TypeInfo)]
pub struct ModelMetadata {
    pub name: BoundedVec<u8, ConstU32<64>>,
    pub version: BoundedVec<u8, ConstU32<16>>,
    pub ipfs_cid: BoundedVec<u8, ConstU32<64>>,   // Download via IPFS
    pub blake2b_hash: [u8; 32],                     // Integritaets-Hash
    pub size_bytes: u64,
    pub context_length: u32,
    pub supports_vision: bool,
    pub task_types: BoundedVec<u8, ConstU32<16>>,  // Bitmap
    pub registered_by: T::AccountId,
    pub is_active: bool,
    pub audit_score: u8,  // 0-100, von DAO gesetzt
}

// Registrierung: Mindest-Stake 100 ATC
pub fn register_model(origin, metadata: ModelMetadata) -> DispatchResult {
    let who = ensure_signed(origin)?;
    T::Currency::reserve(&who, T::ModelRegistrationDeposit::get())?;
    ModelRegistry::<T>::insert(&metadata.name, metadata);
    Ok(())
}
```

---

## 35.6 IPFS-Modell-Download

```python
async def download_model_from_ipfs(model_name: str, ipfs_cid: str) -> str:
    local_path = f"/var/kai/models/{model_name}.gguf"
    if os.path.exists(local_path):
        # Integritaet pruefen
        if router.verify_model_integrity(model_name, local_path):
            return local_path
    # Download via IPFS
    async with aiohttp.ClientSession() as session:
        async with session.get(f"http://localhost:8080/ipfs/{ipfs_cid}") as resp:
            with open(local_path, 'wb') as f:
                async for chunk in resp.content.iter_chunked(1024*1024):
                    f.write(chunk)
    # Hash-Verifikation vor Verwendung
    if not router.verify_model_integrity(model_name, local_path):
        os.remove(local_path)
        raise ValueError(f"Modell-Hash stimmt nicht: {model_name}")
    return local_path
```

---

## 35.7 Roadmap

| Sprint | Aufgabe | Datum |
|--------|---------|-------|
| 3.12 | LLM-Router + pallet-ai-registry Erweiterung + IPFS-Download | Mai 2027 |
| 4.3 | Mainnet: 5 Modelle in Registry registriert | Sep 2027 |

---

*KAI-OS Wiki Kapitel 35 — LLM-Router & Model-Registry | v1.0.0 | 2026-06-09*
