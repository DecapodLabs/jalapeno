# Operations and release direction

## Current development

Run `cargo run -- --help` or `cargo run -- --version` from the checkout. The binary
requires no services or runtime setup. Development uses Rust/Cargo; current checks
are in VALIDATION.md. `Cargo.toml` starts at `0.1.0` and explicitly disables
publishing. There is no release workflow, published crate, GitHub binary release,
engine installation or deployment in this first pass.

Use a claimed Decapod task, an isolated workspace, aligned living specs,
falsifiable claims and actual validation evidence. GitHub PR review remains the
acceptance boundary. Record a concrete validation/permission blocker if a required
gate is unavailable; do not invent success or bypass it. Keep machine-local proof
tool bindings and execution artifacts outside published source.

## Required future release process

Study the actual Decapod and Dactyl workflows before implementing. Initial
orientation confirmed the distinction: Decapod uses release-plz and cargo-dist;
Dactyl uses release-please with a GitHub App token and CARGO_REGISTRY_TOKEN.
Adopt one coherent authority, preferably release-plz for version management and
cargo-dist for standalone distribution, without copying Decapod's larger machinery.

The future pipeline must provide:

- Conventional-commit semantic versioning, automated release PRs and changelog updates.
- Publication only after the intended release PR is merged and required validation
  passes. Feature-branch publication is forbidden.
- Secure supported crates.io authentication, using existing authorized GitHub App
  and `release` environment conventions without copying/exposing secrets.
- Git tags, release notes, executable archives, checksums, clear provenance and
  reproducible release metadata; idempotent recovery and no competing publishers.
- `cargo publish --dry-run` and package-content verification before publication.
- Native macOS ARM64/x86_64, Linux ARM64/x86_64 and Windows x86_64 distribution
  wherever build/test evidence is available; distinguish target ambition from
  actually verified coverage.
- A small published core with no engine/heavy tooling dependencies; adapters can
  version independently and remain unpublished initially.

On 2026-10-24 the exact `jalapeno` registry API and sparse-index entries returned
404. This observation is not a name reservation; recheck at first publication.
Do not silently rename if it becomes occupied. Start at `0.1.0` unless authoritative
repository/registry history requires otherwise. Review authentication availability,
registry ownership, protected environment and validation before enabling publishing.

## Future documentation and recovery

Document CLI/crate installation, sound/button/icon/animation examples, Godot and
Unity setup, Unreal integration scope, feature/platform matrix, architecture,
security/FFI limits, roadmap and release troubleshooting. Explain how to diagnose
missing credentials, name conflicts, failed package verification, tag/version
mismatch and partial releases. Recovery must reuse the same reviewed release
identity rather than creating another publisher. External blockers do not justify
claiming a publication occurred.

<!-- decapod:capability-overlay:persistent-state:start -->

Generated capability obligations: `persistent-state` is declared for this project. These are decisions and proof requirements, not claims of implemented behavior. Resolve each against project source and tests; record unsupported or non-applicable mechanisms explicitly.

## Persistent State Operations Overlay

### Backup & Recovery
- Backup scope, schedule, retention, and restore evidence MUST be selected for the project
- Recovery point objectives MUST be explicit project decisions, not assumed values
- Recovery time objectives MUST be explicit project decisions, not assumed values
- Restore verification cadence MUST be recorded with the operational proof plan

### Migration Operations
- All schema changes via migration files
- Migration rollback procedures documented
- Migration downtime and availability effects MUST be declared; zero downtime requires project-specific proof
- Migration health checks and rollback triggers
<!-- decapod:capability-overlay:persistent-state:end -->

<!-- decapod:codebase-attestation:start -->

## Codebase Attestation

- Repository signal fingerprint: `f4950ae07b01d9cf5e8b97ebc737ac50e7c344f798210f9c0118b8c15d5055cd`
- Significant implementation surfaces: `Cargo.lock/` (1 files), `Cargo.toml/` (1 files), `README.md/` (1 files), `scripts/` (1 files), `src/` (1 files)
- Refreshed from the current codebase by `decapod rpc --op specs.refresh`
<!-- decapod:codebase-attestation:end -->
