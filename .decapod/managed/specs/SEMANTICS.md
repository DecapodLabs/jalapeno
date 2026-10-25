# Semantics

## State Machines

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



<!-- decapod:codebase-attestation:start -->

## Codebase Attestation

- Repository signal fingerprint: `528100f5424d15bf5d4859440a51e0e33e7794288a25ab041050f96ed5d99a01`
- Significant implementation surfaces: `Cargo.lock/` (1 files), `Cargo.toml/` (1 files), `README.md/` (1 files), `scripts/` (1 files), `src/` (1 files)
- Refreshed from the current codebase by `decapod rpc --op specs.refresh`
<!-- decapod:codebase-attestation:end -->

## Invariants

Current behavior terminates after one response, performs no synthesis or external
I/O beyond process text streams, and does not publish. Future mathematical,
determinism and bounded-resource invariants are obligations described above,
not established properties of an absent implementation.
