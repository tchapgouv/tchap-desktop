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
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    sed -i '' 's/version = "'$current_version'"/version = "'$new_version'"/' src-tauri/Cargo.toml
else
    # Linux
    sed -i 's/version = "'$current_version'"/version = "'$new_version'"/' src-tauri/Cargo.toml
fi

# Update version in tauri.conf for prod environment
jq '.version = "'"$new_version"'"' src-tauri/tauri.conf.json > src-tauri/tauri.conf.json.tmp && mv src-tauri/tauri.conf.json.tmp src-tauri/tauri.conf.json

# Update version in tauri.conf.noupdater.json for prod environment
jq '.version = "'"$new_version"'"' src-tauri/tauri.conf.noupdater-windows.json > src-tauri/tauri.conf.noupdater-windows.json.tmp && mv src-tauri/tauri.conf.noupdater-windows.json.tmp src-tauri/tauri.conf.noupdater-windows.json

# Update version in tauri.conf.dev.json for dev environment
jq '.version = "'"$new_version"'"' src-tauri/tauri.conf.dev.json > src-tauri/tauri.conf.dev.json.tmp && mv src-tauri/tauri.conf.dev.json.tmp src-tauri/tauri.conf.dev.json

# Update version in tauri.conf.preprod.json for preprod environment
jq '.version = "'"$new_version"'"' src-tauri/tauri.conf.preprod.json > src-tauri/tauri.conf.preprod.json.tmp && mv src-tauri/tauri.conf.preprod.json.tmp src-tauri/tauri.conf.preprod.json


# Update the version in package.json
jq '.version = "'"$new_version"'"' package.json > package.json.tmp && mv package.json.tmp package.json

echo "Version updated from $current_version to $new_version"
