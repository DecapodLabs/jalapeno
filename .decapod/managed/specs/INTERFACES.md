# Interfaces

## Inbound Contracts

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



<!-- decapod:codebase-attestation:start -->

## Codebase Attestation

- Repository signal fingerprint: `9f304068b8a6db75eeba7619efccbd478ee5965f41633d1a59a452343388cbf6`
- Significant implementation surfaces: `Cargo.lock/` (1 files), `Cargo.toml/` (1 files), `README.md/` (1 files), `scripts/` (1 files), `src/` (1 files)
- Refreshed from the current codebase by `decapod rpc --op specs.refresh`
<!-- decapod:codebase-attestation:end -->

## Outbound Dependencies

The current program calls only Rust's standard library and process stdout/stderr.
No external executable, crate dependency, service or engine is used at runtime.

## Data Ownership

`main` owns its argument vector until process exit. Output is text; no borrowed
buffer or native ownership crosses a public boundary. Future ABI ownership must
be specified explicitly before adapter implementation.

## Failure Semantics

Unsupported arguments emit a fixed stderr diagnostic and exit 2. Help/version
exit 0. Recipe, rendering, export and engine error semantics are future design
work; no success path for those operations currently exists.
