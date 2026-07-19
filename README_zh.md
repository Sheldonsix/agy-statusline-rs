# agy-statusline-rs

[English](README.md) | 中文

[![GitHub Release](https://img.shields.io/github/v/release/Sheldonsix/agy-statusline-rs?style=flat-square)](https://github.com/Sheldonsix/agy-statusline-rs/releases)
[![License](https://img.shields.io/github/license/Sheldonsix/agy-statusline-rs?style=flat-square)](https://github.com/Sheldonsix/agy-statusline-rs/blob/main/LICENSE)

一个为 Antigravity CLI 编写的轻量级 Rust 状态栏工具。它通过 `stdin` 接收 JSON 数据，并在终端输出包含当前目录、Git 分支、大模型、Token 和额度等信息的紧凑状态栏。

![终端运行效果截图](image.png)

## 安装

**一键安装 (macOS / Linux / WSL):**
```bash
curl -fsSL https://github.com/Sheldonsix/agy-statusline-rs/releases/latest/download/installer.sh | sh
```

**一键安装 (Windows PowerShell):**
```powershell
irm https://github.com/Sheldonsix/agy-statusline-rs/releases/latest/download/installer.ps1 | iex
```
*注：一键安装脚本会自动为你生成默认配置文件。*

## 验证

该工具通过标准输入 (`stdin`) 接收 JSON 数据进行渲染。你可以通过以下命令手动测试终端输出效果：
```bash
printf '{"cwd":"/tmp","model":{"display_name":"Gemini"}}' | agy-statusline-rs
```

## 接入 Antigravity CLI

安装完成后，你需要让 Antigravity CLI 知道去调用这个工具来渲染状态栏。主要有两种配置方式：

**方式一：通过命令行界面设置（推荐）**
在终端中进入 Antigravity CLI (`agy`)，输入 `/settings` 打开配置面板，找到 `statusLine` 配置项，将其中的 `command` 设置为刚刚安装的 `agy-statusline-rs`（Windows 下为 `agy-statusline-rs.exe`）。

**方式二：手动修改配置文件**
你也可以直接修改 Antigravity 的配置文件（通常位于 `~/.gemini/antigravity-cli/settings.json`），在其中加入或修改以下字段：

```json
{
  "statusLine": {
    "command": "agy-statusline-rs"
  }
}
```
*注：如果二进制文件没有被加入环境变量 `PATH`，这里请填写它的绝对路径。*

## 配置

配置文件是可选的。默认的模块显示顺序为：`dir -> branch -> model -> tokens -> quota`。

你可以通过命令行参数（如 `--hide-branch`）进行快速隐藏，也可以在系统的标准配置目录下修改 `config.json`：
- **Linux/WSL**: `~/.config/agy-statusline/config.json`
- **macOS**: `~/Library/Application Support/agy-statusline/config.json`
- **Windows**: `%APPDATA%\agy-statusline\config.json`

**配置示例 `config.json`:**
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
*提示：如果你的终端较老或字体显示不全，可将 `color` 设为 `"never"`，`icons` 设为 `"ascii"`。可以通过 `AGY_STATUSLINE_CONFIG` 环境变量显式指定配置路径。*

## 参与贡献
源码编译：`cargo build --release`。维护者发布新版只需推送 tag（例如 `v0.1.0`），GitHub Actions 会自动编译并发布。

## 开源协议
[MIT License](LICENSE)
