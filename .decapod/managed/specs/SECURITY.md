# Security and provenance

## Current surface

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

<!-- decapod:capability-overlay:public-api:start -->

Generated capability obligations: `public-api` is declared for this project. These are decisions and proof requirements, not claims of implemented behavior. Resolve each against project source and tests; record unsupported or non-applicable mechanisms explicitly.

## Public API Security Overlay

### Authentication Requirements
- All public endpoints MUST validate authentication tokens
- Token validation MUST include expiry, revocation, and scope checks
- Anonymous access MUST be explicitly documented and justified

### Input Validation
- All request bodies MUST be validated against schemas
- Reject requests with unknown fields (strict schema validation)
- Size limits MUST be enforced on all request bodies

### Rate Limiting
- Limits and enforcement boundaries MUST be selected for this deployment
- Clustered enforcement behavior MUST be documented when applicable
- Client-visible throttling behavior MUST be part of the contract when applicable
<!-- decapod:capability-overlay:public-api:end -->

<!-- decapod:codebase-attestation:start -->

## Codebase Attestation

- Repository signal fingerprint: `f4950ae07b01d9cf5e8b97ebc737ac50e7c344f798210f9c0118b8c15d5055cd`
- Significant implementation surfaces: `Cargo.lock/` (1 files), `Cargo.toml/` (1 files), `README.md/` (1 files), `scripts/` (1 files), `src/` (1 files)
- Refreshed from the current codebase by `decapod rpc --op specs.refresh`
<!-- decapod:codebase-attestation:end -->
