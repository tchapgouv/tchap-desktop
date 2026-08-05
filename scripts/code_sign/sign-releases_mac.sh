#!/bin/bash

################################################################################
# Script: sign-releases.sh
# Description: Download Tchap desktop releases and sign them with osslsigncode
# Usage: ./sign-releases.sh [VERSION]
# Example: ./sign-releases.sh 4.21.1 prod
################################################################################



# prerequisites


## on MacOS install :
# brew install libp11"
## on MacOS install :
# https://github.com/OpenSC/OpenSC/releases/download/0.27.1/OpenSC-0.27.1.dmg"


## (optional, should test without it) Télécharger le certificat racine HARICA EV Code Signing
# curl -o harica-root.crt https://www.harica.gr/files/certificates/harica_ecc_rsa_root_ca.crt
## Télécharger le certificat intermédiaire
# curl -o harica-intermediate.crt https://www.harica.gr/files/certificates/harica_code_signing_rsa_ca.crt
## Créer une chaîne de certificats PEM
# cat harica-root.crt harica-intermediate.crt > harica-chain.pem









set -e

# Configuration
VERSION="${1:-4.21.1}"
ENV="$2"
OSSLSIGNCODE="/Users/olivier/workspace/tchap/desktop/osslsigncode-2.14-macOS/bin/osslsigncode"
HARICA_CHAIN="/Users/olivier/workspace/tchap/desktop/osslsigncode-2.14-macOS/bin/harica-chain.pem"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK_DIR="${SCRIPT_DIR}/releases/${VERSION}"
GITHUB_BASE_URL="https://github.com/tchapgouv/tchap-desktop/releases/download/tchap-${VERSION}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Files to download
FILES=(
    "Tchap-${ENV}_${VERSION}_x64.exe"
    "Tchap-${ENV}_${VERSION}_x64.msi"
    "Tchap-${ENV}_${VERSION}_x64_no_updater.exe"
    "Tchap-${ENV}_${VERSION}_x64_no_updater.msi"
)

################################################################################
# Functions
################################################################################

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

check_prerequisites() {
    log_info "Checking prerequisites..."

    if [ ! -f "$OSSLSIGNCODE" ]; then
        log_error "osslsigncode not found at: $OSSLSIGNCODE"
        exit 1
    fi
    log_success "osslsigncode found"

    if [ ! -f "$HARICA_CHAIN" ]; then
        log_error "harica-chain.pem not found at: $HARICA_CHAIN"
        exit 1
    fi
    log_success "harica-chain.pem found"

    if ! command -v curl &> /dev/null; then
        log_error "curl is not installed"
        exit 1
    fi
    log_success "curl found"
}

setup_directories() {
    log_info "Setting up directories..."
    mkdir -p "$WORK_DIR"
    log_success "Working directory created: $WORK_DIR"
}

get_pin() {
    log_info "Enter your PKCS#11 PIN:"
    read -s PIN
    if [ -z "$PIN" ]; then
        log_error "PIN cannot be empty"
        exit 1
    fi
    export PKCS11_PIN="$PIN"
    log_success "PIN set"
}

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

sign_file() {
    local input_file="$1"
    local filename=$(basename "$input_file")
    local output_file="${input_file%.*}_signed.${input_file##*.}"

    log_info "on MacOS install : brew install libp11"
    log_info "on MacOS install : https://github.com/OpenSC/OpenSC/releases/download/0.27.1/OpenSC-0.27.1.dmg"

    log_info "Signing: $filename"
    # Note: -ac (additional certificates chain) is optional; remove it to test without it
    if "$OSSLSIGNCODE" sign \
        -engine /opt/homebrew/lib/engines-3/pkcs11.dylib \
        -pkcs11module /usr/local/lib/opensc-pkcs11.so \
        -pkcs11cert 'pkcs11:token=DINUM;object=Certificate%201' \
        -key 'pkcs11:token=DINUM;object=Private%20key%201' \
        -pass "$PKCS11_PIN" \
        -ts http://ts.harica.gr \
        -ac "$HARICA_CHAIN" \
        -in "$input_file" \
        -out "$output_file"; then

        log_success "Signed: $filename -> $(basename "$output_file")"
        return 0
    else
        log_error "Failed to sign: $filename"
        rm -f "$output_file"
        return 1
    fi
}


sign_and_extract_all() {
    log_info "Signing all files..."

    local failed=0

    for file in "${FILES[@]}"; do
        input_file="${WORK_DIR}/${file}"

        if [ ! -f "$input_file" ]; then
            log_error "File not found: $input_file"
            ((failed++))
            continue
        fi

        if ! sign_file "$input_file"; then
            ((failed++))
            continue
        fi

        # Remove original unsigned file
        rm -f "$input_file"
    done

    return $failed
}

verify_signatures() {
    log_info "Verifying signatures..."

    for file in "${FILES[@]}"; do
        signed_file="${WORK_DIR}/${file%.*}_signed.${file##*.}"

        if [ ! -f "$signed_file" ]; then
            continue
        fi

        log_info "Verifying: $(basename "$signed_file")"
        if "$OSSLSIGNCODE" verify -in "$signed_file" 2>&1 | grep -q "Number of verified signatures: 1"; then
            log_success "Signature verified: $(basename "$signed_file")"
        else
            log_warning "Signature verification had warnings: $(basename "$signed_file") (this is normal on macOS)"
        fi
    done
}

generate_report() {
    log_info "Generating report..."

    echo ""
    echo "================================================================================"
    echo "                         SIGNING REPORT"
    echo "================================================================================"
    echo "Version: $VERSION"
    echo "Working Directory: $WORK_DIR"
    echo ""
    echo "Files ready for GitHub release:"
    echo ""

    for file in "${FILES[@]}"; do
        signed_file="${WORK_DIR}/${file%.*}_signed.${file##*.}"

        if [ -f "$signed_file" ]; then
            echo "  ✓ $(basename "$signed_file")"
        fi
    done

    echo ""
    echo "================================================================================"
    echo "Next steps:"
    echo "1. Upload files to GitHub release: tchap-${VERSION}"
    echo "2. Users can verify signatures with:"
    echo "   osslsigncode verify -in <file>"
    echo "================================================================================"
    echo ""
}

################################################################################
# Main
################################################################################

main() {
    echo ""
    echo "================================================================================"
    echo "                    Tchap Release Signing Script"
    echo "================================================================================"
    echo ""

    check_prerequisites
    setup_directories
    get_pin
    download_files

    if sign_and_extract_all; then
        verify_signatures
        generate_report
        log_success "All files signed and ready for release!"
        exit 0
    else
        log_error "Some files failed to sign"
        exit 1
    fi
}

# Run main function
main "$@"
