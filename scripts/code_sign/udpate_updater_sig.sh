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
WORK_DIR="${SCRIPT_DIR}/releases/${VERSION}"
GITHUB_BASE_URL="https://github.com/tchapgouv/tchap-desktop/releases/download/tchap-${VERSION}"

PRIVATE_KEY_PATH=${3:?Missing private key path}

# Files to download
FILES=(
    "latest-${ENV}.json"
    "Tchap-${ENV}_${VERSION}_x64.exe"
    "Tchap-${ENV}_${VERSION}_x64.msi"
)

KEYS_TO_UPDATE=(
    "windows-x86_64-nsis"
    "windows-x86_64-msi"
    "windows-x86_64"
)

download_files() {
    log_info "Downloading release files (version: $VERSION)..."

    for file in "${FILES[@]}"; do
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

# Get signature from latest.json
#
# Needs to create a *.minisig file thact is the decoded signature for exe and msi file
create_minisig_file() {
    # Get downloaded file name

    # Get the signature of the first attribute windows-x86_64

    # convert to base64 -d and create a file with the result with the same downloaded filename with .minisig added at the end
}

setup_directories() {
    log_info "Setting up directories..."
    mkdir -p "$WORK_DIR"
    log_success "Working directory created: $WORK_DIR"
}

update_latest_file_signature() {

}

# Exemple of verifying signature
# minisign -Vm ~/Downloads/Tchap-prod_4.21.1_x64.msi -P "PUB_KEY"
# a "Tchap-prod_4.21.1_x64.msi.minisign" file which contain the decoded signature needs to be in the working directory



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
    update_latest_file_signature
}

# Run main function
main "$@"
