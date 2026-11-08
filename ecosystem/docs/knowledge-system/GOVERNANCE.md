# ATC-KMS Governance Policy

## 1. Authority

ATC-KMS is the repository-level knowledge-management system for a-townchain-ecosystem. It does not replace source repositories, Git history, issue/PR history, or canonical technical standards. It indexes and preserves their project knowledge.

## 2. Capture policy

Knowledge SHOULD be captured at the point where it is created or materially changed. Source references MUST be retained for imported or derived information.

## 3. No-loss policy

Information is never silently discarded. If information becomes obsolete, the record is marked deprecated, archived, or superseded and linked to the successor record.

## 4. Conflict policy

Conflicting statements are retained with their respective sources and timestamps. The system records the conflict and its resolution status instead of rewriting history.

## 5. Traceability

A record that describes a technical change SHOULD reference the relevant commit, pull request, issue, file, or external source.

## 6. Summaries

Executive summaries and condensed views are derived artifacts. The underlying source records remain authoritative for historical completeness.

## 7. Automation boundary

Automation may validate structure, detect missing metadata, classify records, build indexes, and generate summaries. Automation MUST NOT invent facts or silently overwrite historical records.

## 8. Retention

The default retention policy is indefinite. Deletion requires explicit authorization and MUST itself be documented.

## 9. Change control

Changes to the ATC-KMS schema, registry categories, or integrity rules are versioned and reviewed like production code.

## 10. Reproducibility

Given the same repository revision and source records, generated indexes and summaries SHOULD be reproducible.
