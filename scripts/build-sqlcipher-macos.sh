#!/bin/bash
# statically compile SQLCIPHER linked to a statically compiled openssl
set -e

SQLCIPHER_VERSION="v4.5.6"
BUILD_DIR="$(pwd)/build"
INSTALL_DIR="$(pwd)/vendor/sqlcipher-macos"
OPENSSL_DIR="$(pwd)/vendor/openssl-macos"

mkdir -p "$BUILD_DIR"
cd "$BUILD_DIR"

if [ ! -d "sqlcipher" ]; then
    git clone --depth 1 --branch "$SQLCIPHER_VERSION" https://github.com/sqlcipher/sqlcipher.git
fi

cd sqlcipher

./configure \
    --enable-tempstore=yes \
    --disable-shared \
    --enable-static \
    --prefix="$INSTALL_DIR" \
    CFLAGS="-DSQLITE_HAS_CODEC -I${OPENSSL_DIR}/include" \
    LDFLAGS="-L${OPENSSL_DIR}/lib -lcrypto"

make clean
make -j$(sysctl -n hw.ncpu)
make install

echo "SQLCipher compiled and installed in $INSTALL_DIR"
