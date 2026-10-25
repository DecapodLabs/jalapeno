# Intent

## Product Outcome

Jalapeño is intended as a lightweight, standalone, open-source Rust tool and
library for generating music, sound effects, visual assets, UI elements and
animation through mathematical wave synthesis. Its desired utility and portability
are analogous to FFmpeg, applied to procedural audiovisual generation. The central
model is mathematical asset descriptions rendered deterministically ahead of time
or at runtime, with no conventional asset files required by synthesis itself.

Developers should eventually install the `jalapeno` CLI with `cargo install
jalapeno`, depend on the crate with `cargo add jalapeno`, or embed the same Rust
algorithms in Godot, Unity, Unreal or a custom application. These install commands
are future goals beyond the initial `0.0.1` crate. Publication remains gated on
the reviewed release PR, required credentials and successful release workflow.

## Scope

The latest user clarification asks for a tight functional solution: tones and
icons. This first pass now implements a dependency-free Rust library and CLI:
`audio::Tone` streams additive mono PCM16 WAV and `visual::Icon` produces sampled
harmonic geometry and SVG. `src/main.rs` exposes `tone` and `icon`, plus help and
version. Tests exercise the exported formats and underlying math. The Cargo
package and native CLI distribution are prepared for an initial `0.0.1` release;
no publication or release artifact exists yet.

The earlier direction to account for the whole prompt remains in force: broader
music, visual, recipe and engine requirements below remain roadmap. Release
automation is configured around release-plz for version/changelog PRs and
cargo-dist for standalone binaries. It publishes only after merging the intended
release PR. No service or runtime network is introduced. The original brief is
not a claim of complete toolkit delivery.

## Preserved future product requirements

1. Keep an authoritative engine-independent Rust synthesis core, usable as a
   library and CLI, without engine dependencies in the default library graph.
2. Audio: configurable oscillators, additive sine partials, ADSR envelopes,
   phase/frequency modulation, pitch, duration, stereo, sample rate, effects,
   mixing, expressive piano/flute/UI/impact/organic-material sounds; tempo,
   tracks, notes, automation, transitions and bounded-memory long-form streaming.
3. Visuals: Fourier-based geometry, harmonic shape composition, parametrized
   paths, contours, transforms, animated coefficients and morphing; original
   icons/resource glyphs, buttons, borders, backgrounds and simple logo primitives;
   normal, hover, pressed, focused and disabled states. Explore procedural lettering
   as an extensible capability without claiming a complete v0.1 font engine.
4. Render into application-owned geometry/pixel/audio buffers without files;
   standard PCM WAV without FFmpeg, SVG, RGBA and optionally PNG. Other codecs
   and rendering conveniences must be explicit optional features.
5. Seeded procedural generation, serializable synthesis recipes, a versioned
   schema, validation, documented compatibility and realistic numerical tolerances.
6. Thin Godot gdext/GDExtension and Unity C ABI/PInvoke integrations with practical
   procedural button/sound demos. Reuse the C ABI for Unreal C++ without Rust
   owning Unreal objects; document/scaffold Unreal first if needed for release scope.
7. Publish a small core crate and standalone native binaries through release-plz
   and cargo-dist, with reviewed release PRs, one tag/release authority, package
   verification and credential separation; preserve requirements in OPERATIONS.md.
8. Meaningful mathematical, resource, format, ABI, CLI and engine checks;
   measurements of throughput, memory, binary size and initialization. Require
   evidence for claims and use benchmarks before SIMD/GPU/async complexity.
9. Plain-language installation, sound/button/icon/animation and engine examples,
   feature/platform matrix, rationale, security/FFI limits, release troubleshooting
   and concise roadmap. Use an MIT license consistent with DecapodLabs.

## First complete future demonstration

A game-menu button whose contour, five interaction states, hover animation,
confirmation sound and rendering all come from original mathematical recipes.
Examples must call the actual public library, not embed pre-rendered samples.

## References and provenance

- Repository: https://github.com/DecapodLabs/jalapeno
- Release/governance reference: https://github.com/DecapodLabs/decapod
- Smaller distinct release reference: https://github.com/DecapodLabs/dactyl
- Private practical reference: https://github.com/alexhraber/waystead
- Godot integration: https://godot-rust.github.io/
- Zanzlanz, “How I Released A Game That Has No Assets”:
  https://www.youtube.com/watch?v=Qr3VsZYQy4s

The video's ambition includes complete asset-free visual languages rather than
only simple sine sounds. Direct video retrieval was throttled during orientation;
no completed video study is claimed. Revisit the primary source before algorithm
selection. Waystead's generalizable ideas include harmonic piano/flute voices,
resonant material sounds and generated geometry. Its assets, compositions, logos,
coefficients and proprietary material must not be copied or published.

## Stages and non-goals

This stage: working additive tones and harmonic icons, decoded-export checks,
faithful full product context, and a release pipeline prepared for `0.0.1`.
Next: richer reusable synthesis, versioned recipes and composition, then the full
menu demonstration and engine integrations. Full typography,
heavier acceleration and advanced audiovisual composition remain later milestones.
Do not advertise any of these as implemented until source and proof support them.

<!-- decapod:declared-capabilities:start -->

## Declared Capability Surfaces

These declarations select authoring and proof obligations; they do not attest implemented runtime guarantees.

- `event-driven`
- `infrastructure-management`
- `persistent-state`
- `public-api`

<!-- decapod:declared-capabilities:end -->

<!-- decapod:codebase-attestation:start -->

## Codebase Attestation

- Repository signal fingerprint: `d96f4192e1310151b40c0c07a2cc5a552c7862005f66a8ac76c49256eec918b1`
- Significant implementation surfaces: `.github/` (4 files), `Cargo.lock/` (1 files), `Cargo.toml/` (1 files), `README.md/` (1 files), `scripts/` (2 files), `src/` (4 files), `tests/` (1 files)
- Refreshed from the current codebase by `decapod rpc --op specs.refresh`
<!-- decapod:codebase-attestation:end -->

## Constraints

Keep the functional slice small and dependency-free. Preserve the original brief
as roadmap, keep publication behind the release workflow and reviewed release PR,
and keep private reference material, credentials and runtime machine state out
of Git. Standardized tool installation
paths declare the readiness environment without embedding a user's home directory.

## Acceptance Criteria

The public core and CLI must generate audible, bounded WAV tones and distinct,
bounded harmonic SVG icons. Validate exports by decoding them, test mathematical
properties and invalid input, preserve existing output, and document actual limits.
Keep all eight specs aligned and run full exact-candidate Decapod validation before
opening the implementation PR. No engine or publication completion claim is allowed.
