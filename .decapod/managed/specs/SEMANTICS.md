# Semantics

## Current behavior

The only runtime semantics are argument recognition and process output in
`src/main.rs`. No synthesis, seed handling, mathematical output or recipe
compatibility is implemented or promised by version `0.1.0` in this scaffold.

## Required future audio semantics

Define oscillator phase/frequency units, pitch, sample rate, partial amplitudes,
ADSR timing, note onset/duration, pan law and gain behavior. Sine-derived additive
signals must support harmonic and inharmonic modes for instruments/materials,
with modulation and expressive time variation. Specify tempo/track/automation and
transition behavior, event ordering and streaming block consistency.

Handle invalid/nonfinite or unsupported frequencies, silence, discontinuities,
aliasing, clipping and resource limits explicitly. Exclude partials above Nyquist
or choose another quantified anti-aliasing strategy; modulation sidebands need
specific analysis. Endpoints and joins need continuity tests. Define overload and
limiting behavior rather than silently normalizing away intended dynamics.
WAV must be standard PCM without an external executable, with size/format limits
validated before output. Buffered and streaming APIs must agree within a specified
tolerance. Long-composition working memory must be bounded independently of duration.

## Required future visual semantics

Specify Fourier terms for 2D coordinates/contours, coefficient frequency,
amplitude, phase, transform order, scale, rotation and time. Support closed shapes
and parametrized paths, harmonic composition, interpolation/morphing and animated
coefficients. Define topology compatibility, sampling bounds and interpolation
behavior before declaring shape morphing stable. Procedural lettering is an
extension point, not a complete shaping/layout/font engine requirement for v0.1.

SVG, RGBA and optional PNG must share intended geometry. Specify coordinate
system, pixel layout/stride, color space, alpha convention, fill/stroke behavior
and anti-aliasing. Bound vertices, dimensions and work before rendering. UI states
normal/hover/pressed/focused/disabled must have distinct useful behavior.

## Determinism and compatibility requirements

Specify a seeded generator algorithm, random draw ordering and recipe schema
version. Same recipe/seed and compatible implementation must reproduce intended
output; serialization contains parameters rather than rendered assets. Integer
PRNG reproducibility may be exact, while transcendental floating math and platform
rendering require documented absolute/relative tolerances and image/audio error
metrics. Set tolerances from measurements, including supported platforms; no
universal bit-identity claim. Record implementation compatibility and breaking
schema/numerical changes explicitly. No numerical tolerance has been established
by this first pass.

<!-- decapod:capability-overlay:persistent-state:start -->

Generated capability obligations: `persistent-state` is declared for this project. These are decisions and proof requirements, not claims of implemented behavior. Resolve each against project source and tests; record unsupported or non-applicable mechanisms explicitly.

## Persistent State Semantics Overlay

### Transaction Semantics
- Atomicity boundaries and any non-atomic operations MUST be declared with source and test evidence
- Document observed read-after-write behavior; do not infer consistency from a capability declaration
- Eventual consistency windows MUST be documented

### Migration Semantics
- Migration compatibility, any breaking changes, and their recovery path MUST be explicit and tested
- Migration rollback procedures MUST be documented
- Data integrity checks post-migration

### Recovery Semantics
- Document supported recovery mechanisms and evidence; point-in-time recovery is not implied by persistent-state
- Recovery objectives MUST be selected for the project and recorded as proof obligations
- Recovery test cadence MUST be selected for the project and recorded as a proof obligation
<!-- decapod:capability-overlay:persistent-state:end -->

<!-- decapod:codebase-attestation:start -->

## Codebase Attestation

- Repository signal fingerprint: `f4950ae07b01d9cf5e8b97ebc737ac50e7c344f798210f9c0118b8c15d5055cd`
- Significant implementation surfaces: `Cargo.lock/` (1 files), `Cargo.toml/` (1 files), `README.md/` (1 files), `scripts/` (1 files), `src/` (1 files)
- Refreshed from the current codebase by `decapod rpc --op specs.refresh`
<!-- decapod:codebase-attestation:end -->
