#!/bin/bash

# Get the current version from package.json
current_version=$(jq -r '.version' package.json)

# Get new version from argument
new_version=$1

# Check version format x.y.z
if [[ ! $new_version =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo "Invalid version format. Please use x.y.z"
    exit 1
fi

# Update the version in src-tauri/Cargo.toml
sed -i 's/version = "'$current_version'"/version = "'$new_version'"/' src-tauri/Cargo.toml

# Update the version in package.json
jq '.version = "'"$new_version"'"' package.json > package.json.tmp && mv package.json.tmp package.json

echo "Version updated from $current_version to $new_version"


# Update endpoint version  for updater plugin
CONFIG_FILE="./src-tauri/tauri.conf.json"  # Path to your tauri.conf.json file
TEMP_FILE="./src-tauri/tauri.conf.json.tmp"
# Find the download URL for the matching asset
DOWNLOAD_URL="https://github.com/tchapgouv/tchap-desktop/releases/download/tchap-${new_version}/latest.json" 

echo "Found download URL: $DOWNLOAD_URL"

# Update both the version and endpoints in one jq call
if jq --arg ver "$new_version" --arg url "$DOWNLOAD_URL" \
    '.version = $ver | .plugins.updater.endpoints = [$url]' "$CONFIG_FILE" > "$TEMP_FILE"; then
    if [ -s "$TEMP_FILE" ]; then
        mv "$TEMP_FILE" "$CONFIG_FILE"
        echo "Successfully updated version to $new_version and endpoints to $DOWNLOAD_URL in $CONFIG_FILE"
    else
        echo "Error: jq produced an empty file"
        rm "$TEMP_FILE"
        exit 1
    fi
else
    echo "Error: jq command failed"
    rm "$TEMP_FILE"
    exit 1
fi