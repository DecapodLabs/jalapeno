# Security and provenance

## Threat Model

`src/main.rs` accepts local options and writes caller-selected output paths.
The core validates finite numbers and explicit duration, rate, harmonic, work,
canvas, phase and sample-count bounds. SVG emits numeric paths and fixed markup;
no user strings are embedded. WAV export streams bounded scratch to a writer.
There is no network, plugin loading, external code evaluation, credentials or
recipe parser. `src/lib.rs` forbids unsafe code; `Cargo.toml` has no dependencies
and whitelists only source, tests, license and README in the crate archive. The
release workflow is the only publishing path. It runs on `main`, serializes
release jobs, uses the shared scoped release-manager GitHub App and the Actions
`release` environment, and requires merging a release-plz PR before crate
publication. The initial release uses a crates.io API token; OIDC Trusted
Publishing is a later migration after `0.0.1` exists.

The CLI uses create-new temporary files and non-replacing hard-link installation.
It preserves an existing destination, including a symlink, and returns errors for
unsupported filesystems. An abrupt process kill can leave a temporary sibling;
the next invocation does not overwrite it. The caller remains responsible for
choosing an appropriate local output directory.

Decapod proof tooling is development-only. The preparation script stages existing
installed Rust files at a fixed external prefix and verifies their bytes. Tracked
configuration names standardized tool installations, not a user's home or mutable
runtime state. It downloads no executables and reads no credentials. Decapod
measures declared tools and runs the check in fresh scratch against an immutable
snapshot. This initial profile trusts macOS/Xcode's host runtime, not a sandbox
security claim; it supports only the documented native readiness environment.

## Future input and resource boundaries

Recipes will be untrusted local input. Validate schema versions, field ranges,
finite numbers, recursion/collection sizes, frequencies, durations, polyphony,
output dimensions and estimated workload before allocation/rendering. Do not
introduce arbitrary code evaluation, network fetches, external asset references
or shell execution as a side effect of synthesis. Export errors must preserve
useful existing files and avoid partial-success claims.

Unsafe Rust belongs only in isolated adapters with explicit pointer, alignment,
length, capacity, ownership, release, threading and panic boundaries. Test stale
or invalid handles where the contract permits detection, allocation failure/error
paths and buffer lifetime. A C ABI cannot make arbitrary dangling caller pointers
safe; document the caller's obligations and any real-time limitations.

## Originality and release custody

Waystead is a private reference. General engineering patterns may inform original
primitives, but game-specific compositions, assets, logos, geometry coefficients,
names and proprietary source must not be included. Examples must be original and
independently usable. Do not copy reference credentials or print secret values.
The initial broader unverified draft is excluded from this change.

Release credentials belong in the existing DecapodLabs release-manager
GitHub App/release environment conventions. The package whitelist omits CI
workflows, Decapod governance state, scripts, adapters and runtime state. The
release workflow is not exposed to pull-request code and does not publish from
arbitrary `main` changes. Do not publish from an unreviewed feature branch or
replace a taken crate name silently. MIT licensing is in `LICENSE`.



<!-- decapod:codebase-attestation:start -->

## Codebase Attestation

- Repository signal fingerprint: `efd11ee45e79cb3222dc6f98a8597871b16cef3e293daa4feaa8f652576d0a5e`
- Significant implementation surfaces: `.github/` (4 files), `Cargo.lock/` (1 files), `Cargo.toml/` (1 files), `README.md/` (1 files), `scripts/` (2 files), `src/` (4 files), `tests/` (1 files)
- Refreshed from the current codebase by `decapod rpc --op specs.refresh`
<!-- decapod:codebase-attestation:end -->

## Authentication

The CLI has no authentication or remote identity. Local Decapod sessions and future
release tokens are development/operations state and must never enter Git.

## Authorization

The CLI creates new tone/icon exports and cannot overwrite existing files or publish. Future publication requires the
reviewed release process and authorized credentials. Prompt scope does not permit
publishing an unreviewed implementation branch.

## Data Classification

Public: original source, specs, MIT license and validated original examples.
Private: Waystead content and unverified local drafts. Secret/machine-local:
credentials, authentication state, sessions, logs and execution state.
