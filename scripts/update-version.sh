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

# Update the version in src-tauri/tauri.conf.json
sed -i 's/"version": "'$current_version'"/"version": "'$new_version'"/' src-tauri/tauri.conf.json

# Update the version in package.json
jq '.version = "'"$new_version"'"' package.json > package.json.tmp && mv package.json.tmp package.json

echo "Version updated from $current_version to $new_version"


# Update endpoint version  for updater plugin
CONFIG_FILE="./src-tauri/tauri.conf.json"  # Path to your tauri.conf.json file
# Find the download URL for the matching asset
DOWNLOAD_URL="https://github.com/tchapgouv/tchap-desktop/releases/download/tchap-${new_version}/latest.json" 

echo "Found download URL: $DOWNLOAD_URL"

jq --arg url "$DOWNLOAD_URL" '.plugins.updater.endpoints = [$url]' "$CONFIG_FILE" > "config-tmp.json" && mv "config-tmp.json" "$CONFIG_FILE"
if [ $? -eq 0 ]; then
    echo "Successfully updated $CONFIG_FILE with jq"
else
    echo "Error updating ${CONFIG_FILE} with jq"
fi