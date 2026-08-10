#!/bin/bash
# Using Tauri cli and from sources directly build the project without passing by the CI
# This script must be run in src-tauri as root (where there is tauri.conf.json file)

# direct-windows-build-and-sign.sh TARGET ENV VERSION SKIP_SOURCE SKIP_BUILD
# TARGET : can be x86_64-pc-windows-msvc or universal-apple-darwin
# ENV : prod, dev, preprod
# VERSION : x.y.z
# SKIP_SOURCE: true or false (default to false)
# SKIP_BUILD: true or false (default to false)


# Prerequisites :
# - Follow tauri instzallation instruction
# - Install cli tauri : cargo install tauri-cli --version "^2.0.0" --locked


# ENV to set for openssl and build to work on windows system
# Need install https://stackoverflow.com/questions/55912871/how-to-work-with-openssl-for-rust-within-a-windows-development-environment
# set VCPKG_ROOT="C:\Users\DINUM\vcpkg" https://github.com/Microsoft/vcpkg#quick-start-windows
# set SSL_CERT_FILE="C:\Users\DINUM\OpenSSL-win64\cacert.pem" https://curl.se/docs/caextract.html
# set OPENSSL_NO_VENDOR=1
# set RUSTFLAGS=-Ctarget-feature=+crt-static
TARGET="${2:?Missing target (x86_64-pc-windows-msvc or universal-apple-darwin)}"
ENV="${2:?Missing Env}"
VERSION="${3:?Missing version}"
SKIP_SOURCE="${4:?false}"
SKIP_BUILD="${5:?false}"
CONFIG="./tauri.conf.json"
SCRIPT_DIR="../scripts/code_sign"
WORK_DIR="${SCRIPT_DIR}/releases/${VERSION}/sources"
SOURCES_URL="https://github.com/tchapgouv/tchap-desktop/archive/refs/tags/tchap-${VERSION}.tar.gz"

case ENV in
    dev)
        CONFIG = "./tauri.conf.dev.json"
    ;;
    preprod)
        CONFIG = "./tauri.conf.preprod.json"
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

# Download sources
if [ $SKIP_SOURCE = false ]; then
    setup_directories
    download_sources
    echo "Extracting sources now"
    tar -xf "$WORK_DIR/tchap-${VERSION}.tar.gz" -C $WORK_DIR --strip-components=1
fi

if [ $SKIP_BUILD = false ]; then
    #  build apps
    # build front end
    pushd "$WORK_DIR"
    echo "installing frontend package"
    npm install
    npm run fetch-package -- $ENV
    popd
    # build tauri app
    pushd "$WORK_DIR/src-tauri"
    cargo tauri build -v -t $TARGET -c $CONFIG
    # cargo tauri build -v -t 'x86_64-pc-windows-msvc' -c "./tauri.conf.noupdater-windows.json" -f "no-updater"
fi

# # Sign apps
echo "Signing the app"
case TARGET in
    x86_64-pc-windows-msvc)
        cargo tauri bundle -v  -a ./releases/Tchap-$ENV_${version}x64.msi --sign-command "./sign-releases-ossl_linux.sh $VERSION $ENV"
        cargo tauri bundle -v  -a ./releases/Tchap-$ENV_${version}x64_no_updater.msi --sign-command "./sign-releases-ossl_linux.sh $VERSION $ENV"
        cargo tauri bundle -v  -a ./releases/Tchap-$ENV_${version}x64.exe --sign-command "./sign-releases-ossl_linux.sh $VERSION $ENV"
        cargo tauri bundle -v  -a ./releases/Tchap-$ENV_${version}x64_no_updater.exe --sign-command "./sign-releases-ossl_linux.sh $VERSION $ENV"
    ;;
    universal-apple-darwin)
        cargo tauri bundle -v  -a ./releases/Tchap-$ENV_${version}_universal.dmg --sign-command "./sign-releases-ossl_mac.sh $VERSION $ENV"
    ;;
    *)
        echo No correct target found
esac
