#!/bin/bash
# statically compile openssl for ARM64 arch
set -e

OPENSSL_VERSION="3.2.1"
BUILD_DIR="$(pwd)/build"
INSTALL_DIR="$(pwd)/vendor/openssl-macos-arm64"

mkdir -p "$BUILD_DIR"
cd "$BUILD_DIR"

if [ ! -d "openssl-${OPENSSL_VERSION}" ]; then
    curl -L "https://www.openssl.org/source/openssl-${OPENSSL_VERSION}.tar.gz" -o openssl.tar.gz
    tar xzf openssl.tar.gz
fi

cd "openssl-${OPENSSL_VERSION}"

./Configure darwin64-arm64-cc \
    no-shared \
    --prefix="$INSTALL_DIR" \
    --openssldir="$INSTALL_DIR"

make clean
make -j$(sysctl -n hw.ncpu)
make install_sw

echo "openssl compiled and installed in $INSTALL_DIR"
