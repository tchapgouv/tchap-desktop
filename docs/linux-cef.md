# Build a Linux AppImage with CEF

This builds a development AppImage with Tauri/CEF 3.0.0-alpha.2 on Ubuntu 24.04.
Podman and Node.js/npm must already be installed on macOS.

## Prepare the container

From the repository root, create an x64 Ubuntu 24.04 container (skip this if you already have one).
For ARM64, replace `linux/amd64` with `linux/arm64` and `-x64-` with `-arm64-` below.
Building x64 on Apple Silicon uses emulation and can be slow.

```sh
podman machine init --cpus 2 --memory 8192 --disk-size 32 tchap-build # if not already created
podman machine start tchap-build # if not already started
podman pull --platform linux/amd64 ubuntu:24.04
podman run -it --name tchap-appimage-build-x64-ubuntu-24.04 \
  --platform linux/amd64 \
  -v "$PWD:/workspace" -w /workspace \
  ubuntu:24.04 bash
```

For an existing container, use `podman start -ai tchap-appimage-build-x64-ubuntu-24.04`
instead of `podman run`.

Install the build dependencies once inside the container (`locales` supplies the
UTF-8 character maps needed by the bundler):

```sh
apt-get update
DEBIAN_FRONTEND=noninteractive apt-get install -y \
  build-essential ca-certificates clang curl file git patchelf pkg-config python3 \
  strace xvfb xauth locales \
  libasound2t64 libatk1.0-0t64 libatk-bridge2.0-0t64 libatspi2.0-0t64 \
  libclang-dev libdbus-1-dev libgbm-dev libgtk-4-dev \
  libnspr4 libnss3 libssl-dev libx11-dev libx11-xcb-dev libxkbcommon-dev \
  libxss1 libxtst6 zenity
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
  sh -s -- -y --profile minimal --default-toolchain 1.96.0
. "$HOME/.cargo/env"
cargo install tauri-cli --version 3.0.0-alpha.2 --locked
exit
```

## Build

On macOS, prepare the frontend and start the build:

```sh
npm ci
npm run fetch-package -- dev

podman start tchap-appimage-build-x64-ubuntu-24.04
podman exec tchap-appimage-build-x64-ubuntu-24.04 \
  bash -lc '. "$HOME/.cargo/env" && cd /workspace && bash scripts/build-appimage-test.sh'
```

The script picks the architecture and its `tauri.conf.appimage-*-test.json` config automatically.

The script pins DwarFS 0.13.0 to avoid a scanning loop on macOS shared mounts.
The bundler downloads it automatically; no separate DwarFS installation is needed.
For an existing container with another version cached, rename `/tmp/mkdwarfs`
once before building so the bundler downloads the pinned version.

Output: `src-tauri/target-linux-{arm64,x64}-ubuntu-24.04/<rust-target>/release/bundle/appimage/`.

## Try it on Linux

Run the AppImage in your Ubuntu desktop session. It uses CEF's default style.
To try Alloy, quit Tchap completely, then run:

```sh
TCHAP_CEF_STYLE=alloy ./your-build.AppImage
```

Alloy helps with typing, but the focus issue is not fully resolved.

### Optional X11 keyboard focus workaround

If keyboard input loses focus in the CEF webview, quit Tchap completely and
enable the workaround when launching the AppImage:

```sh
TCHAP_X11_FOCUS_WORKAROUND=1 ./your-build.AppImage
```

The workaround removes `WM_TAKE_FOCUS` from the main window's X11 `WM_PROTOCOLS`
property at startup. This is intended to prevent GTK from reclaiming keyboard
focus from the CEF child window. Other window protocols are preserved.
It applies to X11 windows, including those running through XWayland; native
Wayland windows are skipped.

This option is disabled by default and requires the exact value `1`. It is
independent of the CEF style and can also be combined with Alloy:

```sh
TCHAP_CEF_STYLE=alloy TCHAP_X11_FOCUS_WORKAROUND=1 ./your-build.AppImage
```

Look for the `[x11-focus]` message in the terminal to see whether the workaround
was applied, skipped, or failed. To disable it, quit Tchap completely and relaunch
with the variable unset or set to `0`. Validate typing, Tab navigation, and focus
when switching between windows in your desktop session.

### Desktop integration checks

Check the association:

```sh
xdg-mime query default x-scheme-handler/tchap-dev
```

Before release, test typing and Tab, browser login and return to Tchap, dialogs,
notifications, autostart and WebRTC. A successful build does not validate these or the release CI.
