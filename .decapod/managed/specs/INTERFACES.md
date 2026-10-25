# Interfaces

## Implemented CLI

`src/main.rs::main` implements the following complete current contract:

| Invocation | Output | Exit |
| --- | --- | --- |
| `jalapeno` | Help on stdout | 0 |
| `jalapeno --help` or `-h` | Same help on stdout | 0 |
| `jalapeno --version` or `-V` | `jalapeno 0.1.0` from Cargo package metadata | 0 |
| Other arguments, including extra arguments | Fixed usage error on stderr | 2 |

No asset generation command currently exists. `Cargo.toml` exposes no library,
features or ABI. The following contracts are proposals preserved from the brief.

## Planned CLI

```sh
jalapeno render audio music.json -o music.wav
jalapeno render audio click.json -o click.wav
jalapeno render visual button.json -o button.svg
jalapeno render visual icon.json -o icon.png
jalapeno validate recipe.json
jalapeno inspect recipe.json
```

Use a consistent command design, useful field-oriented errors, documented exit
codes, machine-readable diagnostics and deterministic filenames when requested.
PNG/extra codecs should fail clearly when their explicit feature is unavailable.
Define overwrite behavior and resource budgets before implementing export.

## Planned library and recipes

Expose reusable instruments/oscillators, score events, envelopes, modulation,
effects, harmonic geometry/coefficient manipulation, morphing and time sampling.
Provide direct buffers and geometry as well as exporters. A recipe describes
synthesis, not a pre-rendered asset. Specify schema version, seed, mathematical
parameters, validation, supported versions and compatibility policy. Document
cross-platform tolerances rather than asserting universal floating-point identity.
The exact Rust types and JSON layout remain undecided; examples in the original
brief do not freeze an unimplemented API.

## Planned engine/ABI contracts

Godot: Godot 4 gdext/GDExtension API for images, textures, PCM and UI geometry,
with a minimal generated button and interaction-sound demo.

Unity: stable versioned C ABI for audio and visual data with explicit lengths,
capacities, ownership, release functions and error results; C# P/Invoke adapter
and a procedural button/audio demo. Unsafe Rust is isolated and tested.

Unreal: reuse the same ABI with thin C++ wrappers; Unreal owns engine objects.
A documented practical path or scaffold is acceptable initially, with actual
compilation/engine verification identified separately. Adapters depend on the core,
remain independently versionable and need not initially publish to crates.io.

<!-- decapod:capability-overlay:public-api:start -->

Generated capability obligations: `public-api` is declared for this project. These are decisions and proof requirements, not claims of implemented behavior. Resolve each against project source and tests; record unsupported or non-applicable mechanisms explicitly.

## Public API Capability Overlay

### API Contract Requirements
- All public endpoints MUST define explicit request/response schemas
- Versioning strategy MUST be documented (URL path or header-based)
- All public endpoints MUST implement idempotency for mutating operations
- Rate limiting and pagination MUST be implemented for list endpoints

### Compatibility Guarantees
- Backward-compatible changes ONLY within a version
- Breaking changes require new version (v1, v2, etc.)
- Deprecation and removal policy MUST be selected for this project and proven against its consumers

### Security Requirements
- All public endpoints MUST implement authentication
- Abuse-control enforcement point MUST be a documented project decision
- Input validation MUST reject malformed requests with typed errors
<!-- decapod:capability-overlay:public-api:end -->

<!-- decapod:codebase-attestation:start -->

## Codebase Attestation

- Repository signal fingerprint: `f4950ae07b01d9cf5e8b97ebc737ac50e7c344f798210f9c0118b8c15d5055cd`
- Significant implementation surfaces: `Cargo.lock/` (1 files), `Cargo.toml/` (1 files), `README.md/` (1 files), `scripts/` (1 files), `src/` (1 files)
- Refreshed from the current codebase by `decapod rpc --op specs.refresh`
<!-- decapod:codebase-attestation:end -->
