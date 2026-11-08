import React, { useState, useEffect, useRef, useMemo } from 'react';
import {
  Play,
  Pause,
  RotateCcw,
  SkipForward,
  Cpu,
  Zap,
  Terminal,
  FileCode,
  ShieldCheck,
  Flame,
  Binary,
  Layers,
  Sparkles,
  Download,
  Copy,
  Check,
  Search,
  Database,
  Hash,
  AlertTriangle,
  HelpCircle,
  Activity,
  ArrowRight,
  BookOpen,
  Code
} from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';

export interface VmInstruction {
  lineNum: number;
  raw: string;
  opcode: string;
  args: string[];
  comment?: string;
}

export interface VmState {
  pc: number;
  registers: number[]; // R0 to R255
  memory: Uint8Array;  // 64 KB linear memory
  callStack: number[]; // Return addresses
  gasUsed: number;
  gasLimit: number;
  status: 'READY' | 'RUNNING' | 'PAUSED' | 'HALTED' | 'PANIC' | 'OUT_OF_GAS';
  panicMessage?: string;
  events: { topic: string; data: string; step: number }[];
  stepCount: number;
}

// Preset assembly contracts
const PRESET_CONTRACTS: { id: string; name: string; category: string; description: string; code: string }[] = [
  {
    id: 'token-transfer',
    name: 'ATC-20 Token Transfer with Capability Gate',
    category: 'Smart Contract',
    description: 'Verifies sender capability token, checks balance, subtracts sender balance, credits recipient and emits Transfer event.',
    code: `; ==============================================================
; ATC-20 Multi-Token Transfer with Capability Gate (ATC-0004)
; ==============================================================
; R1: Sender Account Address Pointer
; R2: Recipient Account Address Pointer
; R3: Transfer Amount (in base units)
; R4: Sender Current Balance
; R5: Recipient Current Balance
; R6: Capability Scope Token ID

; 1. Setup registers with initial mock state
LOAD_CONST R1, 100       ; Sender address index (0x64)
LOAD_CONST R2, 200       ; Recipient address index (0xC8)
LOAD_CONST R3, 250       ; Transfer Amount = 250 ATC
LOAD_CONST R4, 1000      ; Initial Sender Balance = 1000 ATC
LOAD_CONST R5, 120       ; Initial Recipient Balance = 120 ATC
LOAD_CONST R6, 42        ; Cap Scope: 0x2A (TRANSFER_CAP)

; 2. Security: Verify capability to mutate sender state
CAP_CHECK 1, R6          ; Capability Check: Type 1 (TransferCap), Scope R6
JNE R0, R0, cap_error    ; Guard: R0 is always zero

; 3. Balance verification
JLT R4, R3, underflow    ; If Sender Balance < Transfer Amount -> Underflow

; 4. Execute arithmetic transfer
SUB R4, R4, R3           ; Sender Balance = R4 - R3 (1000 - 250 = 750)
ADD R5, R5, R3           ; Recipient Balance = R5 + R3 (120 + 250 = 370)

; 5. Persist to linear memory (Simulated ShivaFS state slot)
MEM_STORE_64 [R1 + 0], R4 ; Store 750 at memory[100]
MEM_STORE_64 [R2 + 0], R5 ; Store 370 at memory[200]

; 6. Emit Blockchain Audit Event (topic: 0x01 = Transfer, data: amount)
SYS_EVENT 1, R3          ; Emit Transfer(R3=250)

; 7. Normal termination
HALT

cap_error:
PANIC 403                ; Error 403: Capability Denied

underflow:
PANIC 400                ; Error 400: Insufficient Balance
`
  },
  {
    id: 'recursive-fib',
    name: 'Recursive Fibonacci (Call-Stack Benchmark)',
    category: 'Algorithm',
    description: 'Calculates Fibonacci number using recursive function calls to demonstrate stack frames, register preservation and JMP/RET.',
    code: `; ==============================================================
; Recursive Fibonacci Generator (Testing Call-Stack & CALL/RET)
; ==============================================================
; Input: R1 = n (calculate Fib(n))
; Output: R2 = result

LOAD_CONST R1, 7         ; Calculate Fib(7) -> Expected: 13
CALL fib_start
HALT                     ; Result is in R2

fib_start:
; Base Case: if n <= 1 return n
LOAD_CONST R7, 1
JGT R1, R7, rec_case
MOV R2, R1               ; Result = n
RET

rec_case:
; Save current n on simulated memory stack
LOAD_CONST R10, 500      ; Stack pointer base
MEM_STORE_BYTE [R10 + 0], R1

; Calculate Fib(n - 1)
SUB R1, R1, R7           ; n = n - 1
CALL fib_start
MOV R8, R2               ; R8 = Fib(n - 1)

; Restore n and calculate Fib(n - 2)
LOAD_CONST R10, 500
MEM_LOAD_BYTE R1, [R10 + 0]
LOAD_CONST R7, 2
SUB R1, R1, R7           ; n = n - 2

; Save R8
LOAD_CONST R10, 501
MEM_STORE_BYTE [R10 + 0], R8

CALL fib_start           ; R2 = Fib(n - 2)

; Result = Fib(n - 1) + Fib(n - 2)
LOAD_CONST R10, 501
MEM_LOAD_BYTE R8, [R10 + 0]
ADD R2, R8, R2
RET
`
  },
  {
    id: 'poseidon-hasher',
    name: 'Poseidon Cryptographic Merkle Leaf Hasher',
    category: 'Cryptography',
    description: 'Computes state root hash of two account balances using the zero-knowledge-friendly Poseidon permutation opcode.',
    code: `; ==============================================================
; Poseidon Merkle State Root Hasher (ATC-0094 Zero-Knowledge Primitive)
; ==============================================================
; R1: Account A Balance Leaf
; R2: Account B Balance Leaf
; R3: Poseidon Output Hash (Digest 1)
; R4: Root Nonce
; R5: Final State Root

LOAD_CONST R1, 1500000   ; 1.50 ATC
LOAD_CONST R2, 9820000   ; 9.82 ATC
LOAD_CONST R4, 1337      ; Epoch Nonce

; Hash(R1, R2) -> R3
CRYPTO_POSEIDON R3, R1, R2

; Combine Hash with Root Nonce
CRYPTO_POSEIDON R5, R3, R4

; Store State Root into ShivaFS Header Slot
LOAD_CONST R8, 32
MEM_STORE_64 [R8 + 0], R5

; Log event for block producer
SYS_EVENT 99, R5
HALT
`
  },
  {
    id: 'ans-resolver',
    name: 'ANS Domain Name Resolution (ATC-0002)',
    category: 'Protocol',
    description: 'Computes the 256-bit identifier for alice.atc, checks domain expiry and returns the resolved did:atc principal address.',
    code: `; ==============================================================
; A-Town Naming Service (ANS) Resolver (ATC-0002)
; ==============================================================
; Simulates domain resolution: "alice.atc" -> Principal ID
LOAD_CONST R1, 12        ; Domain String Length: 9 chars
LOAD_CONST R2, 100       ; Pointer to string in memory
LOAD_CONST R3, 2026      ; Current Year / Expiry Check
LOAD_CONST R4, 2030      ; Domain Expiry Date

; Check if expired (Expiry > Current)
JLT R4, R3, domain_expired

; Compute Blake3 domain hash
CRYPTO_BLAKE3 R5, R2, R1 ; R5 = Hash("alice.atc")

; Resolve Owner Principal (Mock DID 0x41544301)
LOAD_CONST R6, 1096041217
SYS_EVENT 2, R6          ; Emit ANSResolved(R6)
HALT

domain_expired:
PANIC 404                ; ANS_DOMAIN_EXPIRED
`
  }
];

// Opcode gas costs
const OP_GAS_COSTS: Record<string, number> = {
  LOAD_CONST: 2,
  MOV: 1,
  ADD: 2,
  SUB: 2,
  MUL: 5,
  DIV: 5,
  MOD: 5,
  AND: 2,
  OR: 2,
  XOR: 2,
  NOT: 2,
  SHL: 2,
  SHR: 2,
  JMP: 3,
  JEQ: 4,
  JNE: 4,
  JLT: 4,
  JGT: 4,
  CALL: 12,
  RET: 8,
  MEM_STORE_64: 20,
  MEM_LOAD_64: 15,
  MEM_STORE_BYTE: 10,
  MEM_LOAD_BYTE: 8,
  CAP_CHECK: 15,
  SYS_EVENT: 25,
  CRYPTO_POSEIDON: 45,
  CRYPTO_BLAKE3: 60,
  GAS_REMAINING: 2,
  HALT: 1,
  PANIC: 10,
};

interface AtcVmSimulatorPanelProps {
  onSaveWorkspaceFile?: (fileName: string, content: string) => void;
  onShowToast?: (msg: string, type: 'info' | 'warning' | 'error') => void;
}

export function AtcVmSimulatorPanel({
  onSaveWorkspaceFile,
  onShowToast
}: AtcVmSimulatorPanelProps) {
  const [selectedPreset, setSelectedPreset] = useState<string>('token-transfer');
  const [sourceCode, setSourceCode] = useState<string>(PRESET_CONTRACTS[0].code);
  const [breakpoints, setBreakpoints] = useState<Set<number>>(new Set());
  const [clockSpeed, setClockSpeed] = useState<'step' | 'slow' | 'fast' | 'turbo'>('fast');
  const [registerDisplayFormat, setRegisterDisplayFormat] = useState<'hex' | 'dec'>('dec');
  const [memoryPage, setMemoryPage] = useState<number>(0); // 0 = 0..255, 1 = 256..511
  const [regSearchFilter, setRegSearchFilter] = useState<string>('');
  const [copied, setCopied] = useState<boolean>(false);

  // VM internal state
  const [vm, setVm] = useState<VmState>(() => ({
    pc: 0,
    registers: new Array(256).fill(0),
    memory: new Uint8Array(65536),
    callStack: [],
    gasUsed: 0,
    gasLimit: 1000000,
    status: 'READY',
    events: [],
    stepCount: 0,
  }));

  const [modifiedRegs, setModifiedRegs] = useState<Set<number>>(new Set());
  const [modifiedMem, setModifiedMem] = useState<Set<number>>(new Set());

  const timerRef = useRef<NodeJS.Timeout | null>(null);

  // Parse lines into executable instructions
  const parsedInstructions = useMemo<VmInstruction[]>(() => {
    const lines = sourceCode.split('\n');
    const result: VmInstruction[] = [];

    lines.forEach((rawLine, idx) => {
      const lineNum = idx + 1;
      let clean = rawLine.trim();
      let comment: string | undefined;

      const commentIdx = clean.indexOf(';');
      if (commentIdx >= 0) {
        comment = clean.substring(commentIdx + 1).trim();
        clean = clean.substring(0, commentIdx).trim();
      }

      if (!clean) {
        // Empty or comment-only line
        result.push({ lineNum, raw: rawLine, opcode: '', args: [], comment });
        return;
      }

      // Check for labels (e.g. "loop:")
      if (clean.endsWith(':')) {
        result.push({ lineNum, raw: rawLine, opcode: 'LABEL', args: [clean.slice(0, -1)], comment });
        return;
      }

      const parts = clean.split(/\s+/);
      const opcode = parts[0].toUpperCase();
      const rest = clean.substring(opcode.length).trim();
      const args = rest ? rest.split(',').map((a) => a.trim()) : [];

      result.push({ lineNum, raw: rawLine, opcode, args, comment });
    });

    return result;
  }, [sourceCode]);

  // Label index mapping
  const labelMap = useMemo<Map<string, number>>(() => {
    const map = new Map<string, number>();
    parsedInstructions.forEach((inst, index) => {
      if (inst.opcode === 'LABEL' && inst.args[0]) {
        map.set(inst.args[0], index);
      }
    });
    return map;
  }, [parsedInstructions]);

  // Helper to parse register name (R0..R255)
  const parseReg = (regStr: string): number => {
    const match = regStr.match(/^R?(\d+)$/i);
    if (!match) return 0;
    const r = parseInt(match[1], 10);
    return Math.min(Math.max(r, 0), 255);
  };

  // Helper to parse memory expression: [R1 + 0] or [100]
  const parseMemoryTarget = (expr: string, regs: number[]): number => {
    const stripped = expr.replace(/[\[\]]/g, '').trim();
    if (stripped.includes('+')) {
      const [basePart, offPart] = stripped.split('+').map((s) => s.trim());
      const baseVal = basePart.startsWith('R') ? regs[parseReg(basePart)] : parseInt(basePart, 10) || 0;
      const offVal = parseInt(offPart, 10) || 0;
      return (baseVal + offVal) & 0xffff;
    }
    if (stripped.startsWith('R')) {
      return regs[parseReg(stripped)] & 0xffff;
    }
    return (parseInt(stripped, 10) || 0) & 0xffff;
  };

  // Reset VM
  const handleResetVm = () => {
    if (timerRef.current) clearInterval(timerRef.current);
    setVm({
      pc: 0,
      registers: new Array(256).fill(0),
      memory: new Uint8Array(65536),
      callStack: [],
      gasUsed: 0,
      gasLimit: 1000000,
      status: 'READY',
      events: [],
      stepCount: 0,
    });
    setModifiedRegs(new Set());
    setModifiedMem(new Set());
    onShowToast?.('ATC-VM State & Register Bank zurückgesetzt.', 'info');
  };

  // Single step execution function
  const stepExecution = (): boolean => {
    let shouldContinue = true;

    setVm((prev) => {
      if (prev.status === 'HALTED' || prev.status === 'PANIC' || prev.status === 'OUT_OF_GAS') {
        shouldContinue = false;
        return prev;
      }

      // Find next executable instruction
      let currPc = prev.pc;
      while (currPc < parsedInstructions.length && (!parsedInstructions[currPc].opcode || parsedInstructions[currPc].opcode === 'LABEL')) {
        currPc++;
      }

      if (currPc >= parsedInstructions.length) {
        shouldContinue = false;
        return { ...prev, status: 'HALTED', pc: currPc };
      }

      const inst = parsedInstructions[currPc];
      const op = inst.opcode;
      const args = inst.args;

      const gasCost = OP_GAS_COSTS[op] || 2;
      const newGasUsed = prev.gasUsed + gasCost;

      if (newGasUsed > prev.gasLimit) {
        shouldContinue = false;
        return { ...prev, status: 'OUT_OF_GAS', gasUsed: newGasUsed };
      }

      // Clone mutable registers and memory
      const newRegs = [...prev.registers];
      const newMem = new Uint8Array(prev.memory);
      const newCallStack = [...prev.callStack];
      const newEvents = [...prev.events];
      let nextPc = currPc + 1;
      let newStatus: VmState['status'] = 'RUNNING';
      let panicMsg = prev.panicMessage;

      const updatedRegIndices = new Set<number>();
      const updatedMemIndices = new Set<number>();

      const setReg = (r: number, val: number) => {
        if (r === 0) return; // R0 is immutable zero!
        newRegs[r] = val;
        updatedRegIndices.add(r);
      };

      try {
        switch (op) {
          case 'LOAD_CONST': {
            const rd = parseReg(args[0]);
            const imm = parseInt(args[1], 10) || 0;
            setReg(rd, imm);
            break;
          }
          case 'MOV': {
            const rd = parseReg(args[0]);
            const rs = parseReg(args[1]);
            setReg(rd, newRegs[rs]);
            break;
          }
          case 'ADD': {
            const rd = parseReg(args[0]);
            const rs1 = parseReg(args[1]);
            const rs2 = parseReg(args[2]);
            setReg(rd, newRegs[rs1] + newRegs[rs2]);
            break;
          }
          case 'SUB': {
            const rd = parseReg(args[0]);
            const rs1 = parseReg(args[1]);
            const rs2 = parseReg(args[2]);
            setReg(rd, newRegs[rs1] - newRegs[rs2]);
            break;
          }
          case 'MUL': {
            const rd = parseReg(args[0]);
            const rs1 = parseReg(args[1]);
            const rs2 = parseReg(args[2]);
            setReg(rd, newRegs[rs1] * newRegs[rs2]);
            break;
          }
          case 'DIV': {
            const rd = parseReg(args[0]);
            const rs1 = parseReg(args[1]);
            const rs2 = parseReg(args[2]);
            if (newRegs[rs2] === 0) {
              newStatus = 'PANIC';
              panicMsg = 'Division by zero';
              shouldContinue = false;
            } else {
              setReg(rd, Math.floor(newRegs[rs1] / newRegs[rs2]));
            }
            break;
          }
          case 'MOD': {
            const rd = parseReg(args[0]);
            const rs1 = parseReg(args[1]);
            const rs2 = parseReg(args[2]);
            if (newRegs[rs2] === 0) {
              newStatus = 'PANIC';
              panicMsg = 'Modulo by zero';
              shouldContinue = false;
            } else {
              setReg(rd, newRegs[rs1] % newRegs[rs2]);
            }
            break;
          }
          case 'AND': {
            const rd = parseReg(args[0]);
            const rs1 = parseReg(args[1]);
            const rs2 = parseReg(args[2]);
            setReg(rd, newRegs[rs1] & newRegs[rs2]);
            break;
          }
          case 'OR': {
            const rd = parseReg(args[0]);
            const rs1 = parseReg(args[1]);
            const rs2 = parseReg(args[2]);
            setReg(rd, newRegs[rs1] | newRegs[rs2]);
            break;
          }
          case 'XOR': {
            const rd = parseReg(args[0]);
            const rs1 = parseReg(args[1]);
            const rs2 = parseReg(args[2]);
            setReg(rd, newRegs[rs1] ^ newRegs[rs2]);
            break;
          }
          case 'NOT': {
            const rd = parseReg(args[0]);
            const rs = parseReg(args[1]);
            setReg(rd, ~newRegs[rs]);
            break;
          }
          case 'SHL': {
            const rd = parseReg(args[0]);
            const rs = parseReg(args[1]);
            const imm = parseInt(args[2], 10) || 0;
            setReg(rd, newRegs[rs] << imm);
            break;
          }
          case 'SHR': {
            const rd = parseReg(args[0]);
            const rs = parseReg(args[1]);
            const imm = parseInt(args[2], 10) || 0;
            setReg(rd, newRegs[rs] >>> imm);
            break;
          }
          case 'JMP': {
            const target = args[0];
            const targetIdx = labelMap.get(target);
            if (targetIdx !== undefined) nextPc = targetIdx;
            break;
          }
          case 'JEQ': {
            const rs1 = parseReg(args[0]);
            const rs2 = parseReg(args[1]);
            const target = args[2];
            if (newRegs[rs1] === newRegs[rs2]) {
              const targetIdx = labelMap.get(target);
              if (targetIdx !== undefined) nextPc = targetIdx;
            }
            break;
          }
          case 'JNE': {
            const rs1 = parseReg(args[0]);
            const rs2 = parseReg(args[1]);
            const target = args[2];
            if (newRegs[rs1] !== newRegs[rs2]) {
              const targetIdx = labelMap.get(target);
              if (targetIdx !== undefined) nextPc = targetIdx;
            }
            break;
          }
          case 'JLT': {
            const rs1 = parseReg(args[0]);
            const rs2 = parseReg(args[1]);
            const target = args[2];
            if (newRegs[rs1] < newRegs[rs2]) {
              const targetIdx = labelMap.get(target);
              if (targetIdx !== undefined) nextPc = targetIdx;
            }
            break;
          }
          case 'JGT': {
            const rs1 = parseReg(args[0]);
            const rs2 = parseReg(args[1]);
            const target = args[2];
            if (newRegs[rs1] > newRegs[rs2]) {
              const targetIdx = labelMap.get(target);
              if (targetIdx !== undefined) nextPc = targetIdx;
            }
            break;
          }
          case 'CALL': {
            const target = args[0];
            const targetIdx = labelMap.get(target);
            if (targetIdx !== undefined) {
              newCallStack.push(currPc + 1);
              nextPc = targetIdx;
            }
            break;
          }
          case 'RET': {
            if (newCallStack.length > 0) {
              const retAddr = newCallStack.pop()!;
              nextPc = retAddr;
            } else {
              newStatus = 'HALTED';
              shouldContinue = false;
            }
            break;
          }
          case 'MEM_STORE_64': {
            const addr = parseMemoryTarget(args[0], newRegs);
            const rs = parseReg(args[1]);
            const val = newRegs[rs];
            // Store 4 bytes low to high
            newMem[addr] = val & 0xff;
            newMem[(addr + 1) & 0xffff] = (val >> 8) & 0xff;
            newMem[(addr + 2) & 0xffff] = (val >> 16) & 0xff;
            newMem[(addr + 3) & 0xffff] = (val >> 24) & 0xff;
            updatedMemIndices.add(addr);
            break;
          }
          case 'MEM_LOAD_64': {
            const rd = parseReg(args[0]);
            const addr = parseMemoryTarget(args[1], newRegs);
            const val =
              newMem[addr] |
              (newMem[(addr + 1) & 0xffff] << 8) |
              (newMem[(addr + 2) & 0xffff] << 16) |
              (newMem[(addr + 3) & 0xffff] << 24);
            setReg(rd, val);
            break;
          }
          case 'MEM_STORE_BYTE': {
            const addr = parseMemoryTarget(args[0], newRegs);
            const rs = parseReg(args[1]);
            newMem[addr] = newRegs[rs] & 0xff;
            updatedMemIndices.add(addr);
            break;
          }
          case 'MEM_LOAD_BYTE': {
            const rd = parseReg(args[0]);
            const addr = parseMemoryTarget(args[1], newRegs);
            setReg(rd, newMem[addr]);
            break;
          }
          case 'CAP_CHECK': {
            // Simulated ShivaCore Capability Verification
            // If cap_scope === 42, authorized!
            const capScope = parseReg(args[1]);
            if (newRegs[capScope] !== 42) {
              newStatus = 'PANIC';
              panicMsg = 'Capability Verification Failed (E_CAP_UNAUTHORIZED)';
              shouldContinue = false;
            }
            break;
          }
          case 'SYS_EVENT': {
            const topic = args[0];
            const rs = parseReg(args[1]);
            newEvents.push({
              topic: `Topic#${topic}`,
              data: `Val: ${newRegs[rs]} (0x${newRegs[rs].toString(16)})`,
              step: prev.stepCount + 1,
            });
            break;
          }
          case 'CRYPTO_POSEIDON': {
            const rd = parseReg(args[0]);
            const rs1 = parseReg(args[1]);
            const rs2 = parseReg(args[2]);
            // Simulated algebraic Poseidon hash
            const hashVal = Math.abs((newRegs[rs1] * 7919 + newRegs[rs2] * 65537 + 104729) % 2147483647);
            setReg(rd, hashVal);
            break;
          }
          case 'CRYPTO_BLAKE3': {
            const rd = parseReg(args[0]);
            const rsPtr = parseReg(args[1]);
            const rsLen = parseReg(args[2]);
            const hashVal = Math.abs((newRegs[rsPtr] * 31 + newRegs[rsLen] * 127 + 524287) % 2147483647);
            setReg(rd, hashVal);
            break;
          }
          case 'GAS_REMAINING': {
            const rd = parseReg(args[0]);
            setReg(rd, prev.gasLimit - newGasUsed);
            break;
          }
          case 'HALT': {
            newStatus = 'HALTED';
            shouldContinue = false;
            break;
          }
          case 'PANIC': {
            newStatus = 'PANIC';
            panicMsg = `Panic with code: ${args[0] || '1'}`;
            shouldContinue = false;
            break;
          }
          default:
            // Unknown opcode
            break;
        }
      } catch (err: any) {
        newStatus = 'PANIC';
        panicMsg = err.message || 'Execution exception';
        shouldContinue = false;
      }

      setModifiedRegs(updatedRegIndices);
      setModifiedMem(updatedMemIndices);

      // Check if next instruction is on a breakpoint
      if (breakpoints.has(nextPc + 1)) {
        newStatus = 'PAUSED';
        shouldContinue = false;
      }

      return {
        ...prev,
        pc: nextPc,
        registers: newRegs,
        memory: newMem,
        callStack: newCallStack,
        gasUsed: newGasUsed,
        status: newStatus,
        panicMessage: panicMsg,
        events: newEvents,
        stepCount: prev.stepCount + 1,
      };
    });

    return shouldContinue;
  };

  // Run execution loop with selected clock speed
  const handleStartRun = () => {
    if (vm.status === 'HALTED' || vm.status === 'PANIC' || vm.status === 'OUT_OF_GAS') {
      handleResetVm();
    }

    setVm((prev) => ({ ...prev, status: 'RUNNING' }));

    if (clockSpeed === 'turbo') {
      // Execute 200 steps in batch or until halt
      let steps = 0;
      while (steps < 500) {
        const canContinue = stepExecution();
        steps++;
        if (!canContinue) break;
      }
    } else {
      const delay = clockSpeed === 'slow' ? 300 : clockSpeed === 'fast' ? 50 : 10;
      if (timerRef.current) clearInterval(timerRef.current);
      timerRef.current = setInterval(() => {
        const canContinue = stepExecution();
        if (!canContinue) {
          if (timerRef.current) clearInterval(timerRef.current);
        }
      }, delay);
    }
  };

  const handlePause = () => {
    if (timerRef.current) clearInterval(timerRef.current);
    setVm((prev) => ({ ...prev, status: 'PAUSED' }));
  };

  useEffect(() => {
    return () => {
      if (timerRef.current) clearInterval(timerRef.current);
    };
  }, []);

  const toggleBreakpoint = (lineNum: number) => {
    setBreakpoints((prev) => {
      const next = new Set(prev);
      if (next.has(lineNum)) next.delete(lineNum);
      else next.add(lineNum);
      return next;
    });
  };

  const handleLoadPreset = (presetId: string) => {
    const found = PRESET_CONTRACTS.find((p) => p.id === presetId);
    if (!found) return;
    setSelectedPreset(presetId);
    setSourceCode(found.code);
    handleResetVm();
    setBreakpoints(new Set());
    onShowToast?.(`Preset "${found.name}" in Simulator geladen.`, 'info');
  };

  const handleExportAsm = () => {
    if (onSaveWorkspaceFile) {
      const filename = `contracts/${selectedPreset}.atcasm`;
      onSaveWorkspaceFile(filename, sourceCode);
      onShowToast?.(`Assembly gespeichert als: ${filename}`, 'info');
    }
  };

  const copyCode = () => {
    navigator.clipboard.writeText(sourceCode);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
    onShowToast?.('Assembler-Code in die Zwischenablage kopiert.', 'info');
  };

  // Filtered registers for display
  const filteredRegisters = useMemo(() => {
    return vm.registers.map((val, idx) => ({ idx, val })).filter((item) => {
      if (!regSearchFilter) return true;
      const term = regSearchFilter.toLowerCase();
      return `r${item.idx}`.includes(term) || item.val.toString().includes(term);
    });
  }, [vm.registers, regSearchFilter]);

  return (
    <div className="flex-1 flex flex-col bg-[#0b0f19] text-slate-200 overflow-hidden font-sans">
      {/* Top Header Bar */}
      <div className="flex flex-wrap items-center justify-between px-4 py-3 bg-[#0d1322] border-b border-white/10 gap-3 shrink-0">
        <div className="flex items-center gap-3">
          <div className="w-9 h-9 rounded-lg bg-cyan-500/20 border border-cyan-500/40 flex items-center justify-center text-cyan-400">
            <Cpu className="w-5 h-5" />
          </div>
          <div>
            <div className="flex items-center gap-2">
              <h2 className="text-sm font-bold text-white tracking-wide">ATC-VM Live-Debugger & Bytecode Simulator</h2>
              <span className="px-2 py-0.5 rounded-full text-[10px] font-mono font-bold bg-cyan-500/20 text-cyan-300 border border-cyan-500/40">
                ATC-0094 v1.2
              </span>
              <span className="px-2 py-0.5 rounded-full text-[10px] font-mono bg-indigo-500/20 text-indigo-300 border border-indigo-500/40">
                256 Register • 64KB Mem
              </span>
            </div>
            <p className="text-xs text-slate-400">
              Interaktive Register-VM mit deterministischem Gas-Accounting, linearer Speicherbank & Capability-Gates
            </p>
          </div>
        </div>

        {/* Preset Selector & Action Buttons */}
        <div className="flex items-center gap-2">
          <select
            value={selectedPreset}
            onChange={(e) => handleLoadPreset(e.target.value)}
            className="bg-black/40 border border-white/10 rounded-lg px-2.5 py-1.5 text-xs text-slate-200 outline-none focus:border-cyan-500 cursor-pointer font-medium"
          >
            {PRESET_CONTRACTS.map((p) => (
              <option key={p.id} value={p.id} className="bg-[#0f172a]">
                {p.name}
              </option>
            ))}
          </select>

          <button
            onClick={copyCode}
            className="px-2.5 py-1.5 bg-white/5 hover:bg-white/10 text-slate-300 rounded-lg text-xs font-semibold flex items-center gap-1.5 transition-colors"
            title="Code kopieren"
          >
            {copied ? <Check className="w-3.5 h-3.5 text-emerald-400" /> : <Copy className="w-3.5 h-3.5" />}
            <span>Kopieren</span>
          </button>

          {onSaveWorkspaceFile && (
            <button
              onClick={handleExportAsm}
              className="px-3 py-1.5 bg-indigo-600/30 hover:bg-indigo-600/40 text-indigo-200 border border-indigo-500/40 rounded-lg text-xs font-semibold flex items-center gap-1.5 transition-colors"
            >
              <Download className="w-3.5 h-3.5" />
              <span>In Workspace sichern</span>
            </button>
          )}
        </div>
      </div>

      {/* Execution Control & Metrics Strip */}
      <div className="flex flex-wrap items-center justify-between px-4 py-2 bg-black/40 border-b border-white/10 gap-3 shrink-0 text-xs">
        {/* Playback Controls */}
        <div className="flex items-center gap-2">
          {vm.status === 'RUNNING' ? (
            <button
              onClick={handlePause}
              className="px-3 py-1.5 bg-amber-500/20 hover:bg-amber-500/30 text-amber-300 border border-amber-500/40 rounded-lg font-bold flex items-center gap-1.5 shadow-sm transition-all"
            >
              <Pause className="w-3.5 h-3.5 fill-current" />
              <span>PAUSE</span>
            </button>
          ) : (
            <button
              onClick={handleStartRun}
              className="px-3 py-1.5 bg-emerald-500/20 hover:bg-emerald-500/30 text-emerald-300 border border-emerald-500/40 rounded-lg font-bold flex items-center gap-1.5 shadow-sm transition-all"
            >
              <Play className="w-3.5 h-3.5 fill-current" />
              <span>{vm.status === 'PAUSED' ? 'FORTSETZEN' : 'AUSFÜHREN'}</span>
            </button>
          )}

          <button
            onClick={() => stepExecution()}
            disabled={vm.status === 'HALTED' || vm.status === 'RUNNING'}
            className="px-3 py-1.5 bg-white/5 hover:bg-white/10 disabled:opacity-40 text-slate-300 rounded-lg font-semibold flex items-center gap-1.5 transition-colors"
            title="Einzelschritt (Single Step F10)"
          >
            <SkipForward className="w-3.5 h-3.5" />
            <span>STEP (1 Inst)</span>
          </button>

          <button
            onClick={handleResetVm}
            className="px-3 py-1.5 bg-white/5 hover:bg-white/10 text-slate-400 hover:text-red-400 rounded-lg font-semibold flex items-center gap-1.5 transition-colors"
            title="VM zurücksetzen"
          >
            <RotateCcw className="w-3.5 h-3.5" />
            <span>RESET</span>
          </button>

          <div className="h-4 w-px bg-white/10 mx-1" />

          {/* Speed selector */}
          <div className="flex items-center gap-1 bg-white/5 p-1 rounded-lg border border-white/5">
            {(['slow', 'fast', 'turbo'] as const).map((spd) => (
              <button
                key={spd}
                onClick={() => setClockSpeed(spd)}
                className={`px-2 py-0.5 rounded text-[11px] font-semibold capitalize transition-all ${
                  clockSpeed === spd ? 'bg-cyan-500/20 text-cyan-300 border border-cyan-500/30' : 'text-slate-400 hover:text-slate-200'
                }`}
              >
                {spd}
              </button>
            ))}
          </div>
        </div>

        {/* Live Metrics */}
        <div className="flex items-center gap-4 text-xs font-mono">
          <div className="flex items-center gap-1.5 bg-white/5 px-2.5 py-1 rounded-md border border-white/5">
            <span className="text-slate-400">PC:</span>
            <span className="font-bold text-cyan-400">0x{vm.pc.toString(16).padStart(4, '0')} ({vm.pc})</span>
          </div>

          <div className="flex items-center gap-1.5 bg-white/5 px-2.5 py-1 rounded-md border border-white/5">
            <span className="text-slate-400">Gas:</span>
            <span className="font-bold text-amber-400">
              {vm.gasUsed.toLocaleString()} / {vm.gasLimit.toLocaleString()}
            </span>
          </div>

          <div className="flex items-center gap-1.5 bg-white/5 px-2.5 py-1 rounded-md border border-white/5">
            <span className="text-slate-400">Stack:</span>
            <span className="font-bold text-indigo-400">{vm.callStack.length} Tief</span>
          </div>

          <div className="flex items-center gap-1.5 bg-white/5 px-2.5 py-1 rounded-md border border-white/5">
            <span className="text-slate-400">Status:</span>
            <span
              className={`font-bold uppercase ${
                vm.status === 'RUNNING'
                  ? 'text-emerald-400 animate-pulse'
                  : vm.status === 'HALTED'
                  ? 'text-cyan-400'
                  : vm.status === 'PANIC' || vm.status === 'OUT_OF_GAS'
                  ? 'text-red-400'
                  : 'text-amber-400'
              }`}
            >
              {vm.status}
            </span>
          </div>
        </div>
      </div>

      {/* Main Grid: Code Editor (Left) | Registers (Center) | Memory & Events (Right) */}
      <div className="flex-1 grid grid-cols-12 gap-0 overflow-hidden">
        {/* LEFT COLUMN: Assembly Code & Breakpoints (5 cols) */}
        <div className="col-span-12 lg:col-span-5 border-r border-white/10 flex flex-col bg-[#080d1a] overflow-hidden">
          <div className="px-3 py-2 bg-[#0c1222] border-b border-white/10 flex items-center justify-between text-xs">
            <span className="font-bold text-slate-300 flex items-center gap-1.5">
              <FileCode className="w-3.5 h-3.5 text-cyan-400" />
              <span>ATCASM Bytecode Editor</span>
            </span>
            <span className="text-[11px] text-slate-500 font-mono">
              {parsedInstructions.length} Zeilen • Klick auf Zeilennummer für Breakpoint
            </span>
          </div>

          <div className="flex-1 overflow-y-auto font-mono text-xs p-2 select-text">
            {parsedInstructions.map((inst, idx) => {
              const isCurrentPc = vm.pc === idx;
              const hasBreakpoint = breakpoints.has(inst.lineNum);

              return (
                <div
                  key={idx}
                  className={`flex items-center gap-2 px-1.5 py-0.5 rounded transition-colors group ${
                    isCurrentPc
                      ? 'bg-cyan-500/20 border-l-2 border-cyan-400 font-bold text-white'
                      : 'hover:bg-white/5 text-slate-300'
                  }`}
                >
                  {/* Breakpoint toggle button */}
                  <button
                    onClick={() => toggleBreakpoint(inst.lineNum)}
                    className="w-4 h-4 rounded-full flex items-center justify-center text-[10px] cursor-pointer"
                  >
                    {hasBreakpoint ? (
                      <div className="w-2.5 h-2.5 rounded-full bg-red-500 shadow-[0_0_8px_rgba(239,68,68,0.8)]" />
                    ) : (
                      <div className="w-1.5 h-1.5 rounded-full bg-slate-700 opacity-0 group-hover:opacity-100" />
                    )}
                  </button>

                  {/* Line number */}
                  <span
                    onClick={() => toggleBreakpoint(inst.lineNum)}
                    className={`w-6 text-right cursor-pointer select-none font-mono text-[10px] ${
                      hasBreakpoint ? 'text-red-400 font-bold' : 'text-slate-600'
                    }`}
                  >
                    {inst.lineNum}
                  </span>

                  {/* PC pointer arrow */}
                  <div className="w-3">
                    {isCurrentPc && <ArrowRight className="w-3 h-3 text-cyan-400 animate-pulse" />}
                  </div>

                  {/* Code line */}
                  <div className="flex-1 whitespace-pre truncate">
                    {inst.opcode === 'LABEL' ? (
                      <span className="text-yellow-400 font-bold">{inst.args[0]}:</span>
                    ) : inst.opcode ? (
                      <>
                        <span className="text-cyan-300 font-semibold">{inst.opcode}</span>{' '}
                        <span className="text-slate-300">{inst.args.join(', ')}</span>
                      </>
                    ) : (
                      <span className="text-slate-600">{inst.raw}</span>
                    )}

                    {inst.comment && (
                      <span className="text-slate-500 ml-2 italic">; {inst.comment}</span>
                    )}
                  </div>
                </div>
              );
            })}
          </div>

          {/* Panic banner if triggered */}
          {vm.panicMessage && (
            <div className="p-3 bg-red-950/80 border-t border-red-800/50 flex items-center gap-2 text-xs text-red-200">
              <AlertTriangle className="w-4 h-4 text-red-400 shrink-0" />
              <div>
                <span className="font-bold text-red-300">VM HALTED (PANIC): </span>
                <span>{vm.panicMessage}</span>
              </div>
            </div>
          )}
        </div>

        {/* CENTER COLUMN: 256 Registers Bank (4 cols) */}
        <div className="col-span-12 lg:col-span-4 border-r border-white/10 flex flex-col bg-[#090d18] overflow-hidden">
          <div className="px-3 py-2 bg-[#0c1222] border-b border-white/10 flex items-center justify-between text-xs">
            <span className="font-bold text-slate-300 flex items-center gap-1.5">
              <Binary className="w-3.5 h-3.5 text-indigo-400" />
              <span>Registers (R0 - R255)</span>
            </span>

            <div className="flex items-center gap-2">
              <button
                onClick={() => setRegisterDisplayFormat(registerDisplayFormat === 'hex' ? 'dec' : 'hex')}
                className="px-2 py-0.5 rounded bg-white/5 hover:bg-white/10 text-[10px] font-mono text-cyan-300 border border-white/5"
              >
                {registerDisplayFormat.toUpperCase()}
              </button>
            </div>
          </div>

          {/* Filter / Search for registers */}
          <div className="px-3 py-1.5 border-b border-white/5 bg-black/20 flex items-center gap-2">
            <Search className="w-3 h-3 text-slate-500" />
            <input
              type="text"
              value={regSearchFilter}
              onChange={(e) => setRegSearchFilter(e.target.value)}
              placeholder="Filter (z.B. R1, R4 oder Wert)..."
              className="bg-transparent text-xs text-slate-200 outline-none w-full placeholder-slate-600"
            />
          </div>

          {/* Registers Grid */}
          <div className="flex-1 overflow-y-auto p-2">
            <div className="grid grid-cols-2 sm:grid-cols-2 gap-1.5 font-mono text-xs">
              {filteredRegisters.map(({ idx, val }) => {
                const isModified = modifiedRegs.has(idx);
                const isZeroReg = idx === 0;

                return (
                  <div
                    key={idx}
                    className={`flex items-center justify-between px-2 py-1 rounded border transition-all ${
                      isModified
                        ? 'bg-amber-500/20 border-amber-500/40 text-amber-200'
                        : val !== 0
                        ? 'bg-indigo-500/10 border-indigo-500/20 text-indigo-200'
                        : 'bg-white/[0.02] border-white/5 text-slate-500'
                    }`}
                  >
                    <span className={`font-bold ${isZeroReg ? 'text-slate-500' : 'text-cyan-400'}`}>
                      R{idx}
                    </span>
                    <span className="font-semibold text-right truncate">
                      {registerDisplayFormat === 'hex'
                        ? `0x${(val >>> 0).toString(16).toUpperCase()}`
                        : val.toLocaleString()}
                    </span>
                  </div>
                );
              })}
            </div>
          </div>
        </div>

        {/* RIGHT COLUMN: 64KB Linear Memory & Blockchain Event Log (3 cols) */}
        <div className="col-span-12 lg:col-span-3 flex flex-col bg-[#080d1a] overflow-hidden">
          {/* Top Half: Memory Hex Inspector */}
          <div className="flex-1 flex flex-col border-b border-white/10 overflow-hidden">
            <div className="px-3 py-2 bg-[#0c1222] border-b border-white/10 flex items-center justify-between text-xs">
              <span className="font-bold text-slate-300 flex items-center gap-1.5">
                <Database className="w-3.5 h-3.5 text-purple-400" />
                <span>Linear Memory (64 KB)</span>
              </span>

              {/* Memory page selector */}
              <div className="flex items-center gap-1 text-[10px] font-mono">
                <button
                  disabled={memoryPage === 0}
                  onClick={() => setMemoryPage((p) => Math.max(0, p - 1))}
                  className="px-1.5 py-0.5 rounded bg-white/5 hover:bg-white/10 disabled:opacity-30"
                >
                  ◀
                </button>
                <span className="text-slate-400">
                  0x{(memoryPage * 256).toString(16).padStart(4, '0')}
                </span>
                <button
                  disabled={memoryPage >= 255}
                  onClick={() => setMemoryPage((p) => p + 1)}
                  className="px-1.5 py-0.5 rounded bg-white/5 hover:bg-white/10 disabled:opacity-30"
                >
                  ▶
                </button>
              </div>
            </div>

            <div className="flex-1 overflow-y-auto p-2 font-mono text-[11px] leading-tight">
              {Array.from({ length: 16 }).map((_, row) => {
                const baseAddr = memoryPage * 256 + row * 16;
                const bytes = Array.from({ length: 16 }).map((_, col) => vm.memory[baseAddr + col]);

                return (
                  <div key={row} className="flex items-center gap-2 py-0.5 hover:bg-white/5 rounded px-1">
                    <span className="text-slate-600 select-none">
                      0x{baseAddr.toString(16).padStart(4, '0')}:
                    </span>
                    <div className="flex items-center gap-1 text-slate-300">
                      {bytes.map((b, col) => {
                        const addr = baseAddr + col;
                        const isMod = modifiedMem.has(addr);
                        return (
                          <span
                            key={col}
                            className={`w-4 text-center ${
                              isMod
                                ? 'text-amber-400 font-bold bg-amber-500/20 rounded'
                                : b !== 0
                                ? 'text-cyan-300 font-medium'
                                : 'text-slate-600'
                            }`}
                          >
                            {b.toString(16).padStart(2, '0')}
                          </span>
                        );
                      })}
                    </div>
                  </div>
                );
              })}
            </div>
          </div>

          {/* Bottom Half: Blockchain Event Log */}
          <div className="h-44 flex flex-col bg-[#050811] overflow-hidden">
            <div className="px-3 py-2 bg-[#0a0f1d] border-b border-white/10 flex items-center justify-between text-xs">
              <span className="font-bold text-slate-300 flex items-center gap-1.5">
                <Flame className="w-3.5 h-3.5 text-amber-400" />
                <span>Blockchain Event Log ({vm.events.length})</span>
              </span>
            </div>

            <div className="flex-1 overflow-y-auto p-2 font-mono text-[11px] space-y-1">
              {vm.events.length === 0 ? (
                <div className="text-slate-600 text-center py-6 italic">
                  Keine Events emittiert. SYS_EVENT ausführen.
                </div>
              ) : (
                vm.events.map((ev, i) => (
                  <div key={i} className="p-1.5 rounded bg-white/5 border border-white/5 flex items-start justify-between">
                    <div>
                      <span className="text-amber-400 font-bold">{ev.topic}</span>
                      <div className="text-slate-300 text-[10px]">{ev.data}</div>
                    </div>
                    <span className="text-[9px] text-slate-500">Step #{ev.step}</span>
                  </div>
                ))
              )}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
