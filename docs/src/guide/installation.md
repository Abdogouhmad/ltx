# Installation

## Prerequisites

- **Rust 1.86+** — install via [rustup](https://rustup.rs/)

That's it. LTX compiles with **Tectonic**, a self-contained, native-Rust TeX
engine — no separate TeX distribution is required. The `ltx build` default
uses `tectonic` and downloads its TeX Live bundle on first use.

> **Optional:** if you'd rather use `pdflatex`, `xelatex`, or `lualatex`
> (or need a package not yet supported by Tectonic), install a traditional
> distribution and make sure its binaries are on your `PATH`:
>
> - [TeX Live](https://tug.org/texlive/) (Linux, macOS, Windows)
> - [MiKTeX](https://miktex.org/) (Windows, macOS, Linux)
> - [MacTeX](https://tug.org/mactex/) (macOS)

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
ltx code --lexer   # list the lexer diagnostic codes
```

You should see the CLI help text, the installed version, and the code table.

## Updating

Installations from a GitHub release can update themselves in place:

```bash
ltx update          # confirm and install the latest release
ltx update --check  # just report whether a newer release exists
```

See [CLI Usage](cli.md) for details. Set `LTX_NO_UPDATE_CHECK=1` to silence
the background update hint printed by other commands.

## Platform notes

### Linux

No special steps. If you use a traditional TeX distribution alongside
Tectonic, ensure its binaries are on your `PATH` (TeX Live installs to
`/usr/local/texlive/…` by default).

### macOS

If you installed MacTeX, the binaries are at `/Library/TeX/texbin/`. Add this
to your `PATH` if it isn't already:

```bash
export PATH="/Library/TeX/texbin:$PATH"
```

### Windows

MiKTeX or TeX Live should register themselves on the system `PATH` during
installation.

## Next steps

- [CLI Usage](cli.md) — learn the available commands
- [Configuration](configuration.md) — set up `ltx.toml` for your project
