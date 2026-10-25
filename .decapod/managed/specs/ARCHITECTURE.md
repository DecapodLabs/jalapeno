# Architecture

## Current Facts

`Cargo.toml` defines one Rust 2024 package named `jalapeno`, version `0.1.0`,
with no dependencies and publishing disabled. `src/lib.rs` exposes `audio` and
`visual`. `audio::Tone` validates parameters and streams additive mono PCM16 WAV.
`visual::Icon` validates parameters, samples harmonic contours and exports SVG.
`src/main.rs` parses options and delegates synthesis to this public library.
Its output helper writes a temporary sibling and installs it without replacement.
There are no engines, recipe parser, server or persistent application state.

```text
CLI arguments -> src/main.rs -> audio::Tone -> streamed WAV
                            -> visual::Icon -> geometry / SVG
Rust consumers -------------> same public synthesis modules
```

## Direction

The Rust synthesis core is authoritative and engine agnostic. Start with one
publishable `jalapeno` package containing the useful core library and standalone
CLI. Split workspace packages only where dependency isolation or independent
adapter versioning requires it; logical boundaries are not mandatory crates.

```text
CLI / Rust consumers / Godot / Unity / Unreal / custom applications
                         |
                  Rust synthesis core
             audio          visual
                         |
            render buffers and optional exports
```

- `audio`: oscillators and partials, envelopes, modulation, effects, stereo mixing,
  note/tempo/track sequencing, automation, transitions and bounded streaming.
- `visual`: harmonic coefficients, paths, geometry, transforms, icons, UI states,
  patterns, animation, contour morphing and extensible procedural lettering.
- `render`: audio buffers, WAV, SVG, RGBA, optional PNG and explicit optional codecs.
- Recipe schema/validation: mathematical descriptions and deterministic seeded
  parameters shared by CLI and applications, not encoded audio/image payloads.
- `bindings/godot`: thin gdext GDExtension returning Godot images/textures, audio
  and geometry; all synthesis calls the core.
- `bindings/unity`: isolated portable C ABI plus thin C# native-plugin adapter.
- `bindings/unreal`: the same ABI with C++ wrappers; Rust owns no engine objects.

Core synthesis must require no daemon, server, database, network, telemetry or
external executable. Engine libraries, export codecs and heavyweight conveniences
must not enter the default core dependency graph. Prefer dependency-free math and
small feature-gated serialization/export tooling. No adapter reimplements algorithms.

## Concurrency and Runtime Model

Rendering is synchronous. Tone export uses constant scratch storage (2048 bytes)
and no per-sample allocation; preparation/CLI and icon sampling allocate. File
exports block. No hard real-time guarantee is made. Icons have bounded sample
counts and canvas dimensions. No threads, async runtime or background process
are introduced.

## Topology

The diagram above describes current execution in one local process. The planned
core/adapter topology in Direction is future work, with no runtime network links.

## Execution Path

`main` matches help/version or parses `tone`/`icon` options, validates input, then
calls the corresponding core exporter. Numeric/usage errors exit 2; output I/O
errors exit 3. The temporary file is cleaned up on an ordinary export failure.

## Deployment Topology

Current use is `cargo run`/`cargo build` or a Rust dependency by path. CI runs the
same check script on Linux/macOS/Windows. Native Decapod readiness currently
uses a reviewed macOS ARM64 tool layout; it does not prove the hosted matrix ran.
There is no release workflow or engine plugin yet.

## Data and Contracts

`Tone`, `Icon`, `Shape` and `Point` are small public Rust descriptions of synthesis.
They contain parameters, not recorded assets. There is no versioned JSON schema
or stable C ABI. Current argument, output and numeric contracts are in INTERFACES
and SEMANTICS. Existing output paths are never replaced by the CLI.

## Delivery Plan

Complete tones/icons and exact-candidate proof first. Follow with reusable richer
synthesis, recipes and composition, the full menu demo, then engines and releases.

## Risks and Mitigations

Potential aliasing and discontinuities are addressed by omitting Nyquist partials
and applying short fades, without claiming perfect bandlimiting of envelopes.
Bounds limit duration/harmonics/geometry. Decoded-export and mathematical tests
protect real behavior. Specs distinguish the original roadmap from current code.

<!-- decapod:codebase-attestation:start -->

## Codebase Attestation

- Repository signal fingerprint: `a695576554ce8de4e235c7eef5eb01efb229a04ebdea09fc7e921f4f92a786b3`
- Significant implementation surfaces: `.github/` (1 files), `Cargo.lock/` (1 files), `Cargo.toml/` (1 files), `README.md/` (1 files), `scripts/` (2 files), `src/` (4 files), `tests/` (1 files)
- Refreshed from the current codebase by `decapod rpc --op specs.refresh`
<!-- decapod:codebase-attestation:end -->
