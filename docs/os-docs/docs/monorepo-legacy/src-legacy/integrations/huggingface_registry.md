# HuggingFace Modell-Registry
> Exportiert: 2026-06-12 06:59 UTC | Aurora v3.1

## Eingebundene KI-Modelle (4/4)

| Modell | Likes | Downloads | Status |
|--------|-------|-----------|--------|
| [google/gemma-2-2b-it](https://huggingface.co/google/gemma-2-2b-it) | 1,384 | 320,754 | ✅ Verfügbar |
| [mistralai/Mistral-7B-Instruct-v0.2](https://huggingface.co/mistralai/Mistral-7B-Instruct-v0.2) | 3,157 | 1,507,651 | ✅ Verfügbar |
| [microsoft/phi-2](https://huggingface.co/microsoft/phi-2) | 3,469 | 446,676 | ✅ Verfügbar |
| [meta-llama/Llama-3.2-3B-Instruct](https://huggingface.co/meta-llama/Llama-3.2-3B-Instruct) | 2,219 | 1,472,295 | ✅ Verfügbar |

## Verwendung im Projekt
- **KI-Kernel:** `modules/kernel/ai_kernel/ai_kernel.py`
- **Config:** `config/ai_models.json`
- **Issue:** #50 HuggingFace Code-Review Pipeline

## Geplante Integration
```python
# Beispiel: Code-Review Pipeline
from huggingface_hub import InferenceClient
client = InferenceClient(model="google/gemma-2-2b-it")
review = client.text_generation(f"Review this code: {code}")
```

---
*Automatisch exportiert von Aurora v3.1 — 2026-06-12 06:59 UTC*
