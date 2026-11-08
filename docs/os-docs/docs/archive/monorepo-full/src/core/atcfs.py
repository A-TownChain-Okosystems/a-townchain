# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
ATCFS — Content-adressiertes Dateisystem fuer A-TownChain OS (Issue #23/#26/#54)

Eigene Content-Adressierung (SHA3-256, Praefix "atc1"), einfache
Owner-basierte Zugriffskontrolle und Manifest-Export fuer On-Chain-Anchoring.
"""
import hashlib
import time
from dataclasses import dataclass, field
from typing import Dict, List, Optional, Tuple


def atc_content_id(data: bytes) -> str:
    h = hashlib.sha3_256()
    h.update(b"atcfs_v1||")
    h.update(data)
    return "atc1" + h.hexdigest()


@dataclass
class ATCNode:
    path: str
    name: str
    is_dir: bool
    owner: str
    content_cid: str = ""
    size: int = 0
    created: float = field(default_factory=time.time)
    modified: float = field(default_factory=time.time)
    children: List[str] = field(default_factory=list)  # Kind-Pfade (nur Verzeichnisse)


class ATCFileSystem:
    """Content-adressiertes Dateisystem mit Owner-Zugriffskontrolle."""

    def __init__(self, owner: str = "system"):
        self.owner = owner
        self.nodes: Dict[str, ATCNode] = {}
        self.content: Dict[str, bytes] = {}
        self._init_root()

    def _init_root(self):
        self.nodes["/"] = ATCNode(path="/", name="/", is_dir=True, owner=self.owner)
        for d in ("atc", "home", "tmp", "bin", "var"):
            self._mkdir(f"/{d}", self.owner)

    def _parent(self, path: str) -> str:
        parts = path.rstrip("/").rsplit("/", 1)
        return parts[0] or "/"

    def _mkdir(self, path: str, actor: str):
        if path in self.nodes:
            return
        parent_path = self._parent(path)
        if parent_path not in self.nodes:
            self._mkdir(parent_path, actor)
        node = ATCNode(path=path, name=path.rsplit("/", 1)[-1], is_dir=True, owner=actor)
        self.nodes[path] = node
        self.nodes[parent_path].children.append(path)

    def exists(self, path: str) -> bool:
        path = path.rstrip("/") or "/"
        return path in self.nodes

    def _check_read(self, node: ATCNode, actor: str):
        # Owner darf immer; sonst nur wenn Datei nicht als privat (Standard: Owner-only fuer /home)
        if node.owner == actor or actor == "system":
            return
        if node.path.startswith("/home/") or node.path.startswith("/tmp/") is False and node.owner != actor:
            # Restriktiv: alles ausserhalb von oeffentlichen Pfaden (/atc, /tmp) ist Owner-only
            pass
        public_prefixes = ("/atc/", "/tmp/")
        if node.path.startswith(public_prefixes):
            return
        raise PermissionError(f"'{actor}' hat keinen Lesezugriff auf '{node.path}' (Owner: {node.owner})")

    def write(self, path: str, data: bytes, actor: str) -> ATCNode:
        path = path.rstrip("/")
        parent_path = self._parent(path)
        if parent_path not in self.nodes:
            self._mkdir(parent_path, actor)

        cid = atc_content_id(data)
        self.content[cid] = data
        existing = self.nodes.get(path)
        node = ATCNode(
            path=path, name=path.rsplit("/", 1)[-1], is_dir=False,
            owner=existing.owner if existing else actor,
            content_cid=cid, size=len(data), modified=time.time(),
            created=existing.created if existing else time.time(),
        )
        self.nodes[path] = node
        if path not in self.nodes[parent_path].children:
            self.nodes[parent_path].children.append(path)
        return node

    def read(self, path: str, actor: str) -> Tuple[str, ATCNode]:
        path = path.rstrip("/")
        node = self.nodes.get(path)
        if node is None:
            raise FileNotFoundError(path)
        self._check_read(node, actor)
        return node.content_cid, node

    def ls(self, path: str) -> List[ATCNode]:
        path = path.rstrip("/") or "/"
        node = self.nodes.get(path)
        if node is None or not node.is_dir:
            return []
        return [self.nodes[c] for c in node.children if c in self.nodes]

    def export_manifest(self) -> dict:
        """Exportiert einen Merkle-artigen Root-Hash + Metadaten fuer On-Chain-Anchoring."""
        entries = sorted(p for p in self.nodes if not self.nodes[p].is_dir)
        combined = "|".join(f"{p}:{self.nodes[p].content_cid}" for p in entries)
        root_hash = hashlib.sha256(combined.encode()).hexdigest()
        return {
            "root_hash": root_hash,
            "entries": len(entries),
            "generated_at": time.time(),
        }
