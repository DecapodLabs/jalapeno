#!/usr/bin/env python3
"""Check the initial CLI; this does not validate future synthesis features."""
import os
from pathlib import Path
import subprocess

cargo = os.environ.get('JALAPENO_CARGO', 'cargo')
for args in [('fmt', '--all', '--', '--check'),
             ('clippy', '--all-targets', '--', '-D', 'warnings'),
             ('test',), ('build',)]:
    subprocess.run([cargo, *args], check=True)

binary = Path(os.environ.get('CARGO_TARGET_DIR', 'target')).resolve() / 'debug' / ('jalapeno.exe' if os.name == 'nt' else 'jalapeno')
def run(*args):
    return subprocess.run([str(binary), *args], capture_output=True, text=True, check=False)

help_result = run('--help')
assert help_result.returncode == 0 and not help_result.stderr
assert 'does not render assets yet' in help_result.stdout
assert run().stdout == run('-h').stdout == help_result.stdout
version = run('--version')
assert version.returncode == 0 and version.stdout == 'jalapeno 0.1.0\n'
assert run('-V').stdout == version.stdout
for args in [('render',), ('--help', 'extra'), ('--version', 'extra')]:
    invalid = run(*args)
    assert invalid.returncode == 2 and not invalid.stdout and 'use --help' in invalid.stderr
print('PASS: formatting, clippy, compilation and initial CLI smoke checks')
