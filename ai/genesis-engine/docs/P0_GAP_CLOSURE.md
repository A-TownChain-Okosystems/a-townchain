# P0 Gap Closure

This document tracks the first implementation pass for gaps identified by the engineering audit.

## Evidence rule

A capability is marked implemented only after source exists and the repository CI/build verifies it. Architecture text alone is not evidence of a working implementation.

## Current P0 targets

1. Repair the `atc-genesis-ai` parser error.
2. Verify and repair the direct `atc-genesis-sdk` dependency on `atc-genesis-ecs`.
3. Resolve workspace membership/file-tree inconsistencies.
4. Run formatting, check, test, clippy and documentation gates in that order.
5. Record every remaining failure as an explicit implementation gap rather than hiding it behind roadmap language.

## Scope

This pass is intentionally fail-closed. It does not mark hardware backends, network transports, cryptographic primitives, codecs, or production release infrastructure as complete merely because interfaces exist.
