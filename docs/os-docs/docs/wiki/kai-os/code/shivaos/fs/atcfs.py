# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
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
        if is_group: return bool(self.group_rwx & 0b001)
        return bool(self.world_rwx & 0b001)

    def __str__(self) -> str:
        def bits(v):
            return ("r" if v&4 else "-") + ("w" if v&2 else "-") + ("x" if v&1 else "-")
        return bits(self.owner_rwx) + bits(self.group_rwx) + bits(self.world_rwx)


@dataclass
class INode:
    """ATS-1002 INode — Metadaten einer Datei."""
    cid:       str
    name:      str
    ftype:     FileType
    size:      int
    owner:     str          # ATC-Adresse
    group:     str = ""
    created:   int = field(default_factory=lambda: int(time.time() * 1000))
    modified:  int = field(default_factory=lambda: int(time.time() * 1000))
    perms:     ATCPermissions = field(default_factory=ATCPermissions)
    replicas:  int = 3
    encrypted: bool = False
    children:  List[str] = field(default_factory=list)  # CIDs für Verzeichnisse
    symlink_target: str = ""

    def to_dict(self) -> dict:
        return {
            "cid": self.cid, "name": self.name,
            "type": self.ftype.name, "size": self.size,
            "owner": self.owner, "created": self.created,
            "modified": self.modified, "perms": str(self.perms),
            "replicas": self.replicas,
        }


@dataclass
class FileHandle:
    """Offener Datei-Handle."""
    fh_id:    int
    cid:      str
    mode:     OpenMode
    pos:      int = 0
    pid:      int = 0
    opened_at: int = field(default_factory=lambda: int(time.time() * 1000))


# ══════════════════════════════════════════════════════════
#  ATCFS — DATEISYSTEM
# ══════════════════════════════════════════════════════════

class ATCFS:
    """
    ATCFS — Dezentrales Content-adressiertes Dateisystem.
    Lokal gecacht, dezentral repliziert.
    """

    def __init__(self, root_path: str = "/tmp/atcfs", owner: str = ""):
        self.root_path  = Path(root_path)
        self.owner      = owner
        self.inodes:    Dict[str, INode] = {}
        self.content:   Dict[str, bytes] = {}   # CID → Daten
        self.path_map:  Dict[str, str]   = {}   # Pfad → CID
        self.handles:   Dict[int, FileHandle] = {}
        self._fh_counter = 0
        self.root_path.mkdir(parents=True, exist_ok=True)
        self._init_root()

    def _init_root(self):
        """Root-Verzeichnis / anlegen."""
        root_data = b""
        root_cid  = atc_content_id(b"atcfs_root_v1")
        root_inode = INode(
            cid=root_cid, name="/", ftype=FileType.DIR,
            size=0, owner=self.owner or "ATC" + "0" * 32,
            perms=ATCPermissions(0b111, 0b101, 0b101),
        )
        self.inodes[root_cid]   = root_inode
        self.path_map["/"]      = root_cid

        # Standard-Verzeichnisse
        for dirname in ("home", "contracts", "bin", "tmp", "var", "etc"):
            self.mkdir(f"/{dirname}", self.owner or "ATC" + "0" * 32)

    def _next_fh(self) -> int:
        self._fh_counter += 1
        return self._fh_counter

    def _resolve(self, path: str) -> Optional[str]:
        """Pfad → CID auflösen."""
        path = path.rstrip("/") or "/"
        return self.path_map.get(path)

    def _parent_path(self, path: str) -> str:
        parts = path.rstrip("/").rsplit("/", 1)
        return parts[0] or "/"

    # ── Verzeichnis-Operationen ───────────────────────────

    def mkdir(self, path: str, owner: str) -> Optional[str]:
        """Verzeichnis erstellen."""
        if self._resolve(path):
            return self._resolve(path)   # Existiert bereits

        name    = path.rsplit("/", 1)[-1]
        cid     = atc_content_id(f"dir:{path}:{time.time()}".encode())
        inode   = INode(cid=cid, name=name, ftype=FileType.DIR,
                        size=0, owner=owner)
        self.inodes[cid]  = inode
        self.path_map[path] = cid

        # In Parent eintragen
        parent_path = self._parent_path(path)
        parent_cid  = self._resolve(parent_path)
        if parent_cid and parent_cid in self.inodes:
            self.inodes[parent_cid].children.append(cid)

        return cid

    def listdir(self, path: str) -> List[dict]:
        """Verzeichnis-Inhalt auflisten."""
        cid = self._resolve(path)
        if not cid or cid not in self.inodes:
            return []
        inode = self.inodes[cid]
        if inode.ftype != FileType.DIR:
            return []
        result = []
        for child_cid in inode.children:
            if child_cid in self.inodes:
                result.append(self.inodes[child_cid].to_dict())
        # Auch über path_map suchen
        for p, c in self.path_map.items():
            if p.startswith(path.rstrip("/") + "/") and "/" not in p[len(path.rstrip("/")):].lstrip("/"):
                if c in self.inodes and c not in [r["cid"] for r in result]:
                    result.append(self.inodes[c].to_dict())
        return result

    # ── Datei-Operationen ─────────────────────────────────

    def write_file(self, path: str, data: bytes, owner: str,
                   ftype: FileType = FileType.FILE) -> str:
        """Datei schreiben — gibt CID zurück."""
        cid  = atc_content_id(data)
        name = path.rsplit("/", 1)[-1]
        ext  = "." + name.rsplit(".", 1)[-1] if "." in name else ""

        # FileType aus Extension
        if ext == ".atcb":  ftype = FileType.CONTRACT
        elif ext == ".atc": ftype = FileType.FILE

        # INode erstellen/aktualisieren
        if path in self.path_map:
            old_cid = self.path_map[path]
            if old_cid in self.inodes:
                self.inodes[old_cid].modified = int(time.time() * 1000)
                self.inodes[old_cid].size     = len(data)
                self.inodes[old_cid].cid      = cid
                old_cid_key = old_cid
                self.inodes[cid] = self.inodes.pop(old_cid_key)
        else:
            inode = INode(cid=cid, name=name, ftype=ftype,
                          size=len(data), owner=owner)
            self.inodes[cid] = inode

        self.content[cid]   = data
        self.path_map[path] = cid

        # In Parent eintragen
        parent_path = self._parent_path(path)
        parent_cid  = self._resolve(parent_path)
        if parent_cid and parent_cid in self.inodes:
            if cid not in self.inodes[parent_cid].children:
                self.inodes[parent_cid].children.append(cid)

        return cid

    def read_file(self, path: str) -> Optional[bytes]:
        """Datei lesen."""
        cid = self._resolve(path)
        if not cid: return None
        return self.content.get(cid)

    def delete(self, path: str) -> bool:
        """Datei oder leeres Verzeichnis löschen."""
        cid = self._resolve(path)
        if not cid: return False
        inode = self.inodes.get(cid)
        if inode and inode.ftype == FileType.DIR and inode.children:
            return False  # Nicht leer
        self.inodes.pop(cid, None)
        self.content.pop(cid, None)
        self.path_map.pop(path, None)
        return True

    def stat(self, path: str) -> Optional[dict]:
        """Datei-Metadaten."""
        cid = self._resolve(path)
        if not cid or cid not in self.inodes:
            return None
        return self.inodes[cid].to_dict()

    def exists(self, path: str) -> bool:
        return self._resolve(path) is not None

    # ── File-Handle API (für Kernel) ──────────────────────

    def open(self, path: str, mode: OpenMode, pid: int = 0) -> Optional[FileHandle]:
        cid = self._resolve(path)
        if not cid and mode == OpenMode.READ:
            return None
        if not cid and (mode == OpenMode.WRITE or mode == OpenMode.APPEND):
            # Erstellen
            self.write_file(path, b"", "")
            cid = self._resolve(path)

        fh = FileHandle(fh_id=self._next_fh(), cid=cid, mode=mode, pid=pid)
        self.handles[fh.fh_id] = fh
        return fh

    def fread(self, fh: FileHandle, length: int = -1) -> bytes:
        data = self.content.get(fh.cid, b"")
        if length == -1:
            chunk = data[fh.pos:]
        else:
            chunk = data[fh.pos:fh.pos + length]
        fh.pos += len(chunk)
        return chunk

    def fwrite(self, fh: FileHandle, data: bytes) -> int:
        existing = self.content.get(fh.cid, b"")
        if fh.mode == OpenMode.APPEND:
            new_data = existing + data
        else:
            new_data = existing[:fh.pos] + data + existing[fh.pos + len(data):]
        self.content[fh.cid] = new_data
        if fh.cid in self.inodes:
            self.inodes[fh.cid].size     = len(new_data)
            self.inodes[fh.cid].modified = int(time.time() * 1000)
        fh.pos += len(data)
        return len(data)

    def close(self, fh: FileHandle) -> bool:
        return bool(self.handles.pop(fh.fh_id, None))

    # ── ATCFS-Pfad Format ─────────────────────────────────

    def format_path(self, path: str, node_id: str) -> str:
        """atcfs://node_id/cid/path"""
        cid = self._resolve(path) or "?"
        return f"atcfs://{node_id[:16]}/{cid[:16]}/{path.lstrip('/')}"

    # ── Stats ─────────────────────────────────────────────

    def stats(self) -> dict:
        total_size = sum(len(v) for v in self.content.values())
        return {
            "files":    len([i for i in self.inodes.values() if i.ftype == FileType.FILE]),
            "dirs":     len([i for i in self.inodes.values() if i.ftype == FileType.DIR]),
            "contracts":len([i for i in self.inodes.values() if i.ftype == FileType.CONTRACT]),
            "total_kb": total_size // 1024,
            "handles":  len(self.handles),
        }
