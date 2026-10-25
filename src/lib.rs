//! Small procedural audio and harmonic geometry primitives.
//!
//! Exporters use only the Rust standard library. No engine or external encoder
//! is required. Floating-point results are numerically stable, not guaranteed
//! bit-identical across platforms or compiler versions.
#![forbid(unsafe_code)]

pub mod audio;
pub mod visual;
