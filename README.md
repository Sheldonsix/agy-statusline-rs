# agy-statusline-rs

English | [中文](#中文)

A small Rust statusline for Antigravity CLI. It reads Antigravity CLI statusline JSON from `stdin` and prints a compact terminal-friendly statusline with directory, Git branch, model, token usage, and quota information.

## Features

- Cross-platform CLI support for macOS, Linux, Windows (including WSL)
- Configurable modules: `dir`, `branch`, `model`, `tokens`, `quota`
- Configurable display: color, icons, and layout
- Terminal-aware fallback for `NO_COLOR` and `TERM=dumb`
- Git branch display with quiet fallback outside Git repositories
- Existing `--hide-*` flags for quick per-command overrides

## Installation

### One-command Install

macOS / Linux / WSL:

```bash
curl -fsSL https://github.com/Sheldonsix/agy-statusline-rs/releases/latest/download/installer.sh | sh
agy-statusline-rs --help
```

Windows PowerShell:

```powershell
irm https://github.com/Sheldonsix/agy-statusline-rs/releases/latest/download/installer.ps1 | iex
agy-statusline-rs.exe --help
```

The installer downloads the matching prebuilt binary from the latest GitHub Release, installs it locally, creates a default config file when missing, and prints the command path to use in Antigravity.

### Developer Install With Cargo

Requires Rust and Cargo.

```bash
cargo install --git https://github.com/Sheldonsix/agy-statusline-rs
agy-statusline-rs --help
```

### Build From Source

```bash
git clone https://github.com/Sheldonsix/agy-statusline-rs.git
cd agy-statusline-rs
cargo build --release
```

Binary paths:

```text
macOS / Linux / WSL:
  ./target/release/agy-statusline-rs

Windows:
  .\target\release\agy-statusline-rs.exe
```

### Publishing a Release

Maintainers publish prebuilt binaries by pushing a version tag:

```bash
git tag v0.1.0
git push origin v0.1.0
```

The release workflow builds macOS, Linux, and Windows archives, uploads `installer.sh` and `installer.ps1`, and creates the latest GitHub Release used by the one-command installers.

## Configuration

The config file is optional. Without one, the default modules are shown in this order:

```text
dir -> branch -> model -> tokens -> quota
```

Default config locations:

```text
Linux / WSL:
  $XDG_CONFIG_HOME/agy-statusline/config.json
  or ~/.config/agy-statusline/config.json

macOS:
  ~/Library/Application Support/agy-statusline/config.json

Windows:
  %APPDATA%\agy-statusline\config.json
```

You can also set a config path explicitly:

```bash
agy-statusline-rs --config /path/to/config.json
```

Or use an environment variable:

```bash
export AGY_STATUSLINE_CONFIG="$HOME/.config/agy-statusline/config.json"
```

Windows PowerShell:

```powershell
$env:AGY_STATUSLINE_CONFIG = "$env:APPDATA\agy-statusline\config.json"
```

Copy the example config:

```bash
mkdir -p ~/.config/agy-statusline
cp examples/config.json ~/.config/agy-statusline/config.json
```

macOS:

```bash
mkdir -p "$HOME/Library/Application Support/agy-statusline"
cp examples/config.json "$HOME/Library/Application Support/agy-statusline/config.json"
```

Windows PowerShell:

```powershell
New-Item -ItemType Directory -Force "$env:APPDATA\agy-statusline"
Copy-Item .\examples\config.json "$env:APPDATA\agy-statusline\config.json"
```

Example config:

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

Display options:

```text
display.color:
  auto | always | never

display.icons:
  auto | emoji | ascii | none

display.layout:
  auto | single_line | two_lines
```

## Usage

This command expects statusline JSON from `stdin`. Antigravity provides that input when it runs the statusline command.

Common flags:

```bash
agy-statusline-rs --hide-branch
agy-statusline-rs --hide-model
agy-statusline-rs --hide-tokens
agy-statusline-rs --hide-quota
agy-statusline-rs --hide-dir
```

Manual test:

```bash
printf '%s' '{"cwd":"/tmp","terminal_width":120,"model":{"display_name":"Gemini"},"context_window":{"total_input_tokens":1200,"total_output_tokens":300,"used_percentage":12.5}}' | agy-statusline-rs
```

Windows PowerShell manual test:

```powershell
'{"cwd":"C:\\Temp","terminal_width":120,"model":{"display_name":"Gemini"},"context_window":{"total_input_tokens":1200,"total_output_tokens":300,"used_percentage":12.5}}' | agy-statusline-rs.exe
```

## Antigravity Setup

Set your Antigravity statusline command to the installed binary path.

Examples:

```text
macOS / Linux / WSL:
  agy-statusline-rs

Windows:
  agy-statusline-rs.exe
```

If the binary is not on `PATH`, use the full path.

```text
macOS / Linux / WSL:
  /home/you/.local/bin/agy-statusline-rs

Windows:
  C:\Users\you\AppData\Local\Programs\agy-statusline-rs\bin\agy-statusline-rs.exe
```

For WSL, keep the config inside the WSL filesystem unless Antigravity is explicitly invoking a Windows binary.

## Platform Notes

macOS and modern Linux terminals usually work well with:

```json
{
  "display": {
    "color": "auto",
    "icons": "auto",
    "layout": "auto"
  }
}
```

Windows Terminal usually supports color and emoji. For old terminals or incomplete fonts, use:

```json
{
  "display": {
    "color": "never",
    "icons": "ascii",
    "layout": "auto"
  }
}
```

WSL is treated like Linux. Its default config path is inside WSL, for example:

```text
/home/you/.config/agy-statusline/config.json
```

## 中文

`agy-statusline-rs` 是一个给 Antigravity CLI 使用的状态栏工具。它从 `stdin` 读取 Antigravity CLI 传入的 statusline JSON，然后输出一条适合终端显示的状态栏，包含目录、Git 分支、模型、Token 使用量和 quota 信息。

## 功能

- 支持 macOS、Linux、Windows（包括 WSL）
- 支持配置模块：`dir`、`branch`、`model`、`tokens`、`quota`
- 支持配置颜色、图标和布局
- 自动处理 `NO_COLOR` 和 `TERM=dumb` 等终端降级场景
- Git 分支展示在非 Git 目录下会静默隐藏
- 保留 `--hide-*` 参数，方便临时隐藏某些模块

## 安装

### 一键安装

macOS / Linux / WSL：

```bash
curl -fsSL https://github.com/Sheldonsix/agy-statusline-rs/releases/latest/download/installer.sh | sh
agy-statusline-rs --help
```

Windows PowerShell：

```powershell
irm https://github.com/Sheldonsix/agy-statusline-rs/releases/latest/download/installer.ps1 | iex
agy-statusline-rs.exe --help
```

安装脚本会从最新 GitHub Release 下载当前平台对应的预编译二进制，安装到本地目录，在配置文件不存在时创建默认配置，并输出 Antigravity 里可以直接使用的命令路径。

### 开发者通过 Cargo 安装

需要先安装 Rust 和 Cargo。

```bash
cargo install --git https://github.com/Sheldonsix/agy-statusline-rs
agy-statusline-rs --help
```

### 从源码构建

```bash
git clone https://github.com/Sheldonsix/agy-statusline-rs.git
cd agy-statusline-rs
cargo build --release
```

二进制位置：

```text
macOS / Linux / WSL:
  ./target/release/agy-statusline-rs

Windows:
  .\target\release\agy-statusline-rs.exe
```

### 发布新版本

维护者通过推送版本 tag 发布预编译二进制：

```bash
git tag v0.1.0
git push origin v0.1.0
```

release workflow 会构建 macOS、Linux、Windows 压缩包，上传 `installer.sh` 和 `installer.ps1`，并创建一键安装命令使用的 latest GitHub Release。

## 配置

配置文件是可选的。没有配置文件时，默认按下面的顺序显示：

```text
dir -> branch -> model -> tokens -> quota
```

默认配置路径：

```text
Linux / WSL:
  $XDG_CONFIG_HOME/agy-statusline/config.json
  或 ~/.config/agy-statusline/config.json

macOS:
  ~/Library/Application Support/agy-statusline/config.json

Windows:
  %APPDATA%\agy-statusline\config.json
```

也可以显式指定配置文件：

```bash
agy-statusline-rs --config /path/to/config.json
```

或者使用环境变量：

```bash
export AGY_STATUSLINE_CONFIG="$HOME/.config/agy-statusline/config.json"
```

Windows PowerShell：

```powershell
$env:AGY_STATUSLINE_CONFIG = "$env:APPDATA\agy-statusline\config.json"
```

复制示例配置：

```bash
mkdir -p ~/.config/agy-statusline
cp examples/config.json ~/.config/agy-statusline/config.json
```

macOS：

```bash
mkdir -p "$HOME/Library/Application Support/agy-statusline"
cp examples/config.json "$HOME/Library/Application Support/agy-statusline/config.json"
```

Windows PowerShell：

```powershell
New-Item -ItemType Directory -Force "$env:APPDATA\agy-statusline"
Copy-Item .\examples\config.json "$env:APPDATA\agy-statusline\config.json"
```

配置示例：

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

展示选项：

```text
display.color:
  auto | always | never

display.icons:
  auto | emoji | ascii | none

display.layout:
  auto | single_line | two_lines
```

## 使用

这个命令需要从 `stdin` 读取 statusline JSON。Antigravity 运行 statusline 命令时会自动传入这些数据。

常用参数：

```bash
agy-statusline-rs --hide-branch
agy-statusline-rs --hide-model
agy-statusline-rs --hide-tokens
agy-statusline-rs --hide-quota
agy-statusline-rs --hide-dir
```

手动测试：

```bash
printf '%s' '{"cwd":"/tmp","terminal_width":120,"model":{"display_name":"Gemini"},"context_window":{"total_input_tokens":1200,"total_output_tokens":300,"used_percentage":12.5}}' | agy-statusline-rs
```

Windows PowerShell 手动测试：

```powershell
'{"cwd":"C:\\Temp","terminal_width":120,"model":{"display_name":"Gemini"},"context_window":{"total_input_tokens":1200,"total_output_tokens":300,"used_percentage":12.5}}' | agy-statusline-rs.exe
```

## Antigravity CLI 配置

把 Antigravity CLI 的 statusline command 设置为安装后的二进制路径。

示例：

```text
macOS / Linux / WSL:
  agy-statusline-rs

Windows:
  agy-statusline-rs.exe
```

如果二进制不在 `PATH` 里，就使用完整路径。

```text
macOS / Linux / WSL:
  /home/you/.local/bin/agy-statusline-rs

Windows:
  C:\Users\you\AppData\Local\Programs\agy-statusline-rs\bin\agy-statusline-rs.exe
```

WSL 建议把配置文件放在 WSL 文件系统内，除非 Antigravity 调用的是 Windows 侧的二进制。

## 平台说明

macOS 和现代 Linux 终端通常可以使用：

```json
{
  "display": {
    "color": "auto",
    "icons": "auto",
    "layout": "auto"
  }
}
```

Windows Terminal 通常支持颜色和 emoji。老版本终端或字体不完整时，建议使用：

```json
{
  "display": {
    "color": "never",
    "icons": "ascii",
    "layout": "auto"
  }
}
```

WSL 会按 Linux 处理，默认配置路径在 WSL 内，例如：

```text
/home/you/.config/agy-statusline/config.json
```
