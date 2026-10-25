#!/usr/bin/env python3
"""Build/lint/test the core and decode real tone/icon CLI exports."""
import math
import os
from pathlib import Path
import re
import struct
import subprocess
import tempfile
import wave
import xml.etree.ElementTree as ET

cargo = os.environ.get('JALAPENO_CARGO', 'cargo')
with tempfile.TemporaryDirectory(prefix='jalapeno-check-') as scratch:
    scratch = Path(scratch)
    environment = os.environ.copy()
    # Decapod executes against an immutable source snapshot. All artifacts belong
    # to fresh scratch, including Cargo output and exported examples.
    environment['CARGO_TARGET_DIR'] = str(scratch / 'target')
    package = subprocess.run([cargo, 'package', '--list', '--locked'], env=environment,
                             check=True, capture_output=True, text=True, timeout=60)
    packaged_files = [path.replace('\\', '/') for path in package.stdout.splitlines()]
    assert {'Cargo.toml', 'LICENSE', 'README.md'} <= set(packaged_files)
    assert any(path.startswith('src/') for path in packaged_files)
    assert any(path.startswith('tests/') for path in packaged_files)
    expected_roots = {'Cargo.lock', 'Cargo.toml', 'Cargo.toml.orig', 'LICENSE', 'README.md',
                      'src', 'tests'}
    assert all(path == '.cargo_vcs_info.json' or path.split('/', 1)[0] in expected_roots
               for path in packaged_files)
    assert not any(path.startswith(('.decapod/', '.github/', 'scripts/')) for path in packaged_files)
    subprocess.run([cargo, 'package', '--locked'], env=environment, check=True, timeout=120)
    offline = environment.get('CARGO_NET_OFFLINE', '').lower() == 'true'
    if not offline:
        subprocess.run([cargo, 'publish', '--dry-run', '--locked'], env=environment,
                       check=True, timeout=120)
    else:
        print('SKIP: cargo publish --dry-run requires registry access; package verification passed')
    for args in [('fmt', '--all', '--', '--check'),
                 ('clippy', '--workspace', '--all-targets', '--all-features', '--', '-D', 'warnings'),
                 ('test', '--workspace', '--all-features'), ('build', '--locked')]:
        subprocess.run([cargo, *args], env=environment, check=True, timeout=120)
    binary = scratch / 'target/debug' / ('jalapeno.exe' if os.name == 'nt' else 'jalapeno')
    def run(*args, code=0):
        result = subprocess.run([str(binary), *map(str, args)], env=environment, capture_output=True, text=True, timeout=30)
        assert result.returncode == code, (args, result.returncode, result.stderr)
        return result
    help_result = run('--help')
    assert not help_result.stderr and 'tone' in help_result.stdout and 'icon' in help_result.stdout
    assert run().stdout == run('-h').stdout == help_result.stdout
    version = re.search(r'^version = "([^"]+)"', Path('Cargo.toml').read_text(), re.M).group(1)
    assert run('--version').stdout == run('-V').stdout == 'jalapeno ' + version + '\n'
    tone = scratch / 'tone.wav'
    run('tone', '--hz', '440', '--seconds', '0.1', '--partials', '1,0.3', '-o', tone)
    original = tone.read_bytes()
    with wave.open(str(tone), 'rb') as audio:
        assert (audio.getnchannels(), audio.getsampwidth(), audio.getframerate(), audio.getnframes()) == (1, 2, 48000, 4800)
        pcm = struct.unpack('<4800h', audio.readframes(4800))
        rms = math.sqrt(sum(v*v for v in pcm) / len(pcm)) / 32768
        assert 0.1 < rms < 0.5 and pcm[0] == pcm[-1] == 0 and max(map(abs, pcm)) < 32767
    run('tone', '-o', tone, code=3)
    assert tone.read_bytes() == original
    repeated = scratch / 'repeat.wav'
    run('tone', '--hz', '440', '--seconds', '0.1', '--partials', '1,0.3', '-o', repeated)
    assert repeated.read_bytes() == original
    for shape in ('circle', 'seed', 'spark'):
        icon = scratch / (shape + '.svg')
        run('icon', '--shape', shape, '--size', '128', '--phase', '0.3', '-o', icon)
        svg = ET.parse(icon).getroot()
        assert svg.attrib['viewBox'] == '0 0 128 128'
        path = svg.find('{http://www.w3.org/2000/svg}path')
        assert path is not None and path.attrib['d'].endswith('Z')
        coordinates = list(map(float, re.findall(r'-?\d+\.\d+', path.attrib['d'])))
        assert len(coordinates) == 256 and all(1 < v < 127 for v in coordinates)
    for args in [('unknown',), ('--help', 'extra'), ('--version', 'extra'),
                 ('tone',), ('tone', '--hz', 'NaN', '-o', scratch / 'bad.wav'),
                 ('icon', '--size', '999999', '-o', scratch / 'bad.svg'),
                 ('icon', '--shape', 'unknown', '-o', scratch / 'bad.svg'),
                 ('tone', '--hz', '100', '--hz', '200', '-o', scratch / 'bad.wav')]:
        result = run(*args, code=2)
        assert not result.stdout and 'use --help' in result.stderr
    assert not list(scratch.glob('*.tmp')) and not (scratch / 'bad.wav').exists() and not (scratch / 'bad.svg').exists()
print('PASS: package allowlist/build, fmt, clippy, tests, decoded WAV/SVG, deterministic output and CLI error/ownership checks')
