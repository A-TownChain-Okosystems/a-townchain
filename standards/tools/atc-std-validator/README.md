# atc-std-validator (v0.1.0)

Validator fuer die ATC-STD-000-Verfassung: prueft Standards gegen Metadaten-,
ID-, Lifecycle-, Struktur- und Registry-Regeln (S-01…S-15) inklusive
Abhaengigkeitszyklen-Erkennung.

```bash
python3 tools/atc-std-validator/atc_std_validator.py standards/repository/ATC-STD-201.md
python3 tools/atc-std-validator/atc_std_validator.py governance/ATC-STD-000.md --registry registry/standards.yaml
```

Exit 0 = COMPLIANT, 1 = NON-COMPLIANT. Kein Eintrag in registry/standards.yaml
= S-14 FAIL = kein Standard (ATC-STD-000 §19).
