//! Harmonic contour icons and dependency-free SVG export.
use std::{f64::consts::TAU, io};

/// Original mathematical contour presets. All are editable harmonic geometry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shape {
    Circle,
    Seed,
    Spark,
}
/// One point on a contour, in canvas pixels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
/// Reusable icon description. Phase rotates the contour, in radians.
#[derive(Clone, Copy, Debug)]
pub struct Icon {
    pub shape: Shape,
    pub size: u32,
    pub phase: f64,
}
impl Default for Icon {
    fn default() -> Self {
        Self {
            shape: Shape::Seed,
            size: 128,
            phase: 0.,
        }
    }
}
impl Icon {
    pub fn validate(&self) -> io::Result<()> {
        if !(16..=2048).contains(&self.size) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "icon size must be 16..2048",
            ));
        }
        if !self.phase.is_finite() || self.phase.abs() > TAU {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "phase must be finite and between -tau and tau",
            ));
        }
        Ok(())
    }
    /// Sample a closed Fourier contour. The closing point is not duplicated.
    /// Geometry can be passed directly to an application without SVG or files.
    pub fn points(&self, samples: usize) -> io::Result<Vec<Point>> {
        self.validate()?;
        if !(32..=4096).contains(&samples) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "contour samples must be 32..4096",
            ));
        }
        let center = self.size as f64 / 2.;
        let scale = self.size as f64 * 0.32;
        let (rotation_sine, rotation_cosine) = self.phase.sin_cos();
        Ok((0..samples)
            .map(|i| {
                let t = TAU * i as f64 / samples as f64;
                // Signed high-frequency terms change shape, not just path density.
                let (frequency, amplitude) = match self.shape {
                    Shape::Circle => (1., 0.),
                    Shape::Seed => (3., 0.2),
                    Shape::Spark => (5., 0.28),
                };
                let x = t.cos() + amplitude * (frequency * t).cos();
                let y = t.sin() - amplitude * (frequency * t).sin();
                Point {
                    x: center + scale * (rotation_cosine * x - rotation_sine * y),
                    y: center + scale * (rotation_sine * x + rotation_cosine * y),
                }
            })
            .collect())
    }
    /// Standalone SVG using the same public sampled geometry. Numeric paths and
    /// fixed colors only: no external references, fonts or injected markup.
    pub fn svg(&self) -> io::Result<String> {
        use std::fmt::Write;
        let points = self.points(128)?;
        let mut svg = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{0}\" height=\"{0}\" viewBox=\"0 0 {0} {0}\">\n<path d=\"",
            self.size
        );
        for (i, p) in points.iter().enumerate() {
            write!(
                svg,
                "{} {:.6} {:.6} ",
                if i == 0 { "M" } else { "L" },
                p.x,
                p.y
            )
            .unwrap();
        }
        svg.push_str("Z\" fill=\"#237f61\" stroke=\"#123d32\" stroke-width=\"2\" stroke-linejoin=\"round\"/>\n</svg>\n");
        Ok(svg)
    }
}
