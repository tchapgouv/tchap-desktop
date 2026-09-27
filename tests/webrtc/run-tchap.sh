#!/usr/bin/env bash
# Launch the macOS test build of Tchap with the WebRTC shim forced over WKWebView.
# Build first:  (cd ../../src-tauri && cargo build)
# Devtools: Cmd+Opt+I in the window (debug build). Unset TCHAP_WEBRTC_FORCE_SHIM
# to compare against WKWebView's native WebRTC.
set -euo pipefail
BIN="$(cd "$(dirname "$0")" && pwd)/../../src-tauri/target/debug/tchap-desktop"
export TCHAP_WEBRTC_FORCE_SHIM="${TCHAP_WEBRTC_FORCE_SHIM-1}"
export RUST_LOG="${RUST_LOG:-info,tauri_plugin_webrtc=debug,qrtc=info}"
exec "$BIN" "$@"
