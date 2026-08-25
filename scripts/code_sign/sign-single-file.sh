#!/bin/bash

# Used by cargo tauri bundle sign command to sign the file given in argument
# Adapt the osslsigncode lib path to your platform

#  To find the cert thumbprint run certutil -scinfo
#  And next to your Cert there is a base64 value
# If harica timestamp not working, use http://timestamp.digicert.com

set -e

# Configuration
FILE="$1" # The file is given
OSSLSIGNCODE="C:\Users\DINUM\Workspace\osslsigncode-2.14\build\osslsigncode"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color


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

sign_file() {
    local input_file="$1"
    local filename=$(basename "$input_file")
    local output_file="${input_file%.*}_signed.${input_file##*.}"
    local
    log_info "Signing: $filename"
    # Note: -ac (additional certificates chain) is optional; remove it to test without it
    # if "$OSSLSIGNCODE" sign \
    #     -pkcs11module  "/c/Program Files/OpenSC Project/OpenSC/pkcs11/opensc-pkcs11.dll" \
    #     -pkcs11cert 'pkcs11:token=DINUM;object=Certificate%201' \
    #     -key 'pkcs11:token=DINUM;object=Private%20key%201' \
    #     -pass "$PKCS11_PIN" \
    #     -ts http://ts.harica.gr \
    #     -in "$input_file" \
    #     -out "$output_file"; then

    #     log_success "Signed: $filename -> $(basename "$output_file")"
    #     return 0
    # else
    #     log_error "Failed to sign: $filename"
    #     rm -f "$output_file"
    #     return 1
    # fi

    # For windows, using Mingw64 and signtool
    "/c/Program Files (x86)/Windows Kits/10/bin/10.0.26100.0/x64/signtool.exe" sign \
        /fd SHA256 \
        /sha1 "d6a8895445ee17bded15b88565ca16f582cb3bb7" \
        /tr http://ts.harica.gr \
        /td SHA256 \
        "$output_file"
}

################################################################################
# Main
################################################################################

main() {
    echo ""
    echo "================================================================================"
    echo "                    Tchap Release Sign file"
    echo "================================================================================"
    echo ""

    # get_pin
    # For windows the pin will be asked automatically by the sign tool
    sign_file
}

# Run main function
main "$@"
