# Interfaces

## Inbound Contracts

Current CLI (`src/main.rs`):

```sh
jalapeno tone --hz 440 --seconds 0.4 --partials 1,0.3,0.1 -o tone.wav
jalapeno icon --shape seed --size 128 --phase 0.3 -o seed.svg
```

`tone` accepts `--hz`, `--seconds`, `--rate`, `--gain`, `--partials` (comma-separated
signed amplitudes), and `-o`/`--output`. Defaults are 440 Hz, 0.25 seconds, 48000 Hz,
0.6 gain and one fundamental partial. `icon` accepts `--shape circle|seed|spark`,
`--size`, `--phase` (radians) and output; defaults are seed, 128 and zero rotation.
Output is required. Repeated options and options for the wrong command fail.
No arguments or `--help`/`-h` print help; `--version`/`-V` use Cargo metadata.

The public library exposes `audio::Tone::{validate,write_wav}` and
`visual::Icon::{validate,points,svg}`, with `Shape` and pixel-coordinate `Point`.
Tone exports stream into any `std::io::Write`; icon geometry/SVG can be consumed
without files. The library performs validation and uses `io::Result` for errors.
There is no stable recipe format or engine ABI in this slice.

## Outbound Dependencies

Only the Rust standard library is required at runtime. WAV is mono PCM16,
little-endian RIFF/WAVE; SVG is standalone numeric path markup with fixed colors
and no external resources. There is no encoder executable, network, daemon,
server, database, telemetry, font or engine dependency.

## Data Ownership

The caller owns `Tone` and its harmonic vector. WAV export borrows the writer.
`Icon::points` returns an owned point vector and `Icon::svg` an owned String.
The CLI owns files and writes a temporary sibling, flushes/syncs/closes it, then
hard-links it to an absent destination and removes the temporary name. Existing
outputs are preserved. No native ownership crosses an FFI boundary yet.

## Failure Semantics

Exit 0 means success; exit 2 means invalid command/options or synthesis parameters;
exit 3 means output I/O failure. Errors go to stderr; success prints the output
path. Input validation precedes output creation. A destination that already exists
is an output error and remains unchanged. Ordinary write/link failures remove
the temporary file; a killed process may leave a temporary sibling for inspection.
The CLI does not replace stale temporary files or unsupported hard-link semantics.

## Planned CLI, recipes and integrations

The complete original brief remains future scope:

```sh
jalapeno render audio music.json -o music.wav
jalapeno render audio click.json -o click.wav
jalapeno render visual button.json -o button.svg
jalapeno render visual icon.json -o icon.png
jalapeno validate recipe.json
jalapeno inspect recipe.json
```

Use consistent commands, field-oriented errors, documented exit codes,
machine-readable diagnostics and deterministic filenames when requested. Add a
versioned validated schema containing synthesis parameters and seed, not encoded
assets. Formalize Rust API compatibility and cross-platform numerical tolerances.
Keep optional PNG/codecs separate from the default library.

Godot: Godot 4 gdext/GDExtension API for images, textures, PCM and UI geometry,
with a generated button and interaction-sound demo.

Unity: stable versioned C ABI for audio/visual data, explicit lengths/capacities,
ownership/release and errors; thin C# P/Invoke and procedural button/audio demo.
Unsafe Rust belongs in isolated tested adapters.

Unreal: reuse the ABI with thin C++ wrappers; Unreal owns engine objects. Initially
a practical documented/scaffolded integration is acceptable, with actual build/
engine proof reported separately. All adapters use core algorithms, version
independently and may remain unpublished initially.

<!-- decapod:codebase-attestation:start -->

## Codebase Attestation

- Repository signal fingerprint: `efd11ee45e79cb3222dc6f98a8597871b16cef3e293daa4feaa8f652576d0a5e`
- Significant implementation surfaces: `.github/` (4 files), `Cargo.lock/` (1 files), `Cargo.toml/` (1 files), `README.md/` (1 files), `scripts/` (2 files), `src/` (4 files), `tests/` (1 files)
- Refreshed from the current codebase by `decapod rpc --op specs.refresh`
<!-- decapod:codebase-attestation:end -->
