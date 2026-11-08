# ATCLANG-ENGINEERING-001 — ATCLang Artifact Engineering

**Status:** IMPLEMENTED (initial adapter)

## Scope

`atc-engineering` recognizes ATCLang artifacts and provides governed repository-level operations for:

1. detection and classification;
2. creation of `.atc` source artifacts;
3. controlled source replacement/editing;
4. canonical compiler invocation from `.atc` to `.atvm`.

The implementation does **not** reimplement the ATCLang parser, semantic checker or compiler. The canonical `atclang` toolchain remains authoritative.

## Artifact classes

| Extension | Class | Engineering rule |
|---|---|---|
| `.atc` | `Source` | UTF-8 source; may be created/edited by the adapter |
| `.aes` | `Ecosystem` | recognized; semantics remain specification-governed |
| `.atvm` | `VmBytecode` | recognized as VM artifact; treated as binary/opaque by repository tooling |

## CLI operations

Inspect an artifact:

```bash
cargo run --release -p atc-audit-cli -- /path/to/target --atclang-inspect --input contracts/example.atc
```

Create a source artifact:

```bash
ATC_SOURCE='contract Example {}' \
cargo run --release -p atc-audit-cli -- /path/to/target --atclang-create --output contracts/example.atc
```

Edit/replace a source artifact:

```bash
ATC_SOURCE='contract Example { fn ping() {} }' \
cargo run --release -p atc-audit-cli -- /path/to/target --atclang-edit --input contracts/example.atc
```

Compile through the canonical compiler executable:

```bash
ATCLANG_COMPILER=atclang \
cargo run --release -p atc-audit-cli -- /path/to/target --atclang-compile \
  --input contracts/example.atc --output build/example.atvm
```

The adapter validates the input/output artifact classes and additionally requires the compiler process to succeed and produce the declared `.atvm` output. It does not treat a successful process exit without an output artifact as success.

## Safety boundaries

- Existing files are never overwritten by `--atclang-create`.
- `--atclang-edit` only operates on `.atc` files.
- Compiler input must be `.atc`.
- Compiler output must be `.atvm`.
- `.atvm` is not decoded or edited as text by this layer.
- Compiler semantics are delegated to the canonical ATCLang implementation.

## Requirements linkage

The requirements engine derives `TECH-ATCLANG-001` through `TECH-ATCLANG-006` when ATCLang artifacts are detected. Detection is a signal; it does not establish that creation, editing, compilation, tests or documentation are complete.

## Verification

The adapter contains tests for extension classification, source creation, source replacement and invalid extension rejection. End-to-end compiler verification requires an installed/checked-out canonical `atclang` toolchain and is therefore kept as an integration concern rather than faked inside this crate.
