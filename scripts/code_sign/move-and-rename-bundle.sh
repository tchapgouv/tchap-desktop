#!/bin/bash

# Get the releases artifacts from path, rename and move to desire directory
# ./move-and-rename-bundle.sh ENV VERSION NO_UPDATER
# ex: ./move-and-rename-bundle.sh prod 4.21.4 true

ENV="${1:?Missing Env}"
VERSION="${2:?Missing version}"
NO_UPDATER="${3:-false}"
RELEASE_MSI_PATH="/c/Users/DINUM/Workspace/tchap-desktop/scripts/code_sign/releases/${VERSION}/sources/src-tauri/target/x86_64-pc-windows-msvc/release/bundle/msi"
RELEASE_NSIS_PATH="/c/Users/DINUM/Workspace/tchap-desktop/scripts/code_sign/releases/${VERSION}/sources/src-tauri/target/x86_64-pc-windows-msvc/release/bundle/nsis"
OUTPUT_DIR_PATH="/c/Users/DINUM/Workspace/tchap-desktop/scripts/code_sign/releases/${VERSION}/${ENV}"


if [ ! -d "$OUTPUT_DIR_PATH" ]; then
  mkdir -p $OUTPUT_DIR_PATH
fi


# Rename artificats with correct name pattern Tchap-ENV_VERSION_x64_signed.EXT
# if its .sig file take this format Tchap-ENV_VERSION_x64.EXT_signed.EXT
# if NO_UPDATER arg is true, the format is Tchap-ENV_VERSION_x64_no_updater_signed.EXT
for file in "$RELEASE_MSI_PATH"/* "$RELEASE_NSIS_PATH"/*; do
  [ -f "$file" ] || continue
  filename=$(basename "$file")
  ext="${filename##*.}"
  echo "Processing file: $filename..."
  if [[ "$filename" == *exe.sig ]]; then
    new_name="Tchap-${ENV}_${VERSION}_x64.exe_signed.${ext}"
  elif [[ "$filename" == *msi.sig ]]; then
    new_name="Tchap-${ENV}_${VERSION}_x64.msi_signed.${ext}"
  elif [ "$NO_UPDATER" = "true" ]; then
    new_name="Tchap-${ENV}_${VERSION}_x64_no_updater_signed.${ext}"
  else
    new_name="Tchap-${ENV}_${VERSION}_x64_signed.${ext}"
  fi
  echo "New filename $new_name"
  echo "Moving files..."
  # Take all files under everything RELEASE_MSI_PATH and RELEASE_NSIS_PATH dir and move to the correct OUTPUT_DIR_PATH
  mv "$file" "$OUTPUT_DIR_PATH/$new_name"
done
