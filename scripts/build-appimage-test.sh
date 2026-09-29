#!/usr/bin/env bash
# Run inside Ubuntu 24.04 with native libraries matching the container architecture.
set -euo pipefail

repo_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_dir"

if [[ "$(uname -s)" != Linux ]]; then
  echo "Run this script inside Ubuntu 24.04." >&2
  exit 1
fi
case "$(uname -m)" in
  aarch64) arch=arm64; triple=aarch64-linux-gnu; target=aarch64-unknown-linux-gnu ;;
  x86_64) arch=x64; triple=x86_64-linux-gnu; target=x86_64-unknown-linux-gnu ;;
  *) echo "Unsupported architecture: $(uname -m)" >&2; exit 1 ;;
esac
. /etc/os-release
if [[ "$ID" != ubuntu || "$VERSION_ID" != 24.04 ]]; then
  echo "This test configuration targets Ubuntu 24.04." >&2
  exit 1
fi

for required_command in cargo patchelf strace xvfb-run xauth; do
  if ! command -v "$required_command" >/dev/null 2>&1; then
    echo "Missing build command: $required_command (see docs/linux-cef.md)." >&2
    exit 1
  fi
done

if [[ "$(cargo tauri --version)" != 'tauri-cli 3.0.0-alpha.2' ]]; then
  echo "Install the matching CLI: cargo install tauri-cli --version 3.0.0-alpha.2 --locked" >&2
  exit 1
fi

for required_file in src/index.html src/config.json \
  "/usr/lib/$triple/libatk-1.0.so.0" \
  "/usr/lib/$triple/libatk-bridge-2.0.so.0" \
  "/usr/lib/$triple/libatspi.so.0" \
  "/usr/lib/$triple/libsoftokn3.so" \
  "/usr/lib/$triple/libfreeblpriv3.so" /usr/bin/zenity; do
  if [[ ! -f "$required_file" ]]; then
    echo "Missing build input: $required_file (see docs/linux-cef.md)." >&2
    exit 1
  fi
done

export CARGO_TARGET_DIR="$repo_dir/src-tauri/target-linux-$arch-ubuntu-24.04"
export APPIMAGE_EXTRACT_AND_RUN=1
# The quick-sharun default URL returns 404. Use the source from the same fork
# Tauri uses for quick-sharun, pinned to a known revision.
export ANYLINUX_LIB_SOURCE='https://raw.githubusercontent.com/FabianLars/Anylinux-AppImages/df9cd3246ccbcf61eb22fc321a52277351047b4b/useful-tools/lib/anylinux.c'

cargo tauri build --ci \
  --target "$target" \
  --config src-tauri/tauri.conf.dev.json \
  --config "src-tauri/tauri.conf.appimage-$arch-test.json" \
  -- --locked

echo "AppImage output: $CARGO_TARGET_DIR/$target/release/bundle/appimage/"
