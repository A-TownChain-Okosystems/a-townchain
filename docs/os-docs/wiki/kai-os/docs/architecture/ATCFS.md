# 📂 ATCFS — Dezentrales Dateisystem
**Stand:** 09.06.2026 | **Version:** v2.1.0 | **Datei:** `shivaos/fs/atcfs.py` (330 Zeilen)

---

## Überblick

ATCFS (A-TownChain File System) ist das proprietäre Dateisystem von ShivaOS. Es kombiniert ein lokales, verschlüsseltes Dateisystem mit optionaler dezentraler Speicherung.

```
┌─────────────────────────────────────────────────┐
│                    ATCFS                         │
├───────────────┬─────────────────────────────────┤
│  VFS Layer    │  Encryption Layer               │
│  (Virtual FS) │  (AES-256-GCM)                  │
├───────────────┴─────────────────────────────────┤
│           Storage Backends                       │
├──────────┬──────────┬──────────────────────────┤
│  Local   │  SQLite  │  Distributed (P2P)        │
│  (disk)  │  (DB)    │  (ATCNet DHT)             │
└──────────┴──────────┴──────────────────────────┘
```

---

## Dateisystem-Struktur

```
/
├── system/          ← Kernel + Core Services
│   ├── kernel/
│   ├── services/
│   └── config/
├── contracts/       ← Deployed Smart Contracts
│   ├── atc8300/
│   └── shivamon/
├── data/            ← Blockchain-Daten
│   ├── blocks/
│   ├── txpool/
│   └── state/
├── wallet/          ← Verschlüsselte Wallet-Dateien
├── logs/            ← System-Logs
├── tmp/             ← Temporäre Dateien
└── home/            ← User-Dateien
    └── <address>/   ← Pro-Wallet Verzeichnis
```

---

## API

```python
class ATCFS:
    """
    ATCFS — Proprietäres ShivaOS Dateisystem.
    ATS-1003 konform.
    """

    def open(self, path: str, mode: str = "r") -> FileHandle:
        """Datei öffnen. Modes: r, w, a, rb, wb"""

    def read(self, handle: FileHandle, size: int = -1) -> bytes:
        """Datei lesen. size=-1 → alles lesen."""

    def write(self, handle: FileHandle, data: bytes) -> int:
        """Daten schreiben. Gibt geschriebene Bytes zurück."""

    def close(self, handle: FileHandle) -> None:
        """Datei-Handle schließen."""

    def stat(self, path: str) -> FileStat:
        """Metadaten: size, created, modified, permissions, hash"""

    def mkdir(self, path: str, recursive: bool = False) -> bool:
        """Verzeichnis erstellen."""

    def ls(self, path: str) -> List[FileEntry]:
        """Verzeichnis auflisten."""

    def rm(self, path: str, recursive: bool = False) -> bool:
        """Datei/Verzeichnis löschen."""

    def mv(self, src: str, dst: str) -> bool:
        """Verschieben/Umbenennen."""

    def cp(self, src: str, dst: str) -> bool:
        """Kopieren."""

    def hash_file(self, path: str) -> str:
        """SHA-256-Hash einer Datei berechnen."""

    def encrypt_file(self, path: str, key: bytes) -> str:
        """Datei AES-256-GCM verschlüsseln."""

    def decrypt_file(self, path: str, key: bytes) -> bytes:
        """Verschlüsselte Datei entschlüsseln."""
```

---

## Speicher-Backends

### 1. Local Backend
Direkte Dateisystem-Speicherung auf dem Host:
```python
backend = LocalBackend(base_path="/var/atcfs")
```

### 2. SQLite Backend (Issue #4)
Für Smart Contract State und Blockchain-Daten:
```python
backend = SQLiteBackend(db_path="/var/atcfs/state.db")
# Tabellen: files, directories, metadata, contracts_state
```

### 3. P2P/DHT Backend (v2.2+)
Dezentrale Speicherung über ATCNet:
```python
backend = P2PBackend(atcnet=atcnet_instance, replication=3)
```

---

## Verschlüsselung

- **Algorithmus:** AES-256-GCM
- **Key-Ableitung:** PBKDF2-HMAC-SHA256 (100.000 Iterationen)
- **Wallet-Dateien:** immer verschlüsselt
- **Contract-State:** optional verschlüsselt (Private Contracts)
