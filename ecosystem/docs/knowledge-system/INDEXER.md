# ATC-KMS Automatic Knowledge Indexer

The indexer converts repository and GitHub metadata into ATC-KMS records.

Sources: repository tree, component paths, architecture/release/security/migration/build source files, pull requests, issues, recent commits, and explicit PR/issue references.

Generated records live under docs/knowledge-system/records/generated/. Hand-authored records outside that directory are never modified by the indexer.

Execution: on main pushes outside generated output, nightly at 02:17 UTC, and manually through GitHub Actions.

Git history, canonical source repositories, PRs, issues and standards remain authoritative; KMS stores traceable knowledge about them.