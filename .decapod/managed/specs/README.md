# Jalapeño specification index

These authored specs preserve the original Jalapeño product brief and the user's
subsequent scope reduction: **a very simple initial `src/main.rs`, with all prompt
context accounted for in aligned Decapod specs**. They are requirements and design
direction where marked planned, not an attestation of delivered synthesis.

| Specification | Responsibility |
| --- | --- |
| [INTENT.md](INTENT.md) | Mission, users, original requirements and delivery stages |
| [ARCHITECTURE.md](ARCHITECTURE.md) | Current source and future core/adapter boundaries |
| [INTERFACES.md](INTERFACES.md) | Current CLI and proposed Rust, recipe, export and ABI contracts |
| [SEMANTICS.md](SEMANTICS.md) | Planned mathematical behavior, determinism and limits |
| [SECURITY.md](SECURITY.md) | Provenance, privacy, unsafe code and resource safeguards |
| [OPERATIONS.md](OPERATIONS.md) | Current development and planned publication/recovery |
| [VALIDATION.md](VALIDATION.md) | Current checks and staged future acceptance evidence |

Source of truth for implemented behavior: `Cargo.toml` and `src/main.rs`.
`README.md` presents the same limited capability. `.decapod/governance.json`
records this PR's scope, claims and evidence. GitHub PR review is the acceptance
boundary. Machine-local sessions, logs, credentials and unverified drafts do not
belong in Git.

<!-- decapod:codebase-attestation:start -->

## Codebase Attestation

- Repository signal fingerprint: `f4950ae07b01d9cf5e8b97ebc737ac50e7c344f798210f9c0118b8c15d5055cd`
- Significant implementation surfaces: `Cargo.lock/` (1 files), `Cargo.toml/` (1 files), `README.md/` (1 files), `scripts/` (1 files), `src/` (1 files)
- Refreshed from the current codebase by `decapod rpc --op specs.refresh`
<!-- decapod:codebase-attestation:end -->
