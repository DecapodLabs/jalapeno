# Validation

## Proof Surfaces

For this narrowed first pass verify:

1. `cargo fmt --all -- --check`.
2. `cargo clippy --all-targets -- -D warnings`.
3. `cargo test` (currently no algorithm tests; compilation is not synthesis proof).
4. CLI smoke: no arguments/help show the same purpose and scope; version matches
   Cargo metadata; unsupported/multiple arguments return usage exit 2 on stderr.
5. Read all eight specs and README against `Cargo.toml`/`src/main.rs`: current
   claims must match actual code and every original requirement must be preserved
   as planned work, without template placeholders or false feature claims.
6. Decapod exact-candidate validation before PR creation, with evidence and any
   concrete limitations recorded separately from hosted CI/publication proof.

No DSP, image, recipe, engine or performance checks can pass for unimplemented
features. There is no claim that a unit-test count of zero verifies future behavior.

## Required future CI and acceptance evidence

Establish GitHub Actions for formatting, `cargo clippy --workspace --all-targets
--all-features -- -D warnings`, `cargo test --workspace --all-features`, focused
feature and no-default-features tests, cargo package/publish dry-run and package
content verification, native CLI smoke, adapter compilation and engine smoke on
supported platforms. Generate example assets using the real public API in CI and
decode/measure their content rather than only asserting file existence.

Audio tests must cover waveform/pitch properties, envelopes, continuity, bounded
output, Nyquist/modulation behavior, clipping, silence, invalid frequencies,
resource limits, expressive sound audibility, mixing/pan, sequencing/automation/
transitions, WAV decoding and streaming block invariance/bounded memory.

Visual tests must cover harmonic evaluation/transforms, closed contours, geometry
bounds, morph compatibility, animation, five UI states, pixel/alpha conventions,
SVG parsing and PNG/RGBA decoding. Include invalid/nonfinite/resource-limit input.

Recipe tests must cover seeded repeatability, roundtrip serialization, unknown/
unsupported schemas and fields, compatibility and measured cross-platform error.
ABI tests must cover lengths/capacities, explicit ownership and release behavior,
invalid input and safe failure paths; isolate and test unsafe code. Verify real
Godot and Unity procedural-button/sound integrations where supported, and report
Unreal compilation/engine evidence separately from documented scaffolding.

Measure synthesis throughput, peak memory, binary size and runtime initialization
with reproducible benchmark scope/toolchain/hardware context. Do not claim hard
real-time safety on allocating/blocking paths or introduce acceleration without
measured need. No such measurements exist for this scaffold.

## Original-prompt coverage map

| Brief area | Preserved specification |
| --- | --- |
| Mission, references, original examples, complete menu demonstration | INTENT |
| Small core, logical modules, features, adapter dependency boundaries | ARCHITECTURE |
| CLI, library, schema, Godot/Unity/Unreal contracts | INTERFACES |
| Audio/visual mathematics, streaming, determinism and tolerance | SEMANTICS |
| Private provenance, secrets, unsafe code, resource safeguards | SECURITY |
| Crate name/version, review-gated release authority and artifacts | OPERATIONS |
| Tests, CI, package/engine/export proof, benchmarks | VALIDATION |

The prompt is preserved as an actionable engineering brief, not evidence that
those future capabilities are already delivered.





<!-- decapod:codebase-attestation:start -->

## Codebase Attestation

- Repository signal fingerprint: `9f304068b8a6db75eeba7619efccbd478ee5965f41633d1a59a452343388cbf6`
- Significant implementation surfaces: `Cargo.lock/` (1 files), `Cargo.toml/` (1 files), `README.md/` (1 files), `scripts/` (1 files), `src/` (1 files)
- Refreshed from the current codebase by `decapod rpc --op specs.refresh`
<!-- decapod:codebase-attestation:end -->

## Promotion Gates

The initial branch needs actual local checks and Decapod exact-candidate readiness
before a PR. PR review is the acceptance boundary. Publication remains disabled;
local readiness cannot stand in for future hosted matrix or release proof.

## Evidence Artifacts

`scripts/check.py` executes formatting, lint, compilation and CLI smoke checks.
Decapod stores bound execution receipts locally; the governance document records
scope and evidence references. Keep logs and machine-local tool paths out of Git.

## Regression Guardrails

Reject changes that claim planned synthesis/adapters/releases are already working,
lose a major requirement from this brief, publish prematurely, or add unrelated
runtime dependencies. Expand tests when meaningful synthesis behavior is added.
