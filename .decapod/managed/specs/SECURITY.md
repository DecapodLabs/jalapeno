# Security and provenance

## Threat Model

`src/main.rs` prints fixed help/version/usage text and does not read recipes,
write files, open sockets, load plugins or invoke executables. `Cargo.toml` has no
dependencies and `publish = false`. Decapod is development governance, not an
application runtime dependency. Keep credentials, sessions, authentication state,
logs, generated output and machine state outside Git.

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

Future release credentials belong in the existing DecapodLabs GitHub App/release
environment conventions where authorized and available. Use least privilege,
review-gated publishing and one release writer. Do not publish from an unreviewed
feature branch or replace a taken crate name silently. Verify packages exclude
engine tooling, credentials and machine state. MIT licensing is in `LICENSE`.



<!-- decapod:codebase-attestation:start -->

## Codebase Attestation

- Repository signal fingerprint: `528100f5424d15bf5d4859440a51e0e33e7794288a25ab041050f96ed5d99a01`
- Significant implementation surfaces: `Cargo.lock/` (1 files), `Cargo.toml/` (1 files), `README.md/` (1 files), `scripts/` (1 files), `src/` (1 files)
- Refreshed from the current codebase by `decapod rpc --op specs.refresh`
<!-- decapod:codebase-attestation:end -->

## Authentication

The CLI has no authentication or remote identity. Local Decapod sessions and future
release tokens are development/operations state and must never enter Git.

## Authorization

The initial CLI cannot modify assets or publish. Future publication requires the
reviewed release process and authorized credentials. Prompt scope does not permit
publishing an unreviewed implementation branch.

## Data Classification

Public: original source, specs, MIT license and validated original examples.
Private: Waystead content and unverified local drafts. Secret/machine-local:
credentials, authentication state, sessions, logs and execution state.
