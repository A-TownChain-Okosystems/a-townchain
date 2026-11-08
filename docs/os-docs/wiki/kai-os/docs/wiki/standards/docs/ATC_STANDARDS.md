# ATC Standards — A-TownChain Core Standards
Version: 1.0.0 | Status: DRAFT | Datum: 2026-06-06
Autor: ShivaCore / Aurora

> Alle ATC-Standards sind eigenständig definiert.
> Keine Übernahme aus ERC/EIP oder anderen Blockchains.

---

## ATC-0001 — Core Identity Standard

Jede Entität im A-TownChain Ökosystem besitzt eine eindeutige Identität.

```
IDENTITY := {
    id:       Address,       // ATC-Adresse (35 Zeichen, Präfix "ATC")
    pubkey:   Bytes64,       // Öffentlicher Schlüssel (eigenes ATC-EC)
    created:  UInt64,        // Block-Nummer der Erstellung
    type:     IdentityType,  // WALLET | CONTRACT | NODE | AGENT
}
IdentityType := WALLET | CONTRACT | NODE | AGENT | VALIDATOR
```

---

## ATC-0002 — Wallet Address Format

```
PREFIX    := "ATC"
BODY      := SHA3_ATC(pubkey)[0:32]  // 32 Hex-Zeichen
CHECKSUM  := CRC8(PREFIX + BODY)[0:2]
ADDRESS   := PREFIX + BODY + CHECKSUM  // Gesamt: 37 Zeichen

Beispiel: ATC7a3f9e2b1c8d4a6e0f5b7c9d2e1f3a4b5c6d7e8f9a
```

---

## ATC-0003 — Transaction Schema

```
TX := {
    id:        Hash256,        // SHA3_ATC(Payload)
    from:      Address,        // Sender
    to:        Address,        // Empfänger
    value:     UInt256,        // Menge (in ATCoin, 18 Dezimalstellen)
    data:      Bytes,          // Contract-Calldata (optional)
    nonce:     UInt64,         // Anti-Replay
    gas_limit: UInt64,         // Max. Gas
    gas_price: UInt64,         // Gas-Preis in nano-ATC
    signature: ATCSig,         // ECDSA-Signatur (ATC-EC Kurve)
    timestamp: UInt64,         // Unix-Timestamp
    version:   UInt8 = 1,      // TX-Version
}

ATCSig := {
    r: Bytes32,
    s: Bytes32,
    v: UInt8,   // Recovery ID (0 oder 1)
}
```

---

## ATC-0004 — Block Format

```
BLOCK := {
    header:  BlockHeader,
    txs:     List<TX>,
    receipts: List<TXReceipt>,
}

BlockHeader := {
    version:      UInt8,
    height:       UInt64,
    timestamp:    UInt64,
    prev_hash:    Hash256,
    tx_root:      Hash256,    // Merkle-Root aller TXs
    state_root:   Hash256,    // State-Trie-Root
    receipt_root: Hash256,
    validator:    Address,    // Block-Produzent
    consensus:    ConsensusData,
    nonce:        UInt64,     // PoW-Nonce
    difficulty:   UInt256,
    gas_limit:    UInt64,
    gas_used:     UInt64,
    extra:        Bytes32,    // Freies Feld
}

TXReceipt := {
    tx_id:    Hash256,
    success:  Bool,
    gas_used: UInt64,
    logs:     List<EventLog>,
    error:    Optional<String>,
}
```

---

## ATC-0005 — ShivaConsensus Protocol

Hybrider Mechanismus: PoW + PoS + PoH (Proof of History)

```
Runde := {
    phase_1: PoH_Tick      // Zeitstempel-Kette (VDF-basiert)
    phase_2: PoS_Vote      // Validator-Voting (Stake-gewichtet)
    phase_3: PoW_Seal      // Finaler Hash-Beweis
}

BLOCK_TIME     := 3s
MIN_STAKE      := 1000 ATC
VALIDATOR_SET  := Top-100 nach Stake
FINALITY       := 2/3 Validator-Bestätigung
FORK_RULE      := Längste-gewichtete-Kette (Stake × Länge)
```

---

## ATC-0006 — Smart Contract Interface

```
CONTRACT := {
    address:   Address,
    code_hash: Hash256,     // Hash des Bytecodes
    abi:       List<FuncSpec>,
    state:     StateTree,   // Merkle-Patricia-Trie
    version:   UInt8,
    standard:  ATCStandard, // ATC-8300, ATC-9000, etc.
}

FuncSpec := {
    name:      String,
    inputs:    List<ParamSpec>,
    outputs:   List<ParamSpec>,
    mutates:   Bool,         // ändert State?
    payable:   Bool,         // akzeptiert ATC?
}
```

---

## ATC-0007 — P2P Message Protocol (ATCNet)

```
MSG := {
    version:   UInt8 = 1,
    type:      MsgType,
    from:      NodeID,      // Pubkey-Hash des Senders
    to:        NodeID,      // Ziel (oder broadcast = 0x00)
    payload:   Bytes,
    timestamp: UInt64,
    signature: ATCSig,
    ttl:       UInt8,       // Time-To-Live (Hops)
}

MsgType :=
    HELLO | PING | PONG |
    GET_BLOCKS | BLOCKS |
    GET_TX | TX |
    GET_STATE | STATE |
    CONSENSUS_VOTE | CONSENSUS_BLOCK |
    KI_QUERY | KI_RESPONSE |
    PEER_LIST | DISCONNECT
```

---

## ATC-0008 — KI-OS Process Standard

```
KI_PROCESS := {
    pid:       UInt32,
    name:      String,
    type:      ProcessType,    // AGENT | SERVICE | CONTRACT | SYSTEM
    model:     AIModelRef,     // Referenz auf KI-Modell
    memory:    MemoryRegion,   // isolierter Speicherbereich
    channels:  List<Channel>,  // IPC-Kanäle
    stake:     UInt256,        // ATC-Stake für Rechenrecht
    priority:  UInt8,          // 0=niedrig, 255=System
}

ProcessType := AGENT | SERVICE | CONTRACT | SYSTEM | VALIDATOR
```

---

## ATC-8300 — Fungible Token Standard

```
INTERFACE ATC8300 {
    fn total_supply() -> UInt256
    fn balance_of(owner: Address) -> UInt256
    fn transfer(to: Address, amount: UInt256) -> Bool
    fn approve(spender: Address, amount: UInt256) -> Bool
    fn allowance(owner: Address, spender: Address) -> UInt256
    fn transfer_from(from: Address, to: Address, amount: UInt256) -> Bool

    event Transfer(from: Address, to: Address, amount: UInt256)
    event Approval(owner: Address, spender: Address, amount: UInt256)
}
```

---

## ATC-9000 — NFT Standard (Shivamon)

```
INTERFACE ATC9000 {
    fn owner_of(token_id: UInt256) -> Address
    fn token_uri(token_id: UInt256) -> String
    fn transfer(to: Address, token_id: UInt256) -> Bool
    fn approve(to: Address, token_id: UInt256) -> Bool
    fn mint(to: Address, metadata: ATCMetadata) -> UInt256
    fn burn(token_id: UInt256) -> Bool
    fn total_supply() -> UInt256

    event Mint(to: Address, token_id: UInt256)
    event Burn(token_id: UInt256)
    event Transfer(from: Address, to: Address, token_id: UInt256)
}

ATCMetadata := {
    name:        String,
    description: String,
    image_uri:   String,        // ATCFS:// Pfad
    attributes:  Map<String, String>,
    generation:  UInt8,
    rarity:      RarityLevel,   // COMMON | RARE | EPIC | LEGENDARY
}
```
