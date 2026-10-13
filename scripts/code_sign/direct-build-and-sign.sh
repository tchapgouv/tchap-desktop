#!/bin/bash
# Using Tauri cli and from sources directly build the project without passing by the CI
# This script must be run in src-tauri as root (where there is tauri.conf.json file)

# direct-build-and-sign.sh TARGET ENV VERSION NO_UPDATER SKIP_SOURCE SKIP_BUILD SKIP_TAURI
# TARGET : can be x86_64-pc-windows-msvc or universal-apple-darwin
# ENV : prod, dev, preprod
# VERSION : x.y.z
# NO_UPDATER: true or false (default to false)
# SKIP_SOURCE: true or false (default to false)
# SKIP_BUILD_FRONTEND: true or false (default to false)
# SKIP_BUILD_TAURI: true or false (default to false)


# Prerequisites :
# - install rustup and set toolchain to: rustup toolchain install stable-msvc
# - Install cli tauri : cargo install tauri-cli --version "^2.0.0" --locked

# The resulted installer are found in /realeases/VERSION/sources/src-tauri/target/x86-64-pc-windows-msvc/release/bundle

TARGET="${1:?Missing target x86_64-pc-windows-msvc or universal-apple-darwin}"
ENV="${2:?Missing Env}"
VERSION="${3:?Missing version}"
NO_UPDATER="${4:-false}"
SKIP_SOURCE="${5:-keep}"
SKIP_BUILD_FRONTEND="${6:-keep}"
SKIP_BUILD_TAURI="${7:-keep}"
CONFIG="./tauri.conf.json"
# Modify the script_dir path to match yours
SCRIPT_DIR="/c/Users/DINUM/Workspace/tchap-desktop/scripts/code_sign"
WORK_DIR="${SCRIPT_DIR}/releases/${VERSION}/sources"
SOURCES_URL="https://github.com/tchapgouv/tchap-desktop/archive/refs/tags/tchap-${VERSION}.tar.gz"

# export PATH="/c/Users/DINUM/.cargo/bin/:$PATH" needs if building for windows-gnu mingw64
export OPENSSL_NO_VENDOR=1
export RUSTFLAGS=-Ctarget-feature=+crt-static
export SSL_CERT_FILE="/c/Users/DINUM/OpenSSL-win64/cacert.pem"

echo "$ENV"
case "$ENV" in
    dev)
        CONFIG="./tauri.conf.dev.json"
    ;;
    preprod)
        CONFIG="./tauri.conf.preprod.json"
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
    echo "Enter your updater private signing key password"
    read KEY
    if [ -z "$KEY" ]; then
        echo "KEY cannot be empty"
        exit 1
    fi
    # Use git bash /c/Users/... style
    echo "Enter your updater private signing key file path"
    read KEY_PATH
    if [ ! -f "$KEY_PATH" ]; then
        echo "File not found : $KEY_PATH"
        exit 1
    fi
    export TAURI_SIGNING_PRIVATE_KEY="$(<"$KEY_PATH")"

    cat "$KEY_PATH"
    export TAURI_SIGNING_PRIVATE_KEY_PASSWORD="$KEY"
    echo "KEY and KEY PATH set"
}

get_updater_signing_key

echo "skip_source : $SKIP_SOURCE"
# Download sources
if [ "$SKIP_SOURCE" = "keep" ]; then
    setup_directories
    download_sources
    echo "Extracting sources now"
    tar -xf "$WORK_DIR/tchap-${VERSION}.tar.gz" -C $WORK_DIR --strip-components=1
fi

pushd "$WORK_DIR"
echo "SKIP_BUILD_FRONTEND : $SKIP_BUILD_FRONTEND"
if [ "$SKIP_BUILD_FRONTEND" = "keep" ]; then
    #  build apps
    # build front end
    echo "installing frontend package"
    npm install
    npm run fetch-package -- $ENV
fi

pushd "src-tauri"
echo "SKIP_BUILD_TAURI : $SKIP_BUILD_TAURI"
if [ "$SKIP_BUILD_TAURI" = "keep" ]; then
    #  build tauri
    echo "building tauri no_updater: $NO_UPDATER"
    if [ "$NO_UPDATER" = "false" ]; then
        cargo tauri build -c $CONFIG -t $TARGET --no-bundle
    else
        cargo tauri build -c $CONFIG -c "./tauri.conf.noupdater-windows.json" -c "./tauri.conf.sign-windows.json" -t "$TARGET" -f no-updater --no-bundle
    fi
fi



# # Sign apps
echo "Signing the app"

case "$TARGET" in
    x86_64-pc-windows-msvc)
        if [ "$NO_UPDATER" = "false" ]; then
            cargo tauri bundle -c $CONFIG -c "./tauri.conf.sign-windows.json" -t $TARGET
        else
            cargo tauri bundle -c $CONFIG -c "./tauri.conf.noupdater-windows.json" -c "./tauri.conf.sign-windows.json" -t $TARGET -f no-updater
        fi
        ;;
    universal-apple-darwin)
        cargo tauri bundle -c $CONFIG -t $TARGET
        ;;
    *)
    echo No correct target found $TARGET
esac
