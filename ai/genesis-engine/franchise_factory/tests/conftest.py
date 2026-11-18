"""Testpfad fuer die GFF-Suite (SCR-0146).

Das Paket ``gff`` liegt neben dem tests/-Verzeichnis. Ohne diesen Eintrag schlaegt
der Import fehl, sobald die Suite aus dem Repo-Wurzelverzeichnis laeuft (CI).
"""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
