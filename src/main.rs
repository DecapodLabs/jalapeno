use jalapeno::{
    audio::Tone,
    visual::{Icon, Shape},
};
use std::{
    collections::HashSet,
    env,
    ffi::OsString,
    fs::{self, OpenOptions},
    io::{self, BufWriter, Write},
    path::{Path, PathBuf},
    process::ExitCode,
};

const HELP: &str = "Jalapeño — procedural tones and harmonic icons\n\n\
Usage:\n  jalapeno tone -o sound.wav [--hz 440] [--seconds 0.25] [--rate 48000]\n\
                [--gain 0.6] [--partials 1,0.3,0.1]\n\
  jalapeno icon -o icon.svg [--shape circle|seed|spark] [--size 128] [--phase 0]\n\n\
Tone output is mono PCM16 WAV; icon output is a closed harmonic SVG contour.\n\
Phase is rotation in radians. Existing output files are preserved.\n\
Use --help (-h) or --version (-V). Exit codes: 0 success, 2 input, 3 output I/O.";

fn main() -> ExitCode {
    match run(env::args_os().skip(1).collect()) {
        Ok(()) => ExitCode::SUCCESS,
        Err((code, message)) => {
            eprintln!("jalapeno: {message}");
            ExitCode::from(code)
        }
    }
}
fn usage(message: &str) -> (u8, String) {
    (2, format!("{message}; use --help"))
}
fn number<T: std::str::FromStr>(value: &OsString) -> Result<T, (u8, String)> {
    value
        .to_str()
        .and_then(|v| v.parse().ok())
        .ok_or_else(|| usage("invalid numeric option"))
}
fn run(args: Vec<OsString>) -> Result<(), (u8, String)> {
    match args.as_slice() {
        [] => {
            println!("{HELP}");
            return Ok(());
        }
        [arg] if arg == "--help" || arg == "-h" => {
            println!("{HELP}");
            return Ok(());
        }
        [arg] if arg == "--version" || arg == "-V" => {
            println!("jalapeno {}", env!("CARGO_PKG_VERSION"));
            return Ok(());
        }
        _ => {}
    }
    let command = args[0].to_str().ok_or_else(|| usage("invalid command"))?;
    if !["tone", "icon"].contains(&command) {
        return Err(usage("unknown command"));
    }
    let mut tone = Tone::default();
    let mut icon = Icon::default();
    let mut output = None;
    let mut seen = HashSet::new();
    let mut options = args[1..].iter();
    while let Some(key) = options.next() {
        let key = key.to_str().ok_or_else(|| usage("invalid option"))?;
        let key = if key == "--output" { "-o" } else { key };
        if !seen.insert(key) {
            return Err(usage("duplicate option"));
        }
        let value = options
            .next()
            .ok_or_else(|| usage("missing option value"))?;
        match (command, key) {
            (_, "-o") => output = Some(PathBuf::from(value)),
            ("tone", "--hz") => tone.hz = number(value)?,
            ("tone", "--seconds") => tone.seconds = number(value)?,
            ("tone", "--rate") => tone.sample_rate = number(value)?,
            ("tone", "--gain") => tone.gain = number(value)?,
            ("tone", "--partials") => {
                let values = value.to_str().ok_or_else(|| usage("invalid partials"))?;
                tone.harmonics = values
                    .split(',')
                    .map(|v| v.parse().map_err(|_| usage("invalid partial amplitude")))
                    .collect::<Result<_, _>>()?;
            }
            ("icon", "--size") => icon.size = number(value)?,
            ("icon", "--phase") => icon.phase = number(value)?,
            ("icon", "--shape") => {
                icon.shape = match value.to_str() {
                    Some("circle") => Shape::Circle,
                    Some("seed") => Shape::Seed,
                    Some("spark") => Shape::Spark,
                    _ => return Err(usage("shape must be circle, seed or spark")),
                }
            }
            _ => return Err(usage("unknown option for this command")),
        }
    }
    let output = output.ok_or_else(|| usage("supply -o OUTPUT"))?;
    let result = if command == "tone" {
        tone.validate().map_err(|e| usage(&e.to_string()))?;
        write_output(&output, |writer| tone.write_wav(writer))
    } else {
        let svg = icon.svg().map_err(|e| usage(&e.to_string()))?;
        write_output(&output, |writer| writer.write_all(svg.as_bytes()))
    };
    result.map_err(|e| (3, e.to_string()))?;
    println!("{}", output.display());
    Ok(())
}
/// Install a fully written sibling file without replacing an existing output.
fn write_output(
    path: &Path,
    render: impl FnOnce(&mut BufWriter<fs::File>) -> io::Result<()>,
) -> io::Result<()> {
    let temp = path.with_extension(format!("jalapeno-{}.tmp", std::process::id()));
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temp)?;
    let result = (|| {
        let mut writer = BufWriter::new(file);
        render(&mut writer)?;
        writer.flush()?;
        writer.get_ref().sync_all()?;
        drop(writer);
        fs::hard_link(&temp, path)
    })();
    let cleanup = fs::remove_file(&temp);
    result.and(cleanup)
}
