# Operations

## Current development

Run the tone and icon commands documented in README and INTERFACES. Rust callers
use `audio::Tone` and `visual::Icon`. The program needs no runtime services or
external codecs. `Cargo.toml` is prepared as `0.0.1`; no crate or release archive
has been published and no engine integration is claimed.

Run `python3 scripts/check.py` for fmt, clippy, unit/property checks and decoded
WAV/SVG CLI smoke checks. It also packages the exact crate source set and runs
`cargo publish --dry-run`. GitHub CI uses this same script on Linux/macOS/Windows.
Actual hosted execution results must be observed independently; authored CI is
not execution proof.

For native Decapod readiness on macOS ARM64, use standard Xcode and installed
Rust 1.99.0 (`b940084d7`), then run `python3 scripts/prepare-proof.py`. It copies
installed Rust bin/lib files to `/private/tmp/jalapeno-proof-v1/rust` and verifies
their bytes. No app build, dependency download or validation occurs in preparation.
The tracked execution contract declares standard tool paths and the measured
compiler/runtime/linker/SDK/Python closure. No user home, credentials, logs or
runtime state is committed. The prefix is disposable; rerun preparation after
cleanup, and review any toolchain change explicitly.

The check uses fresh temporary Cargo/export storage, so Decapod can execute it
against an immutable source snapshot. Checkpoint and commit the candidate, then
run `decapod validate`. A reviewed command/source digest binds the selected local
profile. Passing local readiness proves this native scope, not the hosted CI
matrix or releases. Other platforms can use the check script; native Decapod
profiles there remain follow-on work. GitHub PR review remains acceptance.

## Release process

Decapod's current release workflow and Dactyl's release-please workflow were
inspected. Jalapeño uses the smaller release-plz + cargo-dist split, with the
shared DecapodLabs release-manager GitHub App and a single tag/release owner.

The current pipeline uses release-plz as the sole version/changelog and crate
publisher authority, and cargo-dist as the GitHub binary distribution authority.
`.release-plz.toml` names the changelog and `v{{ version }}` tags, disables
release-plz GitHub releases to avoid a second release writer, and sets
`release_always = false` so ordinary main pushes never publish.
`.github/workflows/release-plz.yml` opens release PRs on `main` and runs crate
publication in the `release` environment. It uses the shared DecapodLabs
release-manager GitHub App for PR/tag operations and supports a release-PR-only
dispatch. Merging the release-plz PR is the publication
boundary. The generated `.github/workflows/release.yml` builds and attaches
cargo-dist archives/installers and checksums when the version tag is pushed. CI
runs on the release PR, and the publication workflow reruns package contents,
tests and dry-run publishing on all three host OSes after merge before publishing.
The dist workflow also has reviewed permission, concurrency and retry-safety edits.
`allow-dirty` records this deliberate difference from cargo-dist's generated
template; review the edits against the generator whenever upgrading cargo-dist.

The initial target is `0.0.1`, as explicitly requested. No tag or version history
exists in the repository, and the crates.io index had no `jalapeno` entry at the
time of this check; this is availability evidence, not a name reservation. Before first publication,
the release environment needs a scoped `CARGO_REGISTRY_TOKEN`; the GitHub App
credentials have been shared with this repository for PR/tag operations. Add
required environment reviewers if a separate deployment approval is wanted.
crates.io Trusted Publishing cannot be configured until
the crate has its initial manual publication, so the first release uses that
credential; configure OIDC for subsequent releases as a separate reviewed update.
Publication is never performed directly from a feature branch.

The pipeline must continue to provide:

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
Do not silently rename if it becomes occupied. The requested initial version is
`0.0.1`; subsequent versions follow Conventional Commits from that base. Review
authentication availability,
registry ownership, release environment and any configured reviewer gate before
enabling publishing.

## Future documentation and recovery

Document CLI/crate installation, sound/button/icon/animation examples, Godot and
Unity setup, Unreal integration scope, feature/platform matrix, architecture,
security/FFI limits, roadmap and release troubleshooting. Explain how to diagnose
missing credentials, name conflicts, failed package verification, tag/version
mismatch and partial releases. Recovery must reuse the same reviewed release
identity rather than creating another publisher. External blockers do not justify
claiming a publication occurred.



<!-- decapod:codebase-attestation:start -->

## Codebase Attestation

- Repository signal fingerprint: `efd11ee45e79cb3222dc6f98a8597871b16cef3e293daa4feaa8f652576d0a5e`
- Significant implementation surfaces: `.github/` (4 files), `Cargo.lock/` (1 files), `Cargo.toml/` (1 files), `README.md/` (1 files), `scripts/` (2 files), `src/` (4 files), `tests/` (1 files)
- Refreshed from the current codebase by `decapod rpc --op specs.refresh`
<!-- decapod:codebase-attestation:end -->

## Service Level Objectives

No service uptime objective applies to a local CLI scaffold. Future synthesis
throughput, memory, startup and real-time claims require scoped measurements.

## Monitoring

Current observability is stdout/stderr and the exit code. No telemetry, daemon or
monitoring backend is present or required for core synthesis.

## Incident Response

For incorrect CLI/export behavior, preserve the failing command and diagnostic,
fix the core or entrypoint and rerun checks. For staged-tool mismatch, inspect the
owned temporary installation and re-stage trusted installed tools; do not alter
proof pins merely to suppress an unexplained failure. For a future release failure, preserve the exact
reviewed release identity and validation evidence; follow the recovery constraints
above without exposing credentials or introducing another publisher.
