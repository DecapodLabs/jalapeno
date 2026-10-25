# Jalapeño specification index

These authored specs preserve the original Jalapeño product brief and the user's
subsequent scope refinements: first a simple entrypoint with full context, then
a tight functional slice for **tones and icons**. Current behavior is additive
tone/WAV and harmonic icon/SVG generation; other requirements remain planned.

| Specification | Responsibility |
| --- | --- |
| [INTENT.md](INTENT.md) | Mission, users, original requirements and delivery stages |
| [ARCHITECTURE.md](ARCHITECTURE.md) | Current source and future core/adapter boundaries |
| [INTERFACES.md](INTERFACES.md) | Current CLI and proposed Rust, recipe, export and ABI contracts |
| [SEMANTICS.md](SEMANTICS.md) | Planned mathematical behavior, determinism and limits |
| [SECURITY.md](SECURITY.md) | Provenance, privacy, unsafe code and resource safeguards |
| [OPERATIONS.md](OPERATIONS.md) | Current development and planned publication/recovery |
| [VALIDATION.md](VALIDATION.md) | Current checks and staged future acceptance evidence |

Source of truth for implemented behavior: `Cargo.toml`, `src/main.rs`, `src/audio.rs` and `src/visual.rs`.
`README.md` presents the same limited capability. `.decapod/governance.json`
records this PR's scope, claims and evidence. GitHub PR review is the acceptance
boundary. Machine-local sessions, logs, credentials and unverified drafts do not
belong in Git.

<!-- decapod:codebase-attestation:start -->

## Codebase Attestation

- Repository signal fingerprint: `a695576554ce8de4e235c7eef5eb01efb229a04ebdea09fc7e921f4f92a786b3`
- Significant implementation surfaces: `.github/` (1 files), `Cargo.lock/` (1 files), `Cargo.toml/` (1 files), `README.md/` (1 files), `scripts/` (2 files), `src/` (4 files), `tests/` (1 files)
- Refreshed from the current codebase by `decapod rpc --op specs.refresh`
<!-- decapod:codebase-attestation:end -->
