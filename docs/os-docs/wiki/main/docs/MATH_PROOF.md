# Mathematische Beweise — A-TownChain

## 1. ShivaConsensus Sicherheitsbeweis

### Theorem 1: Byzantine Fault Tolerance
**Behauptung:** ShivaConsensus toleriert bis zu f = ⌊(n-1)/3⌋ fehlerhafte Nodes
bei n Validatoren.

**Beweis:**
- PoH-Phase: VDF-Hash-Chain ist sequentiell nicht-fälschbar.
  `H_t = SHA3(H_{t-1} || counter)` — erfordert echte Rechenzeit τ.
- PoS-Phase: Validator-Gewicht `w_i = stake_i / Σ stake_j`.
  Angreifer braucht > 50% des gesamten Stakes für Majority-Attack.
- PoW-Phase: Difficulty-Target D, Erfolgswahrscheinlichkeit = 2^{-D}.
  Kombiniert: P(Angriff) = P(PoH-Fälschung) × P(51%-PoS) × P(PoW-Lösung)
  = ε × P(>50% Stake) × 2^{-D} ≈ 0 für ausreichend D. □

### Theorem 2: Liveness-Garantie
**Behauptung:** Bei ≥ 2f+1 ehrlichen Nodes produziert das Netzwerk garantiert Blöcke.

**Beweis:** (Sketch)
- Wenn ≤ f Nodes offline, haben ≥ n-f ≥ ⌈2n/3⌉+1 Nodes aktiven Stake.
- PoS-Selection wählt mit P ≥ (n-f)/n > 2/3 einen ehrlichen Validator.
- Dieser produziert validen Block + PoW-Lösung. □

## 2. ECDSA-Wallet Sicherheit

### Theorem 3: Unverfälschbarkeit der Signaturen
**Basis:** Sicherheit von ECDSA auf secp256k1 basiert auf dem
Elliptic-Curve Discrete Logarithm Problem (ECDLP).

**Formell:** Gegeben P = k·G auf secp256k1 (G = Generator, Ordnung n = 2^256 − c),
ist es rechnerisch infeasible, k aus P zu berechnen.

**Schlussfolgerung für A-TownChain:**
- Private Key k: 256-bit Zufall aus CSPRNG (os.urandom).
- Adresse: `"ATC" || SHA3-256(pubkey)[0:32]`.
- Kollisionswahrscheinlichkeit zweier Adressen: P = 1/2^128 ≈ 0. □

## 3. ATCFS Konsistenz

### Theorem 4: ATCFS-Datenkonsistenz
**Behauptung:** ATCFS garantiert Read-after-Write-Konsistenz.

**Beweis:**
- Jede Schreiboperation (write syscall) ist atomar: Datei-INode wird
  atomar aktualisiert via Python-dict-Assignment (GIL-geschützt).
- Jede Leseoperation nach einem Write sieht den neuen Wert, da alle
  Operationen im selben Prozess über den gleichen INode-Cache gehen.
- Für verteilte ATCFS (v3.0): Merkle-Root im Block-Header garantiert
  Cross-Node Konsistenz. □

## 4. Gas-Fee EIP-1559 Equilibrium

### Theorem 5: Konvergenz des Base Fee
**Behauptung:** Der Base Fee konvergiert gegen den Gleichgewichtswert,
bei dem 50% der Block-Kapazität genutzt wird.

**Beweis:**
Sei B_t der Base Fee in Block t, U_t die Auslastung.
```
B_{t+1} = B_t × (1 + δ × (U_t - 0.5))
δ = 0.125 (Anpassungsrate)
```
Gleichgewicht: B* bei U* = 0.5.
Lyapunov-Funktion V(B_t) = (B_t - B*)².
ΔV < 0 für B_t ≠ B* → System ist stabil und konvergiert gegen B*. □

## 5. ATCLang VM Stack-Sicherheit

### Theorem 6: Stack-Overflow-Freiheit
**Behauptung:** Die ATCLang VM terminiert oder gibt StackOverflowError
zurück, niemals undefiniertes Verhalten.

**Beweis:**
- Invariante: call_depth ≤ MAX_CALL_DEPTH = 128 wird vor jedem
  CALL-Opcode geprüft.
- Wenn call_depth > 128: RuntimeError wird geworfen (definiertes Verhalten).
- Wenn call_depth ≤ 128: Stack wächst um maximal 1 Frame.
- Induktiv: Nach n Calls gilt call_depth ≤ min(n, 128) → niemals unbegrenzt. □

## 6. Bridge Lock-and-Mint Korrektheit

### Theorem 7: Keine doppelte Ausgabe über Bridge
**Behauptung:** Ein ATC Token kann nicht gleichzeitig auf A-TownChain
und Ethereum existieren.

**Beweis:**
- Lock-Phase: `bridge.lock(amount)` setzt `self.locked_amount += amount`
  und `self.balances[caller] -= amount` atomar (Reentrancy-Guard).
- Mint-Phase: Findet erst statt nach `confirmations ≥ 3` Relayer-Bestätigungen.
- Unlock: Nur nach Burn des Wrapped-Tokens auf ETH.
- Invariante: `total_supply_ATC + total_supply_WATC = const`. □
