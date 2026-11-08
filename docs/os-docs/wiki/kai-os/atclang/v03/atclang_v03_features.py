# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
ATCLang v0.3.0 — Erweiterte Sprach-Features (Fix #35)
async/await | Generics | Closures | Module-System | Type Inference
"""
from __future__ import annotations
from dataclasses import dataclass, field
from typing import Any, Callable, Dict, List, Optional
import asyncio, time

# ── Neue Token-Typen für v0.3.0 ────────────────────
NEW_TOKENS_V03 = {
    "async", "await", "fn_async", "import", "export",
    "module", "type", "where", "impl", "trait",
    "closure", "lambda", "match", "yield",
}

# ── Generics-System ────────────────────────────────
@dataclass
class GenericType:
    """Repräsentiert einen generischen Typ: Vec<T>, Map<K,V>, Option<T>"""
    base:       str
    params:     List[str]

    def resolve(self, bindings: Dict[str, str]) -> str:
        resolved = [bindings.get(p, p) for p in self.params]
        return f"{self.base}<{', '.join(resolved)}>"

    def __str__(self):
        return f"{self.base}<{', '.join(self.params)}>"

class GenericsResolver:
    """Löst generische Typ-Parameter auf (Monomorphisierung)."""

    def __init__(self):
        self.instances: Dict[str, List[str]] = {}

    def instantiate(self, generic_fn: str, type_args: List[str]) -> str:
        key = f"{generic_fn}_{'-'.join(type_args)}"
        self.instances[key] = type_args
        return key

    def get_all_instances(self) -> Dict[str, List[str]]:
        return dict(self.instances)

# ── Closure-System ─────────────────────────────────
@dataclass
class Closure:
    """
    ATCLang Closure: |param1, param2| body_expression

    Beispiel (ATCLang):
        let double = |x: u128| safe_mul(x, 2)
        let add    = |a: u128, b: u128| safe_add(a, b)
    """
    params:      List[str]
    body:        str
    captured:    Dict[str, Any] = field(default_factory=dict)
    return_type: Optional[str]  = None

    def call(self, *args) -> Any:
        env = dict(zip(self.params, args))
        env.update(self.captured)
        return self._eval(self.body, env)

    def _eval(self, expr: str, env: Dict[str, Any]) -> Any:
        # Einfacher Expression-Evaluator für Closures
        import re
        expr = expr.strip()
        # Variable-Lookup
        for name, val in env.items():
            expr = expr.replace(name, str(val))
        # safe_mul / safe_add inline
        expr = re.sub(r'safe_mul\((\d+),\s*(\d+)\)',
                      lambda m: str(int(m.group(1)) * int(m.group(2))), expr)
        expr = re.sub(r'safe_add\((\d+),\s*(\d+)\)',
                      lambda m: str(int(m.group(1)) + int(m.group(2))), expr)
        try:    return eval(expr, {"__builtins__": {}})
        except: return expr

class ClosureFactory:
    """Erstellt und verwaltet Closures aus ATCLang-Source."""

    @staticmethod
    def parse(source: str) -> Optional[Closure]:
        """
        Parst: |param1: T1, param2: T2| body
        """
        import re
        m = re.match(r'\|([^|]*)\|\s*(.+)', source.strip())
        if not m: return None
        params_raw = m.group(1).strip()
        body       = m.group(2).strip()
        params = []
        for p in params_raw.split(','):
            p = p.strip()
            if ':' in p:
                name = p.split(':')[0].strip()
            else:
                name = p
            if name: params.append(name)
        return Closure(params=params, body=body)

# ── Async/Await Runtime ────────────────────────────
class AsyncRuntime:
    """
    ATCLang async/await Runtime.
    Erlaubt nicht-blockierende Contract-Calls über asyncio.

    ATCLang-Syntax:
        async fn fetch_price(token: string) -> u128 {
            let result = await oracle.get_price(token)
            return result
        }
    """

    def __init__(self):
        self.pending_tasks: Dict[str, asyncio.Task] = {}
        self.results:       Dict[str, Any] = {}

    async def run_async_fn(self, fn_name: str,
                           fn: Callable, *args) -> Any:
        """Führt async ATCLang-Funktion aus."""
        task_id = f"{fn_name}_{int(time.time()*1000)}"
        result  = await asyncio.coroutine(fn)(*args) if asyncio.iscoroutinefunction(fn) else fn(*args)
        self.results[task_id] = result
        return result

    @staticmethod
    async def atc_sleep(ms: int):
        """await sleep(ms) in ATCLang."""
        await asyncio.sleep(ms / 1000)

    @staticmethod
    async def atc_timeout(coro, timeout_ms: int):
        """await mit Timeout."""
        try:
            return await asyncio.wait_for(coro, timeout=timeout_ms/1000)
        except asyncio.TimeoutError:
            raise RuntimeError(f"Timeout nach {timeout_ms}ms")

# ── Module-System ──────────────────────────────────
@dataclass
class ATCModule:
    """
    ATCLang Module-System.
    Syntax: import ATC::Gaming::Shivamon
            import ATC::Crypto::sha256 as hash
    """
    namespace:  str
    exports:    Dict[str, Any] = field(default_factory=dict)
    imports:    List[str]      = field(default_factory=list)

    def export(self, name: str, value: Any):
        self.exports[name] = value

    def get(self, name: str) -> Any:
        return self.exports.get(name)

class ModuleRegistry:
    """Globale Registry aller ATCLang-Module."""
    _modules: Dict[str, ATCModule] = {}

    @classmethod
    def register(cls, namespace: str, module: ATCModule):
        cls._modules[namespace] = module

    @classmethod
    def resolve(cls, import_path: str) -> Optional[ATCModule]:
        """Löst 'ATC::Gaming::Shivamon' → Module auf."""
        parts   = import_path.split("::")
        current = "::".join(parts)
        while current:
            if current in cls._modules:
                return cls._modules[current]
            parts   = parts[:-1]
            current = "::".join(parts)
        return None

    @classmethod
    def list_all(cls) -> List[str]:
        return list(cls._modules.keys())

# ── Type Inference ─────────────────────────────────
class TypeInference:
    """
    Einfache Typ-Inferenz für ATCLang v0.3.0.
    'let x = 42'   → x: u64
    'let s = "hi"' → s: string
    'let b = true' → b: bool
    """

    @staticmethod
    def infer(value: Any) -> str:
        if isinstance(value, bool):   return "bool"
        if isinstance(value, int):
            if value < 0:             return "i128"
            if value <= 255:          return "u8"
            if value <= 65535:        return "u16"
            if value <= 2**32-1:      return "u32"
            if value <= 2**64-1:      return "u64"
            return "u128"
        if isinstance(value, str):
            if value.startswith("ATC") and len(value) == 35:
                return "Address"
            return "string"
        if isinstance(value, list):   return "Vec<auto>"
        if isinstance(value, dict):   return "Map<auto,auto>"
        return "unknown"

    @staticmethod
    def infer_from_literal(literal: str) -> str:
        """Inferiert Typ aus ATCLang-Literal-String."""
        literal = literal.strip()
        if literal in ("true", "false"): return "bool"
        if literal.startswith('"'):      return "string"
        if literal.startswith("ATC") and len(literal) == 35: return "Address"
        if literal.isdigit():
            v = int(literal)
            if v <= 255:        return "u8"
            if v <= 65535:      return "u16"
            if v <= 2**32-1:    return "u32"
            if v <= 2**64-1:    return "u64"
            return "u128"
        if literal.startswith("["):      return "Vec<auto>"
        if literal.startswith("{"):      return "Map<auto,auto>"
        return "unknown"

# ── String-Interpolation ──────────────────────────
def atc_format(template: str, env: Dict[str, Any]) -> str:
    """
    ATCLang String-Interpolation: f"Balance: {self.balance} ATC"
    """
    import re
    def replace(m):
        key = m.group(1).strip()
        return str(env.get(key, f"{{{key}}}"))
    return re.sub(r'\{([^}]+)\}', replace, template)

# ── Error-Reporter v0.3.0 ─────────────────────────
@dataclass
class ATCError:
    """Strukturierter Fehler mit Zeile + Spalte + Kontext."""
    code:    str
    message: str
    line:    int
    col:     int
    source:  str = ""

    def format(self) -> str:
        lines   = self.source.split("\n") if self.source else []
        context = lines[self.line - 1] if 0 < self.line <= len(lines) else ""
        pointer = " " * (self.col - 1) + "^"
        return (
            f"\n[{self.code}] Zeile {self.line}, Spalte {self.col}: {self.message}\n"
            f"  {context}\n"
            f"  {pointer}\n"
        )

class ErrorReporter:
    """Sammelt und formatiert ATCLang-Compiler-Fehler."""

    def __init__(self):
        self.errors:   List[ATCError] = []
        self.warnings: List[ATCError] = []

    def error(self, code: str, msg: str, line: int, col: int, src: str = ""):
        self.errors.append(ATCError(code, msg, line, col, src))

    def warn(self, code: str, msg: str, line: int, col: int, src: str = ""):
        self.warnings.append(ATCError(code, msg, line, col, src))

    def has_errors(self) -> bool:
        return len(self.errors) > 0

    def format_all(self) -> str:
        out = ""
        for e in self.errors:   out += f"ERROR{e.format()}"
        for w in self.warnings: out += f"WARN {w.format()}"
        return out

# ── ATCLang v0.3.0 Feature-Summary ────────────────
ATCLANG_V03_FEATURES = {
    "version":    "0.3.0",
    "new_tokens": sorted(NEW_TOKENS_V03),
    "features": [
        "async/await — nicht-blockierende Contract-Calls",
        "Generics — fn foo<T>(x: T) → T, Vec<T>, Map<K,V>",
        "Closures — |x: u128| safe_mul(x, 2)",
        "Module-System — import ATC::Gaming::Shivamon",
        "Type Inference — let x = 42 (kein expliziter Typ)",
        "String-Interpolation — f'Balance: {self.balance} ATC'",
        "Error-Reporter — Zeile + Spalte + Code-Kontext",
        "WASM-Kompilierung (experimentell, v0.4.0)",
    ],
    "breaking_changes": [
        "MAX_CALL_DEPTH bleibt 128 (auch für async)",
        "Module-Namespaces sind case-sensitive",
    ],
    "backwards_compatible": True,
}
