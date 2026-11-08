---
document_id: ATC-DOC-GENESIS-GNAF-001
title: Genesis Native Asset Format (GNAF) — Draft Specification
version: 0.1.0-draft
status: proposed
standard: ATC-STD-MD-001
---

# Genesis Native Asset Format (GNAF)

**Status:** Design draft only. This document does not define a released or implemented file format. The name `GNAF` and `.gasset` extension are provisional and require repository-wide collision checks before stabilization.

## 1. Purpose and scope

GNAF is a proposed engine-owned binary container for cooked Genesis assets. It complements, rather than replaces, glTF 2.0 and GLB:

- **glTF/GLB:** interoperability and external-tool exchange.
- **Canonical Genesis asset contract:** semantic model, validation rules, and asset identity; reuse `atc-genesis-assets` and the existing `AssetRegistry`.
- **GNAF:** versioned on-disk container for validated, cooked data that the Genesis runtime can load.

GNAF is not a 2D-to-3D generator, renderer, asset registry, or new source-of-truth model. Image-to-3D pipelines must produce the canonical asset contract first, then use the same validation and cooking path as other assets.

## 2. Existing-repository findings and limits

The preliminary source review found:

- `modules/atc-genesis-assets/src/lib.rs` contains `AssetPipeline`, `ImportRecord`, `MeshAsset`, `MaterialAsset`, `TextureAsset`, dependency ordering, and validation. Mesh validation currently checks attribute counts, triangle index count, index bounds, and finite vertex/normal values; additional UV, empty-mesh, degenerate-triangle, and resource-budget policies are not established by the inspected implementation.
- `modules/atc-genesis-assets/src/registry.rs` contains the existing `AssetRegistry`, dependency queries, missing-dependency reporting, and cycle detection. Do not add a second registry.
- `modules/atc-genesis-build/src/lib.rs` has package manifests and artifact checksums, but its current checksum helper is 64-bit FNV-1a, not a cryptographic integrity hash.
- `modules/atc-genesis-renderer/src/lib.rs` provides render graph and renderer abstractions, but the inspected file does not prove native asset loading or end-to-end rendering.
- `Cargo.toml` includes the existing assets, build, renderer, animation, runtime, and CLI modules. The current roadmap still marks M7 (engine loop/ECS stabilization) as the active focus; this proposal must not silently replace it.

This is a **targeted preliminary review**, not a complete audit of all importer, runtime, renderer, workflow, and dependency paths. Implementation must remain blocked until that audit is completed and the contract owners approve this design.

## 3. File naming and versioning

- Proposed single-asset extension: `.gasset`.
- Proposed four-byte ASCII magic: `GNAF`.
- Proposed initial format version: major `1`, minor `0`.
- All multi-byte integers in version 1 are little-endian.
- Major-version incompatibility must be rejected. Unknown optional chunks may be skipped only after their bounds and integrity metadata are validated. Unknown required chunks must cause a clear load failure.
- The extension, magic, version policy, and ownership of the format require approval before a stable release.

## 4. Version 1 container layout proposal

The following layout is a reviewable proposal, not yet a compatibility commitment.

### 4.1 Fixed header — 64 bytes

| Offset | Size | Field | Rule |
|---:|---:|---|---|
| 0 | 4 | magic | ASCII `GNAF` |
| 4 | 2 | major_version | `1` |
| 6 | 2 | minor_version | `0` for initial draft |
| 8 | 1 | endian | `1` = little-endian |
| 9 | 1 | flags | Unknown required flags reject the file |
| 10 | 2 | header_size | `64` for version 1 |
| 12 | 8 | directory_offset | Absolute file offset |
| 20 | 4 | directory_count | Number of entries |
| 24 | 4 | reserved | Must be zero in version 1 |
| 28 | 8 | file_length | Exact total byte length |
| 36 | 8 | manifest_offset | Absolute offset to integrity manifest |
| 44 | 8 | manifest_length | Manifest byte length |
| 52 | 12 | reserved | Must be zero in version 1 |

Readers must reject files whose declared length differs from the actual length, arithmetic overflows, offsets exceed file bounds, or required regions overlap.

### 4.2 Chunk directory — 40 bytes per entry

Entries are sorted by four-byte chunk type, then by chunk offset. Duplicate entries are rejected unless a future schema explicitly permits them.

| Offset | Size | Field | Rule |
|---:|---:|---|---|
| 0 | 4 | type | Four ASCII bytes |
| 4 | 2 | schema_version | Version of this chunk schema |
| 6 | 2 | flags | Bit 0 = required; other unknown bits reject |
| 8 | 8 | offset | Absolute payload offset |
| 16 | 8 | stored_length | Bytes present in file |
| 24 | 8 | raw_length | Expected decoded length |
| 32 | 4 | alignment | Power of two; minimum 1; capped by reader policy |
| 36 | 4 | reserved | Must be zero |

Version 1 should initially support uncompressed chunks only (`stored_length == raw_length`). Compression must not be enabled until codec IDs, decompression budgets, and bomb-resistance tests are specified. Every range calculation must use checked arithmetic before allocation or slicing.

### 4.3 Initial chunk types

| Type | Required | Purpose |
|---|---|---|
| `META` | Yes | Schema identifier, asset kind, stable asset ID, coordinate/unit conventions |
| `MESH` | For mesh assets | Vertex attributes, indices, primitive/topology descriptors |
| `MATL` | When materials exist | PBR material parameters and texture references |
| `TREF` | When textures are referenced | Logical texture asset IDs and safe resource references |
| `TEXD` | Optional | Embedded texture payloads where supported |
| `SKEL` | Optional | Skeleton and skinning data; not required for v1 implementation |
| `ANIM` | Optional | Animation clips/channels; not required for v1 implementation |
| `SCNE` | Optional | Node hierarchy and transforms for multi-object assets |

This list is provisional. Before freezing it, reconcile it with the complete canonical asset contract and renderer/runtime resource model. A reader must not silently discard a required feature.

### 4.4 Integrity manifest

The manifest must have a schema/version and contain SHA-256 for every chunk payload, plus the format's canonical metadata. Hash the exact stored payload bytes for v1; any future compression scheme must explicitly define whether a digest covers stored bytes, decoded bytes, or both. The manifest encoding must be canonical and deterministic.

A digest detects corruption only when the expected digest is trusted. Authenticity/signature policy is a separate package/signing concern. Do not label FNV-1a as cryptographic integrity.

## 5. Canonical semantics and resource safety

- Reuse existing asset types and `AssetRegistry`; GNAF is serialization, not a second semantic asset database.
- Define canonical float representation, attribute ordering, index encoding, coordinate system, units, and treatment of non-finite values before promising byte-for-byte determinism.
- Validate mesh/material/texture data before serialization and again after deserialization.
- Validate all IDs and references, dependency existence, and dependency cycles through the existing registry/pipeline contracts.
- Enforce configurable limits for file size, chunk count, mesh vertices/indices, texture dimensions/pixels, nesting depth, and total allocations.
- Reject truncated headers/directories/manifests, overlapping ranges, invalid alignment, integer overflow, invalid indices, non-finite required numeric data, unknown required chunks/flags, unsupported major versions, and missing dependencies.
- Do not execute code from asset files. External paths must be relative, normalized, sandboxed, and prevented from escaping the asset root.

## 6. Deterministic writer and reader contract

For semantically identical canonical assets and identical writer settings, the writer should emit byte-identical files. To achieve this, define deterministic ordering for assets, chunks, maps, dependencies, strings, and manifest entries; reject or canonicalize NaN and negative zero; and avoid timestamps or host-specific absolute paths in output.

Reader API shape is intentionally not frozen here. The implementation should expose bounded parsing and typed validation errors, and should not allocate from untrusted lengths until all global limits and range checks pass.

## 7. Interoperability and runtime path

```text
glTF/GLB or image-to-3D output
        -> importer / reconstruction pipeline
        -> canonical Genesis asset contract
        -> existing validation + dependency checks
        -> deterministic GNAF writer
        -> GNAF reader / validation
        -> existing runtime resource bindings
        -> actual renderer/runtime verification
```

Roundtrip acceptance is semantic: canonical asset → GNAF → canonical asset must preserve normalized supported content. glTF/GLB conversion must report unsupported or lossy features explicitly. A successful parse alone is not proof of runtime rendering.

## 8. Required tests and acceptance gates

1. **Golden vectors:** fixed valid byte fixtures for header, directory, each required chunk, and manifest.
2. **Determinism:** same canonical input/settings produce byte-identical output; shuffled input maps normalize to the same output.
3. **Roundtrip:** canonical → GNAF → canonical semantic equality for supported fields.
4. **Negative parsing:** truncation at every structural boundary, invalid magic/version/endian, bad offsets/lengths, integer overflow, overlapping regions, bad alignment, duplicate chunks, unknown required features, corrupt digest, invalid mesh indices, and missing references.
5. **Resource budgets:** huge declared sizes/counts, excessive texture dimensions, nested/dependency cycles, and decompression limits if compression is later added.
6. **Fuzzing:** parser fuzz target with no panic, unbounded allocation, or out-of-bounds access.
7. **Interoperability:** glTF/GLB import → canonical model → GNAF → canonical model; compare normalized semantics.
8. **Runtime E2E:** load the exact generated artifact in the real Genesis runtime and demonstrate renderer-visible output on a supported backend.
9. **Evidence:** exact commit SHA, Actions run/job/step and exit/logs, test fixture hashes, and artifact SHA-256. Keep statuses separate: `IMPLEMENTED`, `TESTED`, `CI_VERIFIED`, `E2E_VERIFIED`, `VERIFIED`, `RESIDUAL`, `BLOCKED`. Unknown evidence remains `BLOCKED`; green CI alone is not `VERIFIED`.

## 9. Out of scope for the initial format milestone

- Implementing image-to-3D generation.
- Replacing glTF/GLB as exchange formats.
- Creating a new asset registry or duplicate renderer/import pipeline.
- Enabling compression, streaming, GPU-specific layouts, LOD, skeletons, or animation before each feature has a canonical schema and tests.
- Claiming performance improvements without benchmark fixtures and comparable runtime measurements.

## 10. Review checklist before stabilization

- [ ] Complete read-only audit of all importer, runtime, renderer, package, dependency, and CI paths.
- [ ] Confirm no existing native format or extension conflicts.
- [ ] Obtain approval from canonical asset/runtime contract owners.
- [ ] Freeze coordinate/unit conventions and canonical float/attribute rules.
- [ ] Freeze header, directory, chunk schemas, manifest encoding, and limits.
- [ ] Add golden fixtures and parser negative tests.
- [ ] Demonstrate roundtrip and actual runtime rendering on the exact commit.
- [ ] Record reproducible CI/E2E evidence and artifact hashes.

**Current status:** proposed design only; no parser/writer implemented; no tests run for this specification; native loading, roundtrip, and runtime rendering remain **NOT VERIFIED**.
