#!/usr/bin/env sh
set -eu

APP_NAME="agy-statusline-rs"
REPO="Sheldonsix/agy-statusline-rs"
# Default install dir: ~/.local/bin
INSTALL_DIR="${AGY_STATUSLINE_INSTALL_DIR:-"$HOME/.local/bin"}"

say() {
  printf '%s\n' "$1"
}

fail() {
  printf 'error: %s\n' "$1" >&2
  exit 1
}

need_cmd() {
  command -v "$1" >/dev/null 2>&1 || fail "required command not found: $1"
}

detect_target() {
  os="$(uname -s)"
  arch="$(uname -m)"

  case "$arch" in
    x86_64 | amd64) cpu="x86_64" ;;
    arm64 | aarch64) cpu="aarch64" ;;
    *) fail "unsupported CPU architecture: $arch" ;;
  esac

  case "$os" in
    Darwin) platform="apple-darwin" ;;
    Linux) platform="unknown-linux-gnu" ;;
    *) fail "unsupported OS: $os" ;;
  esac

  printf '%s-%s' "$cpu" "$platform"
}

artifact_for_target() {
  target="$1"

  case "$target" in
    x86_64-apple-darwin)
      printf 'agy-statusline-rs-x86_64-apple-darwin.tar.gz'
      ;;
    aarch64-apple-darwin)
      printf 'agy-statusline-rs-aarch64-apple-darwin.tar.gz'
      ;;
    x86_64-unknown-linux-gnu)
      printf 'agy-statusline-rs-x86_64-unknown-linux-gnu.tar.gz'
      ;;
    aarch64-unknown-linux-gnu)
      printf 'agy-statusline-rs-aarch64-unknown-linux-gnu.tar.gz'
      ;;
    *)
      fail "unsupported target: $target"
      ;;
  esac
}

download() {
  url="$1"
  output="$2"

  if command -v curl >/dev/null 2>&1; then
    curl -fsSL "$url" -o "$output"
  elif command -v wget >/dev/null 2>&1; then
    wget -qO "$output" "$url"
  else
    fail "curl or wget is required"
  fi
}

config_path() {
  os="$(uname -s)"
  case "$os" in
    Darwin)
      printf '%s/Library/Application Support/agy-statusline/config.json' "$HOME"
      ;;
    *)
      if [ -n "${XDG_CONFIG_HOME:-}" ]; then
        printf '%s/agy-statusline/config.json' "$XDG_CONFIG_HOME"
      else
        printf '%s/.config/agy-statusline/config.json' "$HOME"
      fi
      ;;
  esac
}

write_default_config() {
  path="$(config_path)"
  dir="$(dirname "$path")"
  mkdir -p "$dir"

  if [ -f "$path" ]; then
    say "Config already exists: $path"
    return
  fi

  cat >"$path" <<'JSON'
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
JSON
  say "Created config: $path"
}

main() {
  need_cmd uname
  need_cmd tar
  need_cmd mkdir
  need_cmd dirname

  target="$(detect_target)"
  artifact="$(artifact_for_target "$target")"
  url="https://github.com/${REPO}/releases/latest/download/${artifact}"
  tmp_dir="$(mktemp -d 2>/dev/null || mktemp -d -t agy-statusline)"
  archive="${tmp_dir}/${artifact}"

  trap 'rm -rf "$tmp_dir"' EXIT INT TERM

  say "Downloading $url"
  download "$url" "$archive"

  mkdir -p "$tmp_dir/unpacked" "$INSTALL_DIR"
  tar -xzf "$archive" -C "$tmp_dir/unpacked"
  cp "$tmp_dir/unpacked/${APP_NAME}" "$INSTALL_DIR/${APP_NAME}"
  chmod +x "$INSTALL_DIR/${APP_NAME}"

  write_default_config

  say "Installed: $INSTALL_DIR/$APP_NAME"
  case ":$PATH:" in
    *":$INSTALL_DIR:"*) ;;
    *)
      say "Note: $INSTALL_DIR is not in PATH. Add it to PATH or use the full command path below."
      ;;
  esac
  say "Antigravity statusline command:"
  say "  $INSTALL_DIR/$APP_NAME"
}

main "$@"
