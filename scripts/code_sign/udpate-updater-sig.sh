#!/bin/bash

################################################################################
# Script: update_updater_sig.sh
# Description: Take the signed build and generate the new latest.json file with updated signature
# Usage: ./update_updater_sig.sh [VERSION] [ENV]
# Example: ./update_updater_sig.sh 4.21.1 prod
################################################################################

# Prerequisites
# install minisign on your platform https://github.com/jedisct1/minisign#installation

# Configuration
VERSION="${1:-4.21.1}" # Default value to 4.21.1
ENV="${2:?Missing Env}"
# the script should be launch in the script/code_sign directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK_DIR="${SCRIPT_DIR}/releases/${VERSION}"
GITHUB_BASE_URL="https://github.com/tchapgouv/tchap-desktop/releases/download/tchap-${VERSION}"
# Should be the latest.json file downloaded from github
LATEST_JSON="${WORK_DIR}/latest_${ENV}.json"
# This file contains the private key content in minisig format.
# When generated with tauri the key pair return are base64 encoded.
# Need to decode them and put the content of the private key inside a .minisig file
PRIVATE_KEY_PATH=${3:?Missing private key path}

# Files to download
FILES_TO_DOWNLOAD=(
    "latest_${ENV}.json"
)

FILES_IN_RELEASE_DIR=(
    "Tchap-${ENV}_${VERSION}_x64_signed.exe"
    "Tchap-${ENV}_${VERSION}_x64_signed.msi"
)

KEYS_TO_UPDATE_MSI=(
    "windows-x86_64-msi"
    "windows-x86_64"
)
KEYS_TO_UPDATE_EXE=(
    "windows-x86_64-nsis"
)

# Color codes
BLUE='\033[0;34m'
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[✓]${NC} $1"
}

log_error() {
    echo -e "${RED}[✗]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[!]${NC} $1"
}


download_files() {
    log_info "Downloading release files (version: $VERSION)..."

    for file in "${FILES_TO_DOWNLOAD[@]}"; do
        url="${GITHUB_BASE_URL}/${file}"
        output="${WORK_DIR}/${file}"

        if [ -f "$output" ]; then
            log_warning "File already exists, skipping: $file"
            continue
        fi

        log_info "Downloading: $file"
        if curl -L -f -o "$output" "$url"; then
            log_success "Downloaded: $file"
        else
            log_error "Failed to download: $file"
            exit 1
        fi
    done
}

# Get the new signature for the signed files

minisign_files() {
    log_info "Creating minisign signatures for each signed version"

    for file in "${FILES_IN_RELEASE_DIR[@]}"; do
        filepath="${WORK_DIR}/${file}"

        if [ ! -f "$filepath" ]; then
            log_warning "File not found, skipping: $file"
            continue
        fi

        log_info "Signing: $file"

        # Call minisign_file.sh and capture the encoded signature
        if signature=$("${SCRIPT_DIR}/minisign-file-tauri-cli.sh" "$PRIVATE_KEY_PATH" "$filepath"); then
            log_success "Signed signature: $signature"

            # Check file extension
            if [[ "$file" == *.exe ]]; then
                for key in "${KEYS_TO_UPDATE_EXE[@]}"; do
                    update_latest_file_signature "$key" "$signature"
                done
            else
                for key in "${KEYS_TO_UPDATE_MSI[@]}"; do
                    update_latest_file_signature "$key" "$signature"
                done
            fi
        else
            log_error "Failed to sign: $file"
            exit 1
        fi
    done
}

update_latest_file_signature() {
    local key="$1"
    local signature="$2"

    log_info "Updating latest.json: $key"

    # Get current URL and modify it to add _signed before extension
    local url=$(jq -r ".platforms[\"$key\"].url" "$LATEST_JSON")
    local new_url=$(echo "$url" | sed 's/\([^/]*\)\(\.[^/.]*\)$/\1_signed\2/')

    # Update signature and URL (add _signed before file extension)
    jq --arg sig "$signature" --arg url "$new_url" \
        ".platforms[\"$key\"].signature = \$sig | .platforms[\"$key\"].url = \$url" \
        "$LATEST_JSON" > "${LATEST_JSON}.tmp"
    mv "${LATEST_JSON}.tmp" "$LATEST_JSON"

    log_success "Updated signature for: $key"
}

setup_directories() {
    log_info "Setting up directories..."
    mkdir -p "$WORK_DIR"
    log_success "Working directory created: $WORK_DIR"
}


# Exemple of verifying signature
# minisign -Vm ~/Downloads/Tchap-prod_4.21.1_x64.msi -P "PUB_KEY"
# a "Tchap-prod_4.21.1_x64.msi.minisign" file which contain the decoded signature needs to be in the working directory
# convert to base64 -d and create a file with the result with the same downloaded filename with .minisig added at the end


################################################################################
# Main
################################################################################

main() {
    echo ""
    echo "================================================================================"
    echo "                    Tchap update updater latest.json files signature"
    echo "================================================================================"
    echo ""

    setup_directories
    download_files
    minisign_files
    log_success "All signatures updated!"
}

# Run main function
main "$@"
