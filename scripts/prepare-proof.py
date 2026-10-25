#!/usr/bin/env python3
"""Stage installed Rust tools at a reproducible location for local Decapod proof.

macOS ARM64 only for this initial readiness profile. No downloads, credentials,
wrappers or repository binaries are installed. Existing staged tools are verified
against the selected installed toolchain before reuse. Nothing is written to Git.
"""
import os
from pathlib import Path
import subprocess
import sys

PREFIX = Path('/private/tmp/jalapeno-proof-v1')

def main():
    if sys.platform != 'darwin' or os.uname().machine != 'arm64':
        sys.exit('The initial Decapod profile requires macOS ARM64; use scripts/check.py elsewhere.')
    cargo = Path(subprocess.check_output(['rustup', 'which', 'cargo'], text=True).strip()).resolve()
    source = cargo.parent.parent
    version = subprocess.check_output([str(source / 'bin/rustc'), '--version'], text=True).strip()
    if version != 'rustc 1.99.0 (b940084d7 2026-09-28)':
        sys.exit('This reviewed proof profile requires Rust 1.99.0 (b940084d7); review toolchain changes first.')
    destination = PREFIX / 'rust'
    if PREFIX.is_symlink() or (PREFIX.exists() and PREFIX.stat().st_uid != os.getuid()):
        sys.exit('Proof prefix must be owned by the current user and not a symlink.')
    PREFIX.mkdir(mode=0o700, exist_ok=True)
    # Keep the original sysroot layout so rustc resolves its libraries normally.
    # cp -c creates ordinary APFS copy-on-write files, with no symlink aliases.
    if not destination.exists():
        destination.mkdir()
        for name in ('bin', 'lib'):
            subprocess.run(['/bin/cp', '-cR', str(source / name), str(destination / name)], check=True)
    if destination.is_symlink():
        sys.exit('Staged Rust root must not be a symlink.')
    import hashlib
    def digest(path):
        with path.open('rb') as file:
            return hashlib.file_digest(file, 'sha256').digest() if hasattr(hashlib, 'file_digest') else hashlib.sha256(file.read()).digest()
    for name in ('bin', 'lib'):
        for original in (source / name).rglob('*'):
            if original.is_file():
                staged = destination / original.relative_to(source)
                if staged.is_symlink() or not staged.is_file() or digest(original) != digest(staged):
                    sys.exit('Staged tool differs from installed toolchain: ' + str(staged))
    print('Proof tools ready at', destination)
    print('No application build or validation has run.')

if __name__ == '__main__':
    main()
