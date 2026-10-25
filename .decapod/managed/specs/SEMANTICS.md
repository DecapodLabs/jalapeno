# Semantics

## State Machines

Current rendering is synchronous: parse -> validate -> render to a temporary
sibling -> flush/sync/close -> install at an absent output path -> cleanup.
Invalid input exits before output creation. A Rust caller can export to its own
writer or consume icon points/SVG directly without CLI filesystem operations.

## Implemented numerical behavior

`audio::Tone` sums sine partials at integer multiples of the fundamental. Signed
amplitudes are normalized by max(1, sum(abs(amplitudes))); zero gain/amplitudes
produce silence. Terms at or above sample-rate/2 are omitted with their weight
retained in normalization. A five-millisecond raised-cosine fade (shortened for
small durations) gives exact zero endpoints. Float samples round to PCM16 using
32767 full scale. This limits simple oscillator aliasing, not all envelope-sideband
energy; perfect bandlimiting and hard real-time safety are not claimed.

Sample rate is 8000..192000, fundamental is finite and >=1 Hz but below Nyquist,
duration 0.01..600 seconds, gain 0..1, and 1..32 finite amplitudes in -1..1.
Output length is ceil(seconds * sample_rate). Work is limited to 200 million
sample-partial pairs; scratch output storage is always 2048 bytes.

`visual::Icon` samples x=cos(t)+a*cos(k*t), y=sin(t)-a*sin(k*t), applies rotation,
scales by 0.32*canvas size and centers the result. Circle uses a=0; seed uses
k=3,a=0.2; spark uses k=5,a=0.28. Size is 16..2048, rotation finite within +/-tau,
and requested samples 32..4096. SVG samples 128 points and closes the contour.
Coordinates use six decimals, fixed fill/stroke colors and a two-pixel round stroke.

Same inputs on the same compatible binary repeat exactly in tests. Floating
trigonometry is not claimed bit-identical across platforms or compiler versions;
no published cross-platform tolerance has yet been measured. Seeded generation,
recipe compatibility and a quantified cross-platform policy remain future work.

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

- Repository signal fingerprint: `efd11ee45e79cb3222dc6f98a8597871b16cef3e293daa4feaa8f652576d0a5e`
- Significant implementation surfaces: `.github/` (4 files), `Cargo.lock/` (1 files), `Cargo.toml/` (1 files), `README.md/` (1 files), `scripts/` (2 files), `src/` (4 files), `tests/` (1 files)
- Refreshed from the current codebase by `decapod rpc --op specs.refresh`
<!-- decapod:codebase-attestation:end -->

## Invariants

Validated tones cannot exceed their gain headroom before PCM rounding; output
endpoints are zero. Work and memory are bounded and invalid input emits no bytes.
Validated icon contours remain within the canvas with stroke margin. The core
uses no unsafe code, external asset inputs, networking or engine dependency.
Publication remains disabled. Broader numerical/schema guarantees above remain
future obligations rather than properties of absent features.
