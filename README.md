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

This is an intentionally tight first pass, targeting the initial `0.0.1` crate
release under the MIT license. The complete
original toolkit brief—music sequencing, recipes, richer audiovisual synthesis,
Godot/Unity/Unreal, PNG and native releases—is preserved as future requirements
in the [Decapod specs](.decapod/managed/specs/README.md). These are not current
features. Identical inputs reproduce the intended output; cross-platform floating
math is not promised bit-identical.

Run the complete build/lint/test/export and release-package check:

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

## Releases

Releases use one authority: release-plz opens version/changelog PRs from
Conventional Commits; after the intended release PR is merged, release-plz
publishes the crate and creates the `vX.Y.Z` tag. cargo-dist is the sole GitHub
Release creator and reacts to that tag to build standalone archives and checksums
for macOS ARM64 and x86_64, Linux ARM64 and x86_64, and Windows x86_64. It uploads
platform installers, checksums and build attestations. The release workflow is
serialized to avoid competing publishers. Crate publishing runs in the Actions
`release` environment only for a merged release PR. Configure that environment
with the registry token and, if wanted, required reviewers for a second approval.

For the initial `0.0.1` publication, the shared DecapodLabs release-manager app
provides `GH_APP_ID` and `GH_APP_PRIVATE_KEY` to the repository. Add a scoped
`CARGO_REGISTRY_TOKEN` to the Actions `release` environment. Then merge the setup
PR. A main push opens/updates the `release-plz` PR; after it is merged, the
release workflow reruns the full check and package dry-run on Linux, macOS and
Windows before it publishes and creates the `v0.0.1` tag. The cargo-dist workflow
then uploads platform archives, installers, checksums and build attestations to
the GitHub Release. Never run `cargo publish` manually from a feature branch.
crates.io permits configuring Trusted Publishing only after the initial crate
exists; migrate the publisher to OIDC in a reviewed follow-up.

Before each release, inspect the package with `cargo package --list --locked`
and `cargo publish --dry-run --locked`. The package whitelist excludes CI,
governance state, scripts and engine adapters. If release-plz proposes an
unexpected version, correct its release PR before merging. `decapod release check`
checks version/changelog/lockfile consistency. A failed or interrupted release
must be reconciled against the existing tag and registry version before retrying.
