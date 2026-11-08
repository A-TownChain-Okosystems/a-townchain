# ATC-SCF-001 — Native ATC Smart Contract Format

**Status:** SPECIFIED  
**Format:** `.atc`  
**Scope:** Native A-TownChain Smart Contracts  
**Execution target:** ATC-VM

## 1. Purpose

The `.atc` format is the native source format for A-TownChain smart contracts. It defines an independent contract syntax, type model, metadata directives, standard bindings, capabilities, policies and deterministic compilation contract.

The format is not defined as an extension of another smart-contract language.

## 2. Canonical pipeline

```
.atc source
  -> lexer
  -> parser
  -> ATC AST
  -> semantic analysis
  -> ATC IR
  -> ATC-VM bytecode
  -> ATC-VM
```

The canonical source remains the `.atc` file. Compilation must be deterministic for identical source, compiler and specification inputs.

## 3. Header directives

A contract may declare:

```atc
@version 1.0
@standard ATC-001
@standard ATC-005
@type NFT721
@name "ATownAvatar"
@symbol "ATCA"
```

### 3.1 @version

Declares the ATC source-format version.

### 3.2 @standard

Binds the contract to a registered ATC standard. Multiple declarations are permitted.

### 3.3 @type

Declares a native ATC contract or asset type.

Initial registered examples include `NFT721`, `NFT1155`, `Token20`, `Token777`, `Governance`, `Oracle`, `Bridge`, `Rental` and `Hybrid`.

### 3.4 @name

Declares the human-readable contract or asset name.

### 3.5 @symbol

Declares an optional asset symbol.

## 4. Native contract structure

The normative contract model consists of:

- contract declaration
- state
- constants
- events
- errors
- functions
- capabilities
- policies
- asset definitions
- access rules

Example:

```atc
@version 1.0
@standard ATC-001
@type NFT721
@name "ATownAvatar"
@symbol "ATCA"

contract ATownAvatar {
    state {
        next_id: u256 = 1;
    }

    @capability mint
    function mint(to: Address, metadata: String) -> TokenId {
        let id = next_id;
        next_id = next_id + 1;
        mint(to, id);
        set_metadata(id, metadata);
        return id;
    }
}
```

## 5. Native types

The format uses the ATC type system. Contract-facing types include:

`bool`, `u8`, `u16`, `u32`, `u64`, `u128`, `u256`, `i8`, `i16`, `i32`, `i64`, `i128`, `i256`, `Address`, `Hash256`, `Bytes`, `String`, `TokenId`, `ContractId`, `ChainId`, `Timestamp` and `BlockHeight`.

The canonical language specification remains the authority for lexical and core language rules.

## 6. Capabilities

Capabilities explicitly classify privileged contract operations.

```atc
@capability mint
function mint(...) -> TokenId {
    ...
}
```

Examples:

- mint
- burn
- transfer
- freeze
- unfreeze
- upgrade
- govern
- bridge
- oracle_read
- stake
- unstake
- withdraw

A capability declaration does not bypass authorization. The ATC execution environment must evaluate the applicable authorization and policy rules.

## 7. Policies

Policies express normative contract invariants.

```atc
@policy soulbound
```

A registered policy is validated semantically. It is not merely descriptive metadata.

For a soulbound asset, the semantic invariant is:

```
soulbound(token) == true
=> transfer(token) MUST be rejected
```

## 8. Standard registry

Standards are resolved against the canonical ATC standards registry. A contract is invalid when:

- a referenced standard is unknown;
- a required directive is missing;
- a directive has an invalid value;
- incompatible standards are combined;
- a normative policy is violated.

Standard identifiers are not allocated locally by a contract repository.

## 9. Determinism

The compilation result is required to be reproducible.

The following artifacts are part of the evidence chain:

- source hash
- specification version
- compiler version
- AST/IR identity
- bytecode hash
- contract manifest

A successful build alone does not establish verification.

## 10. Manifest

A compiled contract may produce:

```yaml
format: ATC-CONTRACT-1
contract:
  name: ATownAvatar
  source: ATCAvatarNFT.atc
source:
  sha256: "<SOURCE_HASH>"
compiler:
  format: 1.0
standards:
  - ATC-001
types:
  - NFT721
capabilities:
  - mint
bytecode:
  format: ATC-VM
  sha256: "<BYTECODE_HASH>"
```

## 11. Interoperability

External token and chain representations are modeled as explicit ATC interoperability standards or adapters. They do not define the native `.atc` language or the ATC-VM execution model.

## 12. Separation of concerns

```
ATCLang
  = language and compiler foundation

.atc
  = native smart-contract source/profile

ATC Standards
  = normative contract/asset specifications

ATC-VM
  = deterministic execution target

A-TownChain
  = blockchain settlement and consensus layer
```

This separation is normative.

## 13. Compliance

A contract is only considered compliant after the applicable syntax, semantic, standard, determinism and security checks have produced evidence for the exact source revision.

``IMPLEMENTED``, ``TESTED``, ``VERIFIED`` and ``RELEASED`` are distinct evidence states.
