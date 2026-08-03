# ltx_utils

**Low-level filesystem helpers.** Small, dependency-free utilities shared by
the other crates for common I/O operations.

## Functions

| Function | Description |
|----------|-------------|
| `create_dir(path)` | Creates a directory, including all missing parents. |
| `create_file(path)` | Creates a file, including its parent directories. |
| `write_file(path, contents)` | Writes contents to a file, creating parent dirs as needed. |
| `resolve_main_file(path)` | Returns the main entry path, defaulting to `main.tex`; errors with `NotFound` if the given path doesn't exist. |

## Usage

```rust
use ltx_utils::{create_dir, write_file};

create_dir(Path::new("my-paper/src/sections"))?;
write_file(Path::new("my-paper/main.tex"), r"\documentclass{article}")?;
```

## Design notes

- No dependencies on other `ltx_*` crates — usable from anywhere in the workspace.
- All functions return `std::io::Result`; no panicking variants.
