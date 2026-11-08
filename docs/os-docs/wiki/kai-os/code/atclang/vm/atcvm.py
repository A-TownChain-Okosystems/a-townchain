# Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
"""
ATCLang VM — Stack-basierte virtuelle Maschine
Führt ATCLang Bytecode aus
Version: 0.1.0-alpha
"""

from enum import IntEnum, auto
from dataclasses import dataclass, field
from typing import List, Any, Dict, Optional


# ── Opcodes ──────────────────────────────────────────────
class OP(IntEnum):
    # Stack-Operationen
    PUSH       = auto()   # PUSH <wert>
    POP        = auto()   # POP
    DUP        = auto()   # Stack-Top duplizieren
    SWAP       = auto()   # Top zwei tauschen

    # Arithmetik
    ADD        = auto()
    SUB        = auto()
    MUL        = auto()
    DIV        = auto()
    MOD        = auto()
    NEG        = auto()   # Negation

    # Vergleiche
    EQ         = auto()   # ==
    NEQ        = auto()   # !=
    LT         = auto()   # <
    GT         = auto()   # >
    LTE        = auto()   # <=
    GTE        = auto()   # >=

    # Logik
    AND        = auto()
    OR         = auto()
    NOT        = auto()

    # Variablen
    LOAD       = auto()   # LOAD <name>
    STORE      = auto()   # STORE <name>
    LOAD_IDX   = auto()   # Map/List[key]
    STORE_IDX  = auto()   # Map/List[key] = val

    # Sprünge
    JUMP       = auto()   # JUMP <offset>
    JUMP_IF    = auto()   # JUMP wenn True
    JUMP_NOT   = auto()   # JUMP wenn False

    # Funktionen
    CALL       = auto()   # CALL <name> <argc>
    RETURN     = auto()   # RETURN
    CALL_EXT   = auto()   # Externe ATC-Funktion

    # ATC-spezifisch
    EMIT       = auto()   # Event auslösen
    REQUIRE    = auto()   # Assertion
    TRANSFER   = auto()   # ATC-Transfer
    MINT       = auto()   # Token minten
    BURN       = auto()   # Token burnen

    # Objekte
    NEW_MAP    = auto()   # {} erstellen
    NEW_LIST   = auto()   # [] erstellen
    GET_FIELD  = auto()   # obj.field
    SET_FIELD  = auto()   # obj.field = val

    # System
    HALT       = auto()   # Ausführung stoppen
    NOP        = auto()   # Keine Operation
    PRINT      = auto()   # Debug-Ausgabe


@dataclass
class Instruction:
    op:   OP
    args: List[Any] = field(default_factory=list)

    def __repr__(self):
        if self.args:
            return f"{self.op.name:<12} {' '.join(str(a) for a in self.args)}"
        return self.op.name


@dataclass
class ATCFunction:
    name:         str
    params:       List[str]
    instructions: List[Instruction]
    local_vars:   Dict[str, Any] = field(default_factory=dict)


@dataclass
class CallFrame:
    func:    ATCFunction
    ip:      int = 0
    locals:  Dict[str, Any] = field(default_factory=dict)
    ret_val: Any = None


class ATCVMError(Exception):
    pass


class RequireError(ATCVMError):
    pass


# ── Virtuelle Maschine ───────────────────────────────────
class ATCVM:
    """
    Stack-basierte VM für ATCLang.
    Führt Bytecode-Instruktionen aus.
    """

    def __init__(self):
        self.stack:        List[Any]            = []
        self.globals:      Dict[str, Any]       = {}
        self.functions:    Dict[str, ATCFunction] = {}
        self.call_stack:   List[CallFrame]      = []
        self.events:       List[dict]           = []  # emittierte Events
        self.gas_used:     int                  = 0
        self.gas_limit:    int                  = 1_000_000
        self._setup_builtins()

    def _setup_builtins(self):
        """ATC-Standardfunktionen registrieren"""
        self.globals.update({
            'caller':   'ATC0000000000000000000000000000000000',
            'block_num': 0,
            'tx_hash':  '0x' + '0' * 64,
            'now':       0,
        })

    def push(self, val: Any):
        self.stack.append(val)

    def pop(self) -> Any:
        if not self.stack:
            raise ATCVMError("Stack underflow")
        return self.stack.pop()

    def peek_stack(self) -> Any:
        if not self.stack:
            raise ATCVMError("Stack leer")
        return self.stack[-1]

    def gas(self, cost: int = 1):
        self.gas_used += cost
        if self.gas_used > self.gas_limit:
            raise ATCVMError(f"Gas-Limit überschritten: {self.gas_used} > {self.gas_limit}")

    def get_var(self, name: str, frame: Optional[CallFrame]) -> Any:
        if frame and name in frame.locals:
            return frame.locals[name]
        if name in self.globals:
            return self.globals[name]
        raise ATCVMError(f"Variable nicht definiert: '{name}'")

    def set_var(self, name: str, value: Any, frame: Optional[CallFrame]):
        if frame:
            frame.locals[name] = value
        else:
            self.globals[name] = value

    def execute(self, instructions: List[Instruction], frame: Optional[CallFrame] = None) -> Any:
        ip = 0
        while ip < len(instructions):
            instr = instructions[ip]
            self.gas()

            op = instr.op

            if op == OP.NOP:
                pass

            elif op == OP.HALT:
                break

            elif op == OP.PUSH:
                self.push(instr.args[0])

            elif op == OP.POP:
                self.pop()

            elif op == OP.DUP:
                self.push(self.peek_stack())

            elif op == OP.SWAP:
                a, b = self.pop(), self.pop()
                self.push(a); self.push(b)

            # Arithmetik
            elif op == OP.ADD:
                b, a = self.pop(), self.pop()
                self.push(a + b)
            elif op == OP.SUB:
                b, a = self.pop(), self.pop()
                self.push(a - b)
            elif op == OP.MUL:
                b, a = self.pop(), self.pop()
                self.push(a * b)
            elif op == OP.DIV:
                b, a = self.pop(), self.pop()
                if b == 0: raise ATCVMError("Division durch Null")
                self.push(a // b)
            elif op == OP.MOD:
                b, a = self.pop(), self.pop()
                self.push(a % b)
            elif op == OP.NEG:
                self.push(-self.pop())

            # Vergleiche
            elif op == OP.EQ:  b, a = self.pop(), self.pop(); self.push(a == b)
            elif op == OP.NEQ: b, a = self.pop(), self.pop(); self.push(a != b)
            elif op == OP.LT:  b, a = self.pop(), self.pop(); self.push(a < b)
            elif op == OP.GT:  b, a = self.pop(), self.pop(); self.push(a > b)
            elif op == OP.LTE: b, a = self.pop(), self.pop(); self.push(a <= b)
            elif op == OP.GTE: b, a = self.pop(), self.pop(); self.push(a >= b)

            # Logik
            elif op == OP.AND: b, a = self.pop(), self.pop(); self.push(bool(a) and bool(b))
            elif op == OP.OR:  b, a = self.pop(), self.pop(); self.push(bool(a) or bool(b))
            elif op == OP.NOT: self.push(not self.pop())

            # Variablen
            elif op == OP.LOAD:
                name = instr.args[0]
                self.push(self.get_var(name, frame))
            elif op == OP.STORE:
                name  = instr.args[0]
                value = self.pop()
                self.set_var(name, value, frame)
            elif op == OP.LOAD_IDX:
                key = self.pop(); obj = self.pop()
                try: self.push(obj[key])
                except: self.push(None)
            elif op == OP.STORE_IDX:
                val = self.pop(); key = self.pop(); obj = self.pop()
                obj[key] = val; self.push(obj)

            # Sprünge
            elif op == OP.JUMP:
                ip = instr.args[0]; continue
            elif op == OP.JUMP_IF:
                if self.pop(): ip = instr.args[0]; continue
            elif op == OP.JUMP_NOT:
                if not self.pop(): ip = instr.args[0]; continue

            # Funktionen
            elif op == OP.CALL:
                fname = instr.args[0]; argc = instr.args[1] if len(instr.args) > 1 else 0
                args = [self.pop() for _ in range(argc)][::-1]
                if fname in self.functions:
                    func = self.functions[fname]
                    new_frame = CallFrame(func)
                    for pname, pval in zip(func.params, args):
                        new_frame.locals[pname] = pval
                    result = self.execute(func.instructions, new_frame)
                    self.push(result)
                else:
                    raise ATCVMError(f"Funktion nicht gefunden: '{fname}'")

            elif op == OP.RETURN:
                return self.pop() if self.stack else None

            # ATC-spezifisch
            elif op == OP.EMIT:
                event_name = instr.args[0]; argc = instr.args[1] if len(instr.args) > 1 else 0
                event_args = [self.pop() for _ in range(argc)][::-1]
                self.events.append({'event': event_name, 'args': event_args})
                print(f"  📡 Event: {event_name}({', '.join(str(a) for a in event_args)})")

            elif op == OP.REQUIRE:
                condition = self.pop()
                msg = instr.args[0] if instr.args else "Require fehlgeschlagen"
                if not condition:
                    raise RequireError(f"require() fehlgeschlagen: {msg}")

            elif op == OP.PRINT:
                val = self.pop()
                print(f"  [ATCLang] {val}")

            elif op == OP.NEW_MAP:
                self.push({})
            elif op == OP.NEW_LIST:
                self.push([])

            elif op == OP.GET_FIELD:
                field_name = instr.args[0]; obj = self.pop()
                if isinstance(obj, dict):
                    self.push(obj.get(field_name))
                else:
                    self.push(getattr(obj, field_name, None))

            elif op == OP.SET_FIELD:
                val = self.pop(); obj = self.pop()
                field_name = instr.args[0]
                if isinstance(obj, dict): obj[field_name] = val
                else: setattr(obj, field_name, val)
                self.push(obj)

            else:
                raise ATCVMError(f"Unbekannter Opcode: {op}")

            ip += 1

        return self.pop() if self.stack else None

    def register_function(self, func: ATCFunction):
        self.functions[func.name] = func

    def run_program(self, instructions: List[Instruction]) -> Any:
        self.stack.clear()
        return self.execute(instructions)

    def get_events(self) -> List[dict]:
        return self.events.copy()

    def reset(self):
        self.stack.clear()
        self.events.clear()
        self.gas_used = 0


# ── Test ─────────────────────────────────────────────────
if __name__ == "__main__":
    vm = ATCVM()

    # Einfaches Beispiel: balance[addr] += amount
    program = [
        Instruction(OP.PUSH,  [1000]),          # balance start
        Instruction(OP.STORE, ['balance']),
        Instruction(OP.LOAD,  ['balance']),
        Instruction(OP.PUSH,  [250]),
        Instruction(OP.ADD),
        Instruction(OP.STORE, ['balance']),
        Instruction(OP.LOAD,  ['balance']),
        Instruction(OP.PRINT),
        Instruction(OP.EMIT,  ['Transfer', 2]),  # braucht 2 args auf Stack
    ]

    # Transfer-Event vorbereiten
    vm.stack = ['ATC_SENDER', 'ATC_RECEIVER']
    result = vm.run_program(program)
    print(f"\n✅ ATCVM läuft | Gas verbraucht: {vm.gas_used} | Events: {len(vm.get_events())}")
