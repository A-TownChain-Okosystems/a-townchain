# Central Knowledge & Documentation System

**System ID:** ATC-KMS  
**Version:** v1.0.0  
**Status:** ACTIVE  
**Repository:** a-townchain-ecosystem

## Purpose

ATC-KMS is the central, lossless knowledge-management layer for the A-TownChain ecosystem. It records project knowledge as durable, structured, versioned artifacts.

> **Core rule:** No information is lost. Everything is captured, structured, versioned, traceable, and retrievable.

## Scope

The system covers projects, modules, features, code building blocks, ideas and concepts, roadmap/TODO items, changes and decisions, versions/releases, architecture and integration knowledge, source references/evidence, discussions, and derived summaries.

## Storage model

Human-readable Markdown is the canonical presentation format. Machine-readable JSON is the canonical structured-record format.

    docs/knowledge-system/
    ├── README.md
    ├── schema.json
    ├── registry.json
    ├── RECORD_TEMPLATE.md
    ├── records/
    │   ├── projects/
    │   ├── modules/
    │   ├── features/
    │   ├── code/
    │   ├── ideas/
    │   ├── roadmap/
    │   ├── decisions/
    │   └── versions/
    └── summaries/

## Record identity

Every record MUST contain:

- id — globally unique stable identifier
- timestamp — ISO-8601 creation/update timestamp
- category
- title
- context
- priority
- status
- version
- source
- tags
- content

Records are append-oriented. Existing information is extended or superseded through explicit version links rather than silently overwritten.

## Versioning

Semantic versioning is used for knowledge-system releases: MAJOR.MINOR.PATCH.

Individual records also carry their own version. Material changes MUST create a new history entry.

## Integrity rules

1. No deletion without explicit authorization.
2. No silent replacement of historical information.
3. Every material change is traceable to a commit or source.
4. Summaries MUST retain a reference to their source records.
5. Conflicts MUST be recorded, not silently resolved.
6. Unknown information MUST be marked unknown.
7. Derived conclusions MUST be distinguishable from source facts.
8. External claims MUST retain their source reference.
9. Archived records remain addressable.
10. Automation may classify and summarize, but must not fabricate missing facts.

## Operating model

Capture → Normalize → Classify → Link → Version → Validate → Summarize → Archive → Retrieve

The knowledge system is an auditable project memory, not merely a documentation folder.
