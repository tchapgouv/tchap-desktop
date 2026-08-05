#!/bin/bash

# minisign file signer (port of Tauri's sign_file) https://github.com/tauri-apps/tauri/blob/dev/crates/tauri-cli/src/helpers/updater_signature.rs
# Usage: ./minisign_file.sh <secret_key_file> <file_to_sign> [output_sig]

set -euo pipefail

SECRET_KEY_FILE="${1:?Missing secret key file}"
FILE_TO_SIGN="${2:?Missing file to sign}"

# Get current Unix timestamp
TIMESTAMP=$(date +%s)

# Get filename
FILENAME=$(basename "$FILE_TO_SIGN")

# Create trusted comment (timestamp + filename)
TRUSTED_COMMENT="timestamp:${TIMESTAMP}	file:${FILENAME}"
# Temporary file for minisign's raw output
TEMP_SIG=$(mktemp)
trap "rm -f $TEMP_SIG" EXIT
# Sign the file with minisign
# -S: sign
# -s: secret key file
# -t: trusted comment
# -c: comment
minisign -S \
  -s "$SECRET_KEY_FILE" \
  -m "$FILE_TO_SIGN" \
  -t "$TRUSTED_COMMENT" \
  -c "signature from tauri secret key" \
  -x "$TEMP_SIG" >/dev/null # only keep the ENCODED_SIG as result from this script

# Read the signature file that is in minisign format and base64 encode it
ENCODED_SIG=$(base64 -w 0 < "$TEMP_SIG")
echo "$ENCODED_SIG"
