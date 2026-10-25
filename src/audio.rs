//! Additive sine synthesis and bounded-memory PCM WAV export.
use std::{
    f64::consts::TAU,
    io::{self, Write},
};

/// Mono additive tone. Durations include a short raised-cosine fade at both ends.
#[derive(Clone, Debug)]
pub struct Tone {
    pub hz: f64,
    pub seconds: f64,
    pub sample_rate: u32,
    pub gain: f64,
    /// Signed amplitudes for the fundamental and successive integer harmonics.
    /// Absolute amplitudes are normalized so output cannot exceed `gain`.
    pub harmonics: Vec<f64>,
}
impl Default for Tone {
    fn default() -> Self {
        Self {
            hz: 440.,
            seconds: 0.25,
            sample_rate: 48000,
            gain: 0.6,
            harmonics: vec![1.],
        }
    }
}
fn invalid(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message)
}
impl Tone {
    /// Validate all parameters before writing anything. At most ten minutes,
    /// 32 harmonics and 192 kHz. Returns the output mono sample count.
    pub fn validate(&self) -> io::Result<u32> {
        if !(8000..=192000).contains(&self.sample_rate) {
            return Err(invalid("sample rate must be 8000..192000"));
        }
        if !self.hz.is_finite() || self.hz < 1. || self.hz >= self.sample_rate as f64 / 2. {
            return Err(invalid(
                "frequency must be finite, >= 1 Hz and below Nyquist",
            ));
        }
        if !self.seconds.is_finite() || !(0.01..=600.).contains(&self.seconds) {
            return Err(invalid("duration must be finite and 0.01..600 seconds"));
        }
        if !self.gain.is_finite() || !(0.0..=1.).contains(&self.gain) {
            return Err(invalid("gain must be finite and 0..1"));
        }
        if self.harmonics.is_empty()
            || self.harmonics.len() > 32
            || self
                .harmonics
                .iter()
                .any(|v| !v.is_finite() || v.abs() > 1.)
        {
            return Err(invalid("need 1..32 finite harmonic amplitudes in -1..1"));
        }
        let frames = (self.seconds * self.sample_rate as f64).ceil() as u32;
        if frames as u64 * self.harmonics.len() as u64 > 200_000_000 {
            return Err(invalid(
                "tone workload exceeds 200 million sample-partial pairs",
            ));
        }
        Ok(frames)
    }
    /// Stream mono PCM16 RIFF/WAVE. Partials at or above Nyquist are omitted;
    /// omitted partials retain their normalization weight for stable headroom.
    /// Scratch storage is constant regardless of duration. I/O errors propagate.
    pub fn write_wav(&self, mut output: impl Write) -> io::Result<()> {
        let frames = self.validate()?;
        let bytes = frames * 2;
        output.write_all(b"RIFF")?;
        output.write_all(&(bytes + 36).to_le_bytes())?;
        output.write_all(b"WAVEfmt ")?;
        output.write_all(&16u32.to_le_bytes())?;
        output.write_all(&1u16.to_le_bytes())?;
        output.write_all(&1u16.to_le_bytes())?;
        output.write_all(&self.sample_rate.to_le_bytes())?;
        output.write_all(&(self.sample_rate * 2).to_le_bytes())?;
        output.write_all(&2u16.to_le_bytes())?;
        output.write_all(&16u16.to_le_bytes())?;
        output.write_all(b"data")?;
        output.write_all(&bytes.to_le_bytes())?;
        let normalization = self.harmonics.iter().map(|v| v.abs()).sum::<f64>().max(1.);
        let fade = (self.sample_rate as f64 * 0.005).min((frames - 1) as f64 / 2.);
        let mut block = [0u8; 2048];
        let mut used = 0;
        for frame in 0..frames {
            let t = frame as f64 / self.sample_rate as f64;
            let value = self
                .harmonics
                .iter()
                .enumerate()
                .filter_map(|(i, &amplitude)| {
                    let hz = self.hz * (i + 1) as f64;
                    (hz < self.sample_rate as f64 / 2.).then(|| amplitude * (TAU * hz * t).sin())
                })
                .sum::<f64>();
            let edge = frame.min(frames - 1 - frame) as f64;
            let envelope = if edge < fade {
                (1. - (std::f64::consts::PI * edge / fade).cos()) * 0.5
            } else {
                1.
            };
            let pcm =
                (value / normalization * self.gain * envelope * i16::MAX as f64).round() as i16;
            block[used..used + 2].copy_from_slice(&pcm.to_le_bytes());
            used += 2;
            if used == block.len() {
                output.write_all(&block)?;
                used = 0;
            }
        }
        output.write_all(&block[..used])
    }
}
