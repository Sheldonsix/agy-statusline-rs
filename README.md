# agy-statusline-rs

English | [中文](README_zh.md)

[![GitHub Release](https://img.shields.io/github/v/release/Sheldonsix/agy-statusline-rs?style=flat-square)](https://github.com/Sheldonsix/agy-statusline-rs/releases)
[![License](https://img.shields.io/github/license/Sheldonsix/agy-statusline-rs?style=flat-square)](https://github.com/Sheldonsix/agy-statusline-rs/blob/main/LICENSE)

A fast, cross-platform Rust statusline for Antigravity CLI. It reads JSON from `stdin` and prints a compact, terminal-friendly line with your directory, Git branch, model, token usage, and quota.

![agy-statusline-rs screenshot](image.png)

## Installation

**Quick Install (macOS / Linux / WSL):**
```bash
curl -fsSL https://github.com/Sheldonsix/agy-statusline-rs/releases/latest/download/installer.sh | sh
```

**Quick Install (Windows PowerShell):**
```powershell
irm https://github.com/Sheldonsix/agy-statusline-rs/releases/latest/download/installer.ps1 | iex
```
*Note: The quick installers will automatically set up the default configuration file for you.*

## Verification

The tool expects a JSON payload from `stdin`. You can manually test how it looks in your terminal:
```bash
printf '{"cwd":"/tmp","model":{"display_name":"Gemini"}}' | agy-statusline-rs
```

## Antigravity Setup

After installation, you need to configure Antigravity CLI to use this tool for rendering its statusline. There are two ways to do this:

**Option 1: Interactive Settings (Recommended)**
Open Antigravity CLI by typing `agy` in your terminal. Type `/settings` to open the configuration panel, find the `statusLine` setting, and set its `command` to `agy-statusline-rs` (`agy-statusline-rs.exe` on Windows).

**Option 2: Edit `settings.json`**
You can directly edit the Antigravity configuration file (typically located at `~/.gemini/antigravity-cli/settings.json` or `%USERPROFILE%\.gemini\antigravity-cli\settings.json` on Windows). Add or update the `statusLine` field:

```json
{
  "statusLine": {
    "command": "agy-statusline-rs",
    "enabled": true
  }
}
```
*Note: On Windows, use `"agy-statusline-rs.exe"`. If the executable is not in your system `PATH`, provide the absolute path here.*

## Configuration

The config file is optional. By default, modules are shown in this order: `dir -> branch -> model -> tokens -> quota`.

You can also use CLI flags (e.g., `--hide-branch`) for quick per-command overrides, or edit `config.json` in your standard OS config directory:
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
*Tip: Set `color: "never"` and `icons: "ascii"` for older terminals or incomplete fonts. You can explicitly set the config path via the `AGY_STATUSLINE_CONFIG` environment variable.*

## Contributing
To build from source: `cargo build --release`. Maintainers can publish new releases by pushing a Git tag (e.g. `v0.1.0`), which triggers the GitHub Actions release workflow.

## License
[MIT License](LICENSE)
