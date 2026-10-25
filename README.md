# Jalapeño

A small Rust toolkit for procedural **tones and icons**. Generate additive audio
as PCM WAV and harmonic geometry as SVG, using the same dependency-free library
as the CLI. No recordings, bitmap assets, engines, network or external encoder
are required.

```sh
cargo run -- tone --hz 440 --seconds 0.4 --partials 1,0.3,0.1 -o tone.wav
cargo run -- icon --shape seed --size 128 -o seed.svg
cargo run -- icon --shape spark --phase 0.4 -o spark.svg
cargo run -- --help
```

Tone options are frequency, duration, sample rate, gain and signed harmonic
amplitudes. Output is mono PCM16 WAV. High partials at or above Nyquist are omitted;
normalization preserves headroom and short raised-cosine fades soften the ends.
WAV rendering uses a fixed 2 KB scratch buffer, independent of duration.

Icon shapes are `circle`, `seed` and `spark`. Each is an original harmonic contour;
size sets the canvas and phase rotates it in radians. SVG uses numeric geometry
with fixed colors and no external resources. Applications can consume points
directly. Existing output files are preserved; completed exports are installed
from a temporary sibling file. Exit codes: 0 success, 2 input, 3 output I/O.

The library exposes the same primitives:

```rust
use jalapeno::{audio::Tone, visual::Icon};
let mut wav = Vec::new();
Tone::default().write_wav(&mut wav)?;
let svg = Icon::default().svg()?;
let contour = Icon::default().points(128)?;
# Ok::<(), std::io::Error>(())
```

This is an intentionally tight first pass, version `0.1.0`, under the MIT license.
Publishing is disabled until a reviewed release process is ready. The complete
original toolkit brief—music sequencing, recipes, richer audiovisual synthesis,
Godot/Unity/Unreal, PNG and native releases—is preserved as future requirements
in the [Decapod specs](.decapod/managed/specs/README.md). These are not current
features. Identical inputs reproduce the intended output; cross-platform floating
math is not promised bit-identical.

Run the complete build/lint/test/export check:

```sh
python3 scripts/check.py
```

It decodes generated WAV and SVG, checks pitch/partials, boundaries, fades,
repeatability, invalid inputs and preservation of existing output. GitHub CI runs
the same script on Linux, macOS and Windows. This first pass has no engine or
release assurance claim.

Decapod's reviewed local readiness profile currently targets macOS ARM64 with
Xcode at its standard location and Rust 1.99.0 (`b940084d7`). To prepare it:

```sh
python3 scripts/prepare-proof.py
# Checkpoint/commit changes through the repository governance workflow, then:
decapod validate
```

Preparation copies already installed Rust tools to a fixed temporary prefix and
verifies their bytes. It performs no download or application build. The proof
contract records standardized installation paths, not a user's home directory,
credentials or machine state. Decapod measures declared tools before/after the
check, runs against an immutable source snapshot, and keeps generated outputs in
fresh scratch. Other platforms can run `scripts/check.py`; adding native Decapod
profiles remains separate work. GitHub PR review is the acceptance boundary.
