# Jalapeño

Jalapeño is intended to become a small, engine-independent Rust toolkit for
procedural audio, geometry, UI and animation through mathematical wave synthesis.

This first pass contains a dependency-free CLI entrypoint and Decapod
specifications preserving the complete product brief. Synthesis, export formats,
recipes, engine adapters and release automation are **planned, not implemented**.
The latest task deliberately limits code to this initial scaffold.

```sh
cargo run -- --help
cargo run -- --version
```

The executable is named `jalapeno`. No arguments show help; unsupported arguments
exit with code 2. This package starts at `0.1.0` and has `publish = false` until a
reviewed implementation and release process are ready. No crate or binary release
has been published. There are no runtime dependencies or background services.

Start with the [spec index](.decapod/managed/specs/README.md).
[Intent](.decapod/managed/specs/INTENT.md) records the mission and delivery stages;
[architecture](.decapod/managed/specs/ARCHITECTURE.md) defines the planned core and
adapter boundaries; [interfaces](.decapod/managed/specs/INTERFACES.md) describes the
future public contracts. The remaining specs cover numerical semantics, security,
operations, and validation. Each distinguishes current code from future work.

Development checks:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
```

The full original brief is accounted for across the specs, including the music,
visual, engine integration, open-source release, and evidence requirements. MIT
licensed. Contributions and future implementation should use Conventional Commits
and GitHub PR review; neither a specification nor a task claim proves a feature
works.
