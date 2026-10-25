# Validation

## Proof Surfaces

The required current check is `python3 scripts/check.py`:

1. `cargo fmt --all -- --check`.
2. `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
3. `cargo test --workspace --all-features` and a locked binary build.
4. Mathematical/export tests for frequency, partials, headroom, zero endpoints,
   continuity, silence, Nyquist omission, determinism, resource/numeric rejection,
   bounded WAV writes and writer failures; distinct, bounded, closed rotating icons.
5. Native CLI smoke: decode generated WAV using Python's wave/struct and SVG using
   ElementTree, check output content, repeated-input bytes, input errors, duplicate
   options, no unintended files and preservation of an existing destination.
6. Read all specs/README against the actual public core and CLI; future features
   remain labeled as planned. Refresh attestations and source-bound spec reviews.
7. Full exact-candidate Decapod readiness before PR creation.

The check writes Cargo output and exports only to fresh temporary scratch, suitable
for the controller's immutable source snapshot. GitHub CI runs the same command on
Linux, macOS and Windows. Local execution proves one native platform; matrix
success must come from actual hosted runs. Recipe, engine, PNG, performance and
release checks cannot pass for features that remain unimplemented.

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

- Repository signal fingerprint: `d96f4192e1310151b40c0c07a2cc5a552c7862005f66a8ac76c49256eec918b1`
- Significant implementation surfaces: `.github/` (4 files), `Cargo.lock/` (1 files), `Cargo.toml/` (1 files), `README.md/` (1 files), `scripts/` (2 files), `src/` (4 files), `tests/` (1 files)
- Refreshed from the current codebase by `decapod rpc --op specs.refresh`
<!-- decapod:codebase-attestation:end -->

## Promotion Gates

The branch requires actual core/export checks and full Decapod exact-candidate
readiness before a PR. The tracked `repo.pr-readiness.v1` contract binds reviewed
workflow/source and command digests, standardized installed tool paths and the
Rust/Python/linker/SDK closure. `scripts/prepare-proof.py` stages existing trusted
Rust tools outside the checkout; it performs no build or validation. The current
native Decapod profile supports macOS ARM64 with standard Xcode and Rust 1.99.0.
It makes no hosted matrix or release assurance claim.

GitHub PR review remains the acceptance boundary. Publication is disabled.
Neither project-check nor directly running the test script substitutes for full
Decapod readiness. A failed gate must be repaired and rerun on the exact candidate.

## Evidence Artifacts

`tests/synthesis.rs` covers public mathematical/export behavior.
`scripts/check.py` covers the actual native CLI and decoded artifacts.
Decapod stores bound execution receipts locally; the governance record carries
scope/evidence references. Hosted check runs prove the CI matrix separately.
Logs, binaries, credentials and machine runtime state are excluded from Git.

## Regression Guardrails

Keep the functional tone/icon slice dependency-free, preserve outputs on failure,
and reject unbounded/nonfinite input. Protect meaningful math and decoded formats,
not just file existence. Do not lose original requirements, advertise roadmap
features as delivered, or enable publication prematurely. Add appropriate tests
with later synthesis, recipe, adapter or release behavior.
