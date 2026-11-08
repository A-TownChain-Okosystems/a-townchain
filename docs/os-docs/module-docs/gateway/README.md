# gateway — L7: API Gateway :4000 — Auth, Rate-Limiting, Routing

Part of [A-TownChain OS Monorepo](../../README.md)

## Inhalt

```
modules/gateway/
├── .env.example
├── CHANGELOG.md
├── SECURITY.md
├── __init__.py
├── gateway.atc
├── main.py
├── middleware/__init__.py
├── middleware/auth.py
├── middleware/logger.py
├── middleware/rate_limit.py
├── middleware/signature_verify.py
├── requirements.txt
├── router.py
```

## Starten

```bash
cd modules/gateway
pip install -r requirements.txt  # falls vorhanden
python -m gateway  # oder spezifischer Einstiegspunkt
```

## Layer-Kontext

| Layer | Modul | Ports | Abhängigkeiten |
|-------|-------|-------|----------------|
| L7 | gateway | — | core/, blockchain/ |

---
*Teil des A-TownChain OS Monorepo — Apache 2.0*
