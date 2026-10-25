# Intent

## Product mission

Jalapeño is intended as a lightweight, standalone, open-source Rust tool and
library for generating music, sound effects, visual assets, UI elements and
animation through mathematical wave synthesis. Its desired utility and portability
are analogous to FFmpeg, applied to procedural audiovisual generation. The central
model is mathematical asset descriptions rendered deterministically ahead of time
or at runtime, with no conventional asset files required by synthesis itself.

Developers should eventually install the `jalapeno` CLI with `cargo install
jalapeno`, depend on the crate with `cargo add jalapeno`, or embed the same Rust
algorithms in Godot, Unity, Unreal or a custom application. These install commands
are future goals, not currently available publication claims.

## Current acceptance scope

The user explicitly narrowed the first pass after the original broad prompt:
focus on aligned Decapod specs and a simple initial `main.rs`, while accounting
for the original context. This PR therefore supplies a dependency-free Rust binary
with help/version output, MIT licensing, and source-grounded product specs.
`src/main.rs` does not synthesize or export assets. `Cargo.toml` uses `0.1.0` and
disables publishing. No daemon, server, database, telemetry or network dependency
is introduced into the application.

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
7. Publish a small core crate and standalone native binaries through a reviewed,
   coherent release authority; preserve the release requirements in OPERATIONS.md.
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

This stage: minimal entrypoint and faithful product context. Next: implement and
validate a small reusable core and recipes, then the CLI/export demonstration,
then engine integrations and the review-gated v0.1 release. Full typography,
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

- Repository signal fingerprint: `f4950ae07b01d9cf5e8b97ebc737ac50e7c344f798210f9c0118b8c15d5055cd`
- Significant implementation surfaces: `Cargo.lock/` (1 files), `Cargo.toml/` (1 files), `README.md/` (1 files), `scripts/` (1 files), `src/` (1 files)
- Refreshed from the current codebase by `decapod rpc --op specs.refresh`
<!-- decapod:codebase-attestation:end -->
