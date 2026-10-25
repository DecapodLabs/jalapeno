# Architecture

## Current Facts

`Cargo.toml` defines one Rust 2024 binary package named `jalapeno`, version `0.1.0`,
with no dependencies and publishing disabled. `src/main.rs::main` reads command
arguments, prints help or the package version, and returns success or usage exit
code 2. There is no library, synthesis engine, asset schema, export pipeline or
engine adapter yet. No filesystem persistence or service process is needed.

```text
shell arguments -> src/main.rs -> stdout help/version or stderr usage error
                              -> exit 0 or 2
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

Separate recipe validation/preparation from rendering. Bounded block processing
should enable long compositions without buffering the entire output. Applications
must also consume geometry and pixel/audio buffers directly. Define ownership,
limits and failure behavior before FFI. Only claim real-time safety for measured,
allocation-free/nonblocking paths; preparation and convenience exports may allocate.
Use benchmarks before choosing SIMD, GPU acceleration or asynchronous runtimes.

No numerical representation, ABI layout or render algorithm is frozen in this
first pass. SEMANTICS.md records requirements for those later decisions.



<!-- decapod:codebase-attestation:start -->

## Codebase Attestation

- Repository signal fingerprint: `528100f5424d15bf5d4859440a51e0e33e7794288a25ab041050f96ed5d99a01`
- Significant implementation surfaces: `Cargo.lock/` (1 files), `Cargo.toml/` (1 files), `README.md/` (1 files), `scripts/` (1 files), `src/` (1 files)
- Refreshed from the current codebase by `decapod rpc --op specs.refresh`
<!-- decapod:codebase-attestation:end -->

## Topology

The two text diagrams above show the actual one-process CLI and the intended
future core/adapter topology. There are no runtime services or network links.

## Execution Path

`main` collects OS arguments, matches a supported option and prints a fixed
message; unsupported argument lists return `ExitCode::from(2)`.

## Deployment Topology

Current deployment is local `cargo run`/`cargo build`. There is no hosted
application or engine plugin. Native release archives are a future requirement.

## Data and Contracts

No persisted application data or recipe schema exists yet. The current argument
contract and planned Rust/recipe/ABI surfaces are in INTERFACES.md.

## Delivery Plan

First preserve the product brief and establish the entrypoint. Follow with a
validated small core, recipes and export examples, then engine and release work.

## Risks and Mitigations

The main current risk is confusing the complete brief with delivered features.
README and every spec mark the boundary. Future numerical, resource, ownership
and release risks have explicit requirements in the corresponding specs.
