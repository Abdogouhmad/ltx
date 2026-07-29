# Installation

## Prerequisites

- **Rust 1.86+** — install via [rustup](https://rustup.rs/)
- **A TeX distribution** — one of:
  - [TeX Live](https://tug.org/texlive/) (Linux, macOS, Windows)
  - [MiKTeX](https://miktex.org/) (Windows, macOS, Linux)
  - [MacTeX](https://tug.org/mactex/) (macOS)

The TeX distribution must provide `pdflatex` (or `xelatex`/`lualatex`) on your `PATH`.

## Install via Cargo

```bash
cargo install ltx
```

This downloads the latest published version from [crates.io](https://crates.io/crates/ltx) and compiles a release binary.

## Install from source

```bash
git clone https://github.com/Abdogouhmad/ltx.git
cd ltx
cargo install --path .
```

This builds and installs the current `main` branch.

## Verify installation

```bash
ltx --help
ltx --version
```

You should see the CLI help text and the installed version number.

## Platform notes

### Linux

No special steps. Ensure `pdflatex` is on your `PATH` (TeX Live installs to `/usr/local/texlive/…` by default).

### macOS

If you installed MacTeX, the binaries are at `/Library/TeX/texbin/`. Add this to your `PATH` if it isn't already:

```bash
export PATH="/Library/TeX/texbin:$PATH"
```

### Windows

MiKTeX or TeX Live should register themselves on the system `PATH` during installation. If `pdflatex` is not found, check your environment variables.

## Next steps

- [CLI Usage](cli.md) — learn the available commands
- [Configuration](configuration.md) — set up `config.toml` for your project
