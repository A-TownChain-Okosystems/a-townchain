# ATGAF-001 — A-Town Genesis Asset Format

- **Status:** Draft specification; implementation not yet verified
- **Version:** 1.0 draft
- **File extension:** `.atgasset`
- **Canonical owner:** `modules/atc-genesis-assets`
- **Issue:** [#40](https://github.com/A-TownChain-Okosystems/genesis-engine/issues/40)

The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHOULD**, **SHOULD NOT**, and **MAY** are normative requirements.

## 1. Scope and ownership

ATGAF defines a deterministic binary serialization for one root asset and its local payload chunks. It serializes the canonical asset domain; it does not create a second asset model, registry, resolver, or renderer.

- `atc-genesis-assets` owns parsing, writing, typed validation, and integration with the existing asset registry/pipeline.
- `atc-genesis-platform` remains the owner of `AssetId`, `AssetKind`, `AssetStore`, and `WorldAssetResolver` contracts.
- `atc-genesis-world` consumes validated world/mesh assets.
- `atc-genesis-renderer` consumes validated canonical assets and MUST NOT parse ATGAF or resolve asset dependencies itself.
- Build/evidence generation belongs to the existing build module and CI.

A file contains exactly one root asset. Dependencies are references to other assets by stable ID; a file is not a bundle of multiple root assets.

## 2. Version 1 invariants

1. Multi-byte integers are unsigned Big-Endian unless a typed payload explicitly specifies another representation.
2. V1 has no compression: codec ID `0` means raw bytes, and stored length MUST equal decoded length.
3. V1 reserved fields and unknown flag bits MUST be zero.
4. The writer MUST emit the canonical ordering defined below. The reader MUST reject structural ambiguity, overflow, invalid ranges, and unsupported major versions.
5. A reader MUST validate all structural bounds and configured size limits before allocating payload-sized buffers.
6. The reader MUST NOT publish or register a partially parsed asset. Typed validation must succeed before the result is made available to consumers.
7. Unknown critical metadata fields, critical chunks, or critical flags MUST be rejected. Unknown non-critical fields/chunks MAY be skipped only if their ranges and hashes validate.
8. An integrity hash proves byte integrity only; it does not authenticate the publisher.

## 3. File layout

The file is laid out as:

`Header | Metadata TLVs | Chunk data region | Chunk directory`

There are no padding bytes or unclaimed gaps in a canonical V1 file. Chunk offsets are absolute offsets from byte zero. The directory is the final region in the file.

### 3.1 Fixed header (80 bytes)

| Offset | Size | Field | V1 rule |
|---:|---:|---|---|
| 0 | 8 | Magic | ASCII bytes `ATGASSET` |
| 8 | 2 | Major version | `1` |
| 10 | 2 | Minor version | `0` for this draft |
| 12 | 4 | Header size | `80` in V1 |
| 16 | 4 | Header flags | Must be zero |
| 20 | 2 | Root asset type | Stable ATGAF type ID (section 5) |
| 22 | 2 | Reserved | Must be zero |
| 24 | 4 | Metadata length | Byte length of the TLV region |
| 28 | 4 | Chunk count | Number of directory entries |
| 32 | 8 | Payload length | Total file size minus header size |
| 40 | 8 | Directory offset | Absolute byte offset to the directory |
| 48 | 32 | File SHA-256 | Hash rule in section 6 |

The fixed header is exactly 80 bytes in V1. Readers MUST reject other header sizes for major version 1; a future minor version may define extensions only through a new normative revision. The metadata region starts at offset 80.

The total file length MUST equal `header_size + payload_length`. The directory length MUST equal `chunk_count * 64`, computed with checked arithmetic. The directory offset MUST equal `file_length - directory_length`. Metadata and chunk data MUST end at or before the directory; V1 canonical files have chunk data packed contiguously immediately after metadata and immediately before the directory.

## 4. Metadata TLV encoding

Metadata is a binary TLV sequence, not JSON. Each TLV header is 8 bytes:

| Size | Field |
|---:|---|
| 2 | Field ID (u16 BE) |
| 2 | Field flags (u16 BE) |
| 4 | Value length (u32 BE) |
| variable | Value bytes |

Bit 0 of field flags means **critical**. All other bits MUST be zero. Field IDs MUST be in ascending order; V1 does not permit duplicate field IDs. Each value MUST fit entirely inside the declared metadata region.

| Field ID | Name | Encoding | Requirement |
|---:|---|---|---|
| 1 | Asset ID | Exactly 16 bytes; unsigned u128 BE | REQUIRED |
| 2 | Display name | UTF-8, normalized to Unicode NFC | Optional; non-empty if present |
| 3 | Dependencies | u32 count followed by count × 16-byte Asset IDs (u128 BE) | Optional; IDs sorted ascending, unique |
| 4 | Source schema ID | UTF-8, NFC | Optional; non-empty if present |

The Asset ID MUST match the ID used by the existing canonical registry. Dependency IDs MUST be checked through the existing registry/resolver contract. Missing dependencies and dependency cycles MUST fail validation at the pipeline/registry boundary; parsing a file alone does not prove that external dependencies are present.

Unknown fields with the critical bit set MUST be rejected. Unknown non-critical fields MAY be ignored by readers, but writers MUST NOT silently drop them when performing a lossless rewrite.

## 5. Root asset type IDs and chunk types

Root asset type IDs are stable ATGAF values and MUST NOT be inferred from Rust enum discriminants.

| Root type ID | Name | V1 status |
|---:|---|---|
| 1 | Mesh | Defined |
| 2 | Material | Defined |
| 3 | Texture | Defined |
| 4 | Animation | Reserved for a later specification |
| 5 | Skeleton | Reserved for a later specification |
| 6 | Scene | Reserved for a later specification |
| 7 | Collision | Reserved for a later specification |
| 8 | LOD set | Reserved for a later specification |

A V1 reader MUST reject reserved root types until their payload schemas are specified.

Each chunk directory entry identifies a four-byte ASCII type, so examples are `VERT`, `NORM`, `UV00`, `INDX`, `MATL`, and `TEXR`. Chunk schemas are typed contracts and MUST be documented before the corresponding root type is considered implemented. A chunk is not automatically a validated asset merely because its hash is correct.

## 6. Chunk directory (64 bytes per entry)

Entries are sorted by chunk type bytes ascending, then by absolute offset ascending. In V1 duplicate chunk types are rejected unless a future schema explicitly declares that type repeatable.

| Offset within entry | Size | Field |
|---:|---:|---|
| 0 | 4 | Chunk type (four ASCII bytes) |
| 4 | 2 | Codec ID; V1 requires `0` (raw) |
| 6 | 2 | Chunk flags; bit 0 critical, other bits zero |
| 8 | 8 | Absolute file offset |
| 16 | 8 | Stored length |
| 24 | 8 | Decoded length |
| 32 | 32 | SHA-256 of stored chunk bytes |

All arithmetic MUST be checked for overflow. Every chunk range MUST lie wholly within the chunk data region. Chunk ranges MUST be non-overlapping and contiguous in canonical output. For codec `0`, stored and decoded lengths MUST match. Hashes MUST be checked before typed decoding.

### 6.1 File hash

The file SHA-256 is computed over the complete file bytes after replacing header bytes `[48, 80)` (the file-hash field) with 32 zero bytes. The resulting digest is stored at bytes `[48, 80)`. This rule avoids self-reference and is mandatory for every V1 writer and reader.

The hash is computed after all other header fields, metadata, chunk bytes, directory entries, and per-chunk hashes have their final canonical values.

## 7. Canonical typed payloads

V1 root types are only considered supported when both their binary payload schema and typed validation are implemented.

### 7.1 Mesh (`MESH`)

A Mesh root serializes the existing `MeshAsset` fields from `modules/atc-genesis-assets/src/lib.rs`: vertices, normals, UVs, and indices. The canonical chunk mapping is:

- `VERT`: 3 IEEE-754 binary32 values per vertex, X/Y/Z order.
- `NORM`: 3 IEEE-754 binary32 values per normal, X/Y/Z order.
- `UV00`: 2 IEEE-754 binary32 values per UV, U/V order.
- `INDX`: unsigned u32 indices, Big-Endian.

All floats MUST be finite. Negative zero MUST be canonicalized to positive zero by writers. NaN and infinity MUST be rejected. The three vertex streams MUST have equal element counts; index count MUST be divisible by three; every index MUST reference an existing vertex. Stream byte lengths MUST be exact multiples of their element widths. The existing `MeshAsset::validate` MUST remain the canonical semantic validator.

### 7.2 Material (`MATL`)

The Material root serializes the existing `MaterialAsset`: base color RGBA, metallic, roughness, and an optional base-color texture Asset ID. Float values are binary32 Big-Endian and finite; metallic and roughness MUST be in `[0,1]`. The optional texture reference MUST be resolved through the existing registry/asset pipeline. Exact chunk encoding is gated on a typed codec implementation.

### 7.3 Texture (`TEXR`)

The Texture root serializes the existing `TextureAsset`: width, height, channel count, and raw pixel bytes in row-major order. V1 channel count is 1–4; width and height are non-zero. The expected byte count MUST be calculated with checked multiplication and MUST equal the decoded pixel length. No GPU-native or platform-specific texture layout is part of V1.

## 8. Initial resource limits

Implementations MUST enforce configurable limits before allocation. Initial default limits for the first implementation are:

- maximum file size: 512 MiB;
- maximum metadata size: 1 MiB;
- maximum chunk count: 4096;
- maximum stored or decoded size per chunk: 256 MiB;
- maximum dependency count: 65,536.

Limits MUST be checked using checked arithmetic. A limit increase is a policy change and MUST NOT silently change the byte format.

## 9. Error and publication semantics

Malformed magic/version/header, unsupported critical fields, invalid UTF-8/NFC, duplicate fields, invalid sort order, invalid offsets/lengths, overlaps/gaps in canonical layout, invalid hashes, unsupported codecs, resource-limit violations, or invalid typed payloads MUST return an explicit error.

On any error, the parser MUST return no usable root asset and MUST NOT mutate the registry, pipeline, world, or renderer. Registration occurs only after the complete file, all local chunks, and the typed root asset have passed validation. External dependency resolution is a separate explicit pipeline step using the existing asset contracts.

## 10. Determinism and conformance tests

A conforming writer MUST produce byte-identical output for semantically identical input assets and dependency sets. A conforming reader/writer pair MUST pass:

1. roundtrip tests for mesh, material, and texture;
2. two-write byte equality tests for each root type;
3. known header offsets, lengths, and file/chunk hash vectors;
4. truncated header, metadata, directory, and chunk tests;
5. integer-overflow, oversized-input, invalid-offset, overlap, and gap tests;
6. duplicate/unsorted metadata and directory entry tests;
7. invalid asset ID, missing dependency, dependency-cycle, and typed payload tests;
8. fuzz tests for parser entry points and property tests for bounds safety;
9. render integration proving a validated serialized mesh reaches the existing renderer and expected image pixels are asserted.

The test suite and CI results are evidence for a specific commit only. This specification alone does not claim that the parser, writer, codecs, fuzzing, or render end-to-end path are implemented.

## 11. Compatibility and evolution

A major version change is required for incompatible layout or semantic changes. Minor versions may add only explicitly backward-compatible features and MUST define unknown-field behavior. Root type IDs and chunk IDs are never reused. Compression, signatures, animation, skeleton, scene, collision, and LOD payloads require separate versioned schema definitions and tests before use.

## 12. Implementation gates

1. **Specification:** review this contract against existing platform/asset types.
2. **Container codec:** implement header, TLV, directory, bounds validation, and file/chunk hashes.
3. **Typed codecs:** implement Mesh, Material, and Texture using existing models and validators.
4. **Security tests:** negative vectors, size limits, fuzzing, and deterministic roundtrips.
5. **Integration:** existing resolver/registry plus renderer E2E image artifact and pixel assertions.
6. **Evidence:** exact commit SHA, CI run/job/step exit codes and logs, artifact path and SHA-256.

Until these gates are evidenced, status remains **DRAFT / BLOCKED / NOT VERIFIED**.
