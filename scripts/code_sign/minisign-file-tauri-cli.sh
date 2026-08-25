#!/bin/bash

# use tauri cli to sign releases for updater plugin https://github.com/tauri-apps/tauri/blob/dev/crates/tauri-cli/src/helpers/updater_signature.rs
# Usage: ./minisign_file.sh <secret_key_file> <file_to_sign> [output_sig]

set -euo pipefail

SECRET_KEY_FILE="${1:?Missing secret key file}"
FILE_TO_SIGN="${2:?Missing file to sign}"

# Get filename
FILENAME=$(basename "$FILE_TO_SIGN")

read -r -s -p "Enter your private key password: " PASSWORD

# Sign the file with tauri signer cli
cargo tauri signer sign $FILE_TO_SIGN \
    -f $SECRET_KEY_FILE \
    -p $PASSWORD >/dev/null

# Read the signature file
RESULT_PATH="$FILE_TO_SIGN.sig"
cat "$RESULT_PATH"
