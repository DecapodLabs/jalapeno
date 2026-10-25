# Architecture

## Implemented first pass

`Cargo.toml` defines one Rust 2024 binary package named `jalapeno`, version `0.1.0`,
with no dependencies and publishing disabled. `src/main.rs::main` reads command
arguments, prints help or the package version, and returns success or usage exit
code 2. There is no library, synthesis engine, asset schema, export pipeline or
engine adapter yet. No filesystem persistence or service process is needed.

```text
shell arguments -> src/main.rs -> stdout help/version or stderr usage error
                              -> exit 0 or 2
```

## Planned design, not implemented

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

## Ownership and performance direction

Separate recipe validation/preparation from rendering. Bounded block processing
should enable long compositions without buffering the entire output. Applications
must also consume geometry and pixel/audio buffers directly. Define ownership,
limits and failure behavior before FFI. Only claim real-time safety for measured,
allocation-free/nonblocking paths; preparation and convenience exports may allocate.
Use benchmarks before choosing SIMD, GPU acceleration or asynchronous runtimes.

No numerical representation, ABI layout or render algorithm is frozen in this
first pass. SEMANTICS.md records requirements for those later decisions.

<!-- decapod:capability-overlay:persistent-state:start -->

Generated capability obligations: `persistent-state` is declared for this project. These are decisions and proof requirements, not claims of implemented behavior. Resolve each against project source and tests; record unsupported or non-applicable mechanisms explicitly.

## Persistent State Architecture Overlay

### State Ownership
- Each entity type MUST have a designated state owner
- State ownership boundaries MUST be explicitly documented
- Cross-boundary state access MUST go through defined interfaces

### Transaction Boundaries
- Multi-entity mutation atomicity and partial-failure treatment MUST be documented and proven for the selected store
- Transaction boundaries MUST be documented in ARCHITECTURE.md
- Document compensation or other partial-failure treatment if distributed operations exist

### Storage Abstraction
- Storage ownership, consistency behavior, and access boundaries MUST be explicit
- Portability or swappable implementations are project decisions, not universal requirements
- Migration and rollback treatment MUST match the selected storage technology
<!-- decapod:capability-overlay:persistent-state:end -->

<!-- decapod:codebase-attestation:start -->

## Codebase Attestation

- Repository signal fingerprint: `f4950ae07b01d9cf5e8b97ebc737ac50e7c344f798210f9c0118b8c15d5055cd`
- Significant implementation surfaces: `Cargo.lock/` (1 files), `Cargo.toml/` (1 files), `README.md/` (1 files), `scripts/` (1 files), `src/` (1 files)
- Refreshed from the current codebase by `decapod rpc --op specs.refresh`
<!-- decapod:codebase-attestation:end -->
