# agy-statusline-rs

English | [中文](README_zh.md)

[![GitHub Release](https://img.shields.io/github/v/release/Sheldonsix/agy-statusline-rs?style=flat-square)](https://github.com/Sheldonsix/agy-statusline-rs/releases)
[![Crates.io](https://img.shields.io/crates/v/agy-statusline-rs?style=flat-square)](https://crates.io/crates/agy-statusline-rs)
[![License](https://img.shields.io/github/license/Sheldonsix/agy-statusline-rs?style=flat-square)](https://github.com/Sheldonsix/agy-statusline-rs/blob/main/LICENSE)

A fast, cross-platform Rust statusline for Antigravity CLI. It reads JSON from `stdin` and prints a compact, terminal-friendly line with your directory, Git branch, model, token usage, and quota.

![agy-statusline-rs screenshot](image.png)

## Installation

**Quick Install (macOS / Linux / WSL):**
```bash
curl -fsSL https://github.com/Sheldonsix/agy-statusline-rs/releases/latest/download/installer.sh | sh
```

**Quick Install (Windows):**
```powershell
irm https://github.com/Sheldonsix/agy-statusline-rs/releases/latest/download/installer.ps1 | iex
```
*Note: The quick installers will automatically set up the default configuration file for you.*

**Cargo Install:**
```bash
cargo install agy-statusline-rs
```
*(Requires Rust 1.85.0+)*

## Antigravity Setup

Once installed, simply set your Antigravity statusline command to `agy-statusline-rs` (or `agy-statusline-rs.exe` on Windows). If it's not in your system `PATH`, provide the absolute path to the executable.

## Configuration

The config file is optional. By default, modules are shown in this order: `dir -> branch -> model -> tokens -> quota`.

You can find or create your `config.json` in your standard OS config directory:
- **Linux/WSL**: `~/.config/agy-statusline/config.json`
- **macOS**: `~/Library/Application Support/agy-statusline/config.json`
- **Windows**: `%APPDATA%\agy-statusline\config.json`

**Example `config.json`:**
```json
{
  "modules": {
    "dir": { "enabled": true, "order": 10 },
    "branch": { "enabled": true, "order": 20 },
    "model": { "enabled": true, "order": 30 },
    "tokens": { "enabled": true, "order": 40 },
    "quota": { "enabled": true, "order": 50 }
  },
  "display": {
    "color": "auto",
    "icons": "auto",
    "layout": "auto"
  }
}
```
*Tip: Set `color: "never"` and `icons: "ascii"` for older terminals or incomplete fonts.*

You can also override settings via CLI flags (e.g., `--hide-branch`, `--hide-model`) or by setting the `AGY_STATUSLINE_CONFIG` environment variable.

## Contributing
To build from source: `cargo build --release`. Maintainers can publish new releases by pushing a Git tag (e.g. `v0.1.0`), which triggers the GitHub Actions release workflow.

## License
[MIT License](LICENSE)
