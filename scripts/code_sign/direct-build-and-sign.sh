#!/bin/bash
# Using Tauri cli and from sources directly build the project without passing by the CI
# This script must be run in src-tauri as root (where there is tauri.conf.json file)

# direct-windows-build-and-sign.sh TARGET ENV VERSION SKIP_SOURCE SKIP_BUILD
# TARGET : can be x86_64-pc-windows-gnu or universal-apple-darwin
# ENV : prod, dev, preprod
# VERSION : x.y.z
# SKIP_SOURCE: true or false (default to false)
# SKIP_BUILD: true or false (default to false)


# Prerequisites :
# - Follow tauri installation instruction, for windows use MINGW64 and pacman -S mingw64/...rust
# use rust-target: x86_64-pc-windows-gnu and rust-toolchain: stable-x86_64-pc-windows-gnu
# - Install cli tauri : cargo install tauri-cli --version "^2.0.0" --locked


TARGET="${1:?Missing target (x86_64-pc-windows-gnu or universal-apple-darwin)}"
ENV="${2:?Missing Env}"
VERSION="${3:?Missing version}"
SKIP_SOURCE="${4:-keep}"
SKIP_BUILD_FRONTEND="${5:-keep}"
SKIP_BUILD_TAURI="${6:-keep}"
CONFIG="./src-tauri/tauri.conf.json"
# Modify the script_dir path to match your
SCRIPT_DIR="/c/Users/DINUM/Workspace/tchap-desktop/scripts/code_sign"
WORK_DIR="${SCRIPT_DIR}/releases/${VERSION}/sources"
SOURCES_URL="https://github.com/tchapgouv/tchap-desktop/archive/refs/tags/tchap-${VERSION}.tar.gz"

export OPENSSL_NO_VENDOR=1
export RUSTFLAGS=-Ctarget-feature=+crt-static
export SSL_CERT_FILE="/c/Users/DINUM/OpenSSL-win64/cacert.pem"

echo "$ENV"
case "$ENV" in
    dev)
        CONFIG = "./src-tauri/tauri.conf.dev.json"
    ;;
    preprod)
        CONFIG = "./src-tauri/tauri.conf.preprod.json"
    ;;
esac

setup_directories() {
    echo "Setting up directories..."
    mkdir -p "$WORK_DIR"
    echo "Working directory created: $WORK_DIR"
}

# Download correct version source from github
download_sources() {
    echo "Downloading sources files (version: $VERSION)..."
    if curl -L -f -o "$WORK_DIR/tchap-${VERSION}.tar.gz" "$SOURCES_URL"; then
        echo "Downloaded"
    else
        echo "Failed to download"
        exit 1
    fi
}

get_updater_signing_key() {
    echo "Enter your updater private signing key"
    read -s KEY
    if [ -z "$KEY" ]; then
        echo "KEY cannot be empty"
        exit 1
    fi
    export TAURI_SIGNING_PRIVATE_KEY="$KEY"
    echo "KEY set"
}

get_updater_signing_key

# Download sources
if [ "$SKIP_SOURCE" = "keep" ]; then
    setup_directories
    download_sources
    echo "Extracting sources now"
    tar -xf "$WORK_DIR/tchap-${VERSION}.tar.gz" -C $WORK_DIR --strip-components=1
fi

pushd "$WORK_DIR"
if [ "$SKIP_BUILD_FRONTEND" = "keep" ]; then
    #  build apps
    # build front end
    echo "installing frontend package"
    npm install
    npm run fetch-package -- $ENV
fi

if [ "$SKIP_BUILD_TAURI" = "keep" ]; then
    #  build apps
    # build front end
    echo "building tauri"
    cargo tauri build -c $CONFIG -t $TARGET --no-bundle
fi



# # Sign apps
echo "Signing the app"
echo "$pwd"
case "$TARGET" in
    x86_64-pc-windows-gnu)
        cargo tauri bundle -c $CONFIG -c "./src-tauri/tauri.conf.sign-windows.json" -t $TARGET
    ;;
    universal-apple-darwin)
    cargo tauri bundle -c $CONFIG -t $TARGET
    ;;
    *)
    echo No correct target found $TARGET
esac
