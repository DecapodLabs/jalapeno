use jalapeno::{
    audio::Tone,
    visual::{Icon, Shape},
};
use std::{f64::consts::TAU, io};
fn pcm(tone: &Tone) -> Vec<i16> {
    let mut wav = Vec::new();
    tone.write_wav(&mut wav).unwrap();
    assert_eq!(&wav[..4], b"RIFF");
    assert_eq!(&wav[8..16], b"WAVEfmt ");
    assert_eq!(u16::from_le_bytes(wav[22..24].try_into().unwrap()), 1);
    assert_eq!(
        u32::from_le_bytes(wav[24..28].try_into().unwrap()),
        tone.sample_rate
    );
    assert_eq!(
        u32::from_le_bytes(wav[40..44].try_into().unwrap()) as usize,
        wav.len() - 44
    );
    wav[44..]
        .chunks_exact(2)
        .map(|b| i16::from_le_bytes(b.try_into().unwrap()))
        .collect()
}
#[test]
fn tone_has_expected_pitch_headroom_and_fades() {
    let tone = Tone {
        seconds: 1.,
        ..Tone::default()
    };
    let samples = pcm(&tone);
    assert_eq!(samples.len(), 48000);
    assert_eq!(samples[0], 0);
    assert_eq!(*samples.last().unwrap(), 0);
    assert!(
        samples
            .iter()
            .all(|&v| (v as f64).abs() <= 32767. * 0.6 + 0.5)
    );
    let crossings = samples[4800..43200]
        .windows(2)
        .filter(|w| w[0] <= 0 && w[1] > 0)
        .count();
    assert!((crossings as i32 - 352).abs() <= 1);
    assert!(
        samples
            .windows(2)
            .all(|w| (w[0] as i32 - w[1] as i32).abs() < 1200)
    );
}
#[test]
fn additive_partials_are_real_and_nyquist_terms_are_omitted() {
    let fundamental = Tone {
        gain: 0.8,
        seconds: 0.1,
        ..Tone::default()
    };
    let harmonic = Tone {
        harmonics: vec![0., 1.],
        ..fundamental.clone()
    };
    let a = pcm(&fundamental);
    let b = pcm(&harmonic);
    assert_ne!(a, b);
    let projection = |samples: &[i16], hz: f64| {
        samples
            .iter()
            .enumerate()
            .map(|(i, &v)| v as f64 * (TAU * hz * i as f64 / 48000.).sin())
            .sum::<f64>()
            .abs()
    };
    assert!(projection(&b, 880.) > projection(&b, 440.) * 100.);
    let high = Tone {
        hz: 13000.,
        harmonics: vec![0., 1.],
        ..fundamental
    };
    assert!(pcm(&high).iter().all(|&v| v == 0));
}
#[test]
fn silence_determinism_and_parameter_limits() {
    assert!(
        pcm(&Tone {
            gain: 0.,
            ..Tone::default()
        })
        .iter()
        .all(|&v| v == 0)
    );
    assert_eq!(pcm(&Tone::default()), pcm(&Tone::default()));
    for hz in [f64::NAN, f64::INFINITY, 0., -1., 24000.] {
        let mut output = Vec::new();
        assert!(
            Tone {
                hz,
                ..Tone::default()
            }
            .write_wav(&mut output)
            .is_err()
        );
        assert!(output.is_empty());
    }
    for seconds in [f64::NAN, 0., 601.] {
        assert!(
            Tone {
                seconds,
                ..Tone::default()
            }
            .validate()
            .is_err()
        );
    }
    assert!(
        Tone {
            harmonics: vec![f64::NAN],
            ..Tone::default()
        }
        .validate()
        .is_err()
    );
    assert!(
        Tone {
            harmonics: vec![1.; 33],
            ..Tone::default()
        }
        .validate()
        .is_err()
    );
}
#[test]
fn expensive_tones_and_bad_rates_or_gains_are_rejected() {
    assert!(
        Tone {
            sample_rate: 192000,
            seconds: 600.,
            harmonics: vec![1.; 32],
            ..Tone::default()
        }
        .validate()
        .is_err()
    );
    assert!(
        Tone {
            sample_rate: 7999,
            ..Tone::default()
        }
        .validate()
        .is_err()
    );
    assert!(
        Tone {
            gain: f64::NAN,
            ..Tone::default()
        }
        .validate()
        .is_err()
    );
    assert!(
        Tone {
            gain: 1.01,
            ..Tone::default()
        }
        .validate()
        .is_err()
    );
}

#[test]
fn wav_is_streamed_in_bounded_writes_and_io_failures_propagate() {
    struct Counter {
        bytes: usize,
        largest: usize,
    }
    impl io::Write for Counter {
        fn write(&mut self, b: &[u8]) -> io::Result<usize> {
            self.bytes += b.len();
            self.largest = self.largest.max(b.len());
            Ok(b.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    let mut out = Counter {
        bytes: 0,
        largest: 0,
    };
    let tone = Tone {
        seconds: 3.,
        ..Tone::default()
    };
    tone.write_wav(&mut out).unwrap();
    assert_eq!(out.bytes, 44 + 144000 * 2);
    assert!(out.largest <= 2048);
    assert!(tone.write_wav([0u8; 12].as_mut_slice()).is_err());
}
#[test]
fn icons_are_bounded_closed_harmonic_contours_and_rotate() {
    let mut outputs = std::collections::BTreeSet::new();
    for shape in [Shape::Circle, Shape::Seed, Shape::Spark] {
        let icon = Icon {
            shape,
            ..Icon::default()
        };
        let points = icon.points(128).unwrap();
        for p in &points {
            assert!(p.x >= 2. && p.y >= 2. && p.x <= 126. && p.y <= 126.);
        }
        let first = points[0];
        let last = points[127];
        assert!((first.x - last.x).hypot(first.y - last.y) < 5.);
        assert_ne!(points, Icon { phase: 1., ..icon }.points(128).unwrap());
        let svg = icon.svg().unwrap();
        assert!(svg.contains("Z\""));
        outputs.insert(svg);
    }
    assert_eq!(outputs.len(), 3);
    assert!(
        Icon {
            size: 0,
            ..Icon::default()
        }
        .svg()
        .is_err()
    );
    assert!(
        Icon {
            phase: f64::NAN,
            ..Icon::default()
        }
        .svg()
        .is_err()
    );
    assert!(Icon::default().points(100000).is_err());
}
