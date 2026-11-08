# ATCFS — Proprietäres Dateisystem

## Übersicht
ATCFS (A-TownChain FileSystem) ist ein dezentrales, blockchain-integriertes Dateisystem.
Kein POSIX-Klon — vollständig proprietäre Implementierung nach ATS-1003 Standard.

## Knoten-Typen (INode)
| Typ | Beschreibung |
|-----|-------------|
| `FILE` | Reguläre Datei |
| `DIRECTORY` | Verzeichnis |
| `SYMLINK` | Symbolischer Link |
| `CHAIN_STATE` | Blockchain-State-Datei |
| `CONTRACT` | Smart Contract Bytecode |

## Berechtigungen (ATC-Permissions)
```
OWNER: read | write | execute
GROUP: read | write | execute  
OTHER: read | execute
```

## API
```python
fs = ATCFS()
# Datei schreiben
fs.write("/contracts/hello.atc", source_bytes, mode=OpenMode.WRITE)
# Datei lesen
data = fs.read("/contracts/hello.atc")
# Verzeichnis
fs.mkdir("/contracts")
entries = fs.listdir("/contracts")
# Löschen
fs.delete("/contracts/hello.atc")
```

## ATCFS Quellcode (Auszug)
```python
"""
ATCFS — Dezentrales Dateisystem für ShivaOS
Version: 1.0.0-alpha | ATS-1002 konform
Kein IPFS-Klon — eigene Content-Adressierung
"""

import hashlib, time, json, os
from dataclasses import dataclass, field
from typing import Dict, List, Optional
from enum import IntEnum, auto
from pathlib import Path


# ══════════════════════════════════════════════════════════
#  TYPEN & KONSTANTEN
# ══════════════════════════════════════════════════════════

class FileType(IntEnum):
    FILE     = auto()
    DIR      = auto()
    SYMLINK  = auto()
    CONTRACT = auto()   # .atcb Dateien

class OpenMode(IntEnum):
    READ    = 0b001
    WRITE   = 0b010
    APPEND  = 0b100
    EXEC    = 0b1000

ATC_EXTENSIONS = {
    ".atc":  "ATCLang Quellcode",
    ".atcb": "ATCLang Bytecode",
    ".atcm": "ATC-Modul",
    ".atcw": "ATC-Wallet",
    ".atcd": "ATC-Daten",
    ".atcp": "ATC-Prozess-Image",
}


def atc_content_id(data: bytes) -> str:
    """
    Eigene Content-ID Berechnung.
    SHA3-256 mit ATCFS-Domain — kein IPFS CID.
    """
    h = hashlib.sha3_256()
    h.update(b"atcfs_v1||")
    h.update(data)
    return "atc1" + h.hexdigest()  # Präfix "atc1" statt "Qm"


@dataclass
class ATCPermissions:
    owner_rwx: int = 0b111   # rwx
    group_rwx: int = 0b101   # r-x
    world_rwx: int = 0b100   # r--

    def can_read(self, is_owner: bool, is_group: bool) -> bool:
        if is_owner: return bool(self.owner_rwx & 0b100)
        if is_group: return bool(self.group_rwx & 0b100)
        return bool(self.world_rwx & 0b100)

    def can_write(self, is_owner: bool) -> bool:
        if is_owner: return bool(self.owner_rwx & 0b010)
        return False

    def can_exec(self, is_owner: bool, is_group: bool) -> bool:
        if is_owner: return bool(self.owner_rwx & 0b001)
        if is_group
```
