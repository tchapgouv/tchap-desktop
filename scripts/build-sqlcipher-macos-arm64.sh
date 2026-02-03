#!/bin/bash
# statically compile SQLCIPHER linked to a statically compiled openssl
set -e

SQLCIPHER_VERSION="v4.13.0"
BUILD_DIR="$(pwd)/build"
INSTALL_DIR="$(pwd)/vendor/sqlcipher-macos-arm64"
OPENSSL_DIR="$(pwd)/vendor/openssl-macos-arm64"

mkdir -p "$BUILD_DIR"
cd "$BUILD_DIR"

if [ ! -d "sqlcipher" ]; then
    git clone --depth 1 --branch "$SQLCIPHER_VERSION" https://github.com/sqlcipher/sqlcipher.git
fi

cd sqlcipher

# Build with full-text search enabled (from homebrew recipe)
CFLAGS="-DSQLITE_HAS_CODEC \
-DSQLITE_ENABLE_JSON1 \
-DSQLITE_ENABLE_FTS3 \
-DSQLITE_ENABLE_FTS3_PARENTHESIS \
-DSQLITE_ENABLE_FTS5 \
-DSQLITE_ENABLE_COLUMN_METADATA \
-DSQLITE_EXTRA_INIT=sqlcipher_extra_init \
-DSQLITE_EXTRA_SHUTDOWN=sqlcipher_extra_shutdown \
-I${OPENSSL_DIR}/include"

./configure \
    --with-tempstore=yes \
    --prefix="$INSTALL_DIR" \
    --disable-shared \
    --enable-static \
    --disable-tcl \
    --enable-load-extension \
    CFLAGS="$CFLAGS" \
    LDFLAGS="-L${OPENSSL_DIR}/lib -lcrypto"


make clean
make -j
make install

echo "SQLCipher compiled and installed in $INSTALL_DIR"
