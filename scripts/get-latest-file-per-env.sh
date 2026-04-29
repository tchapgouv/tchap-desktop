
#!/bin/bash

# This script generate for each env a latest.json file used for tauri updater
#  The latest json looks like this
# {
#   "version": "4.19.5",
#   "notes": "",
#   "pub_date": "2026-04-15T14:29:30.604Z",
#   "platforms": {
#     "linux-x86_64": {
#       "signature": "dW50cnVzdGVkIGNvbW1lbnQ6IHNpZ25hdHVyZSBmcm9tIHRhdXJpIHNlY3JldCBrZXkKUlVUUyt0VEJNWWVnUnU2M293Slh4dWZLQ3RPUHU1eGFrNXNTNE9TTFFJaExoNU9OWi9lcjFYQ2pqQWJYQTFiVk4vUGhLUXRtMGhETFVESDlrWnFEL1JmSWhxam1EVDhMMlFJPQp0cnVzdGVkIGNvbW1lbnQ6IHRpbWVzdGFtcDoxNzc2MjYyOTIwCWZpbGU6VGNoYXBfNC4xOS41X2FtZDY0LkFwcEltYWdlCm1US0RZVmV6QU1CQTIweXRwWUhVakJUWFcxaXgydTlKdFZQSUVVT0dDUEx3T3hrRXQzM0hvMVFwRVVHa3gvWlRHeVZsa2Fxa0w3VXQzcy9MdmR1OUN3PT0K",
#       "url": "https://github.com/tchapgouv/tchap-desktop/releases/download/tchap-4.19.5/Tchap-prod_4.19.5_amd64.AppImage"
#     },
#     "linux-x86_64-appimage": {
#       "signature": "dW50cnVzdGVkIGNvbW1lbnQ6IHNpZ25hdHVyZSBmcm9tIHRhdXJpIHNlY3JldCBrZXkKUlVUUyt0VEJNWWVnUnU2M293Slh4dWZLQ3RPUHU1eGFrNXNTNE9TTFFJaExoNU9OWi9lcjFYQ2pqQWJYQTFiVk4vUGhLUXRtMGhETFVESDlrWnFEL1JmSWhxam1EVDhMMlFJPQp0cnVzdGVkIGNvbW1lbnQ6IHRpbWVzdGFtcDoxNzc2MjYyOTIwCWZpbGU6VGNoYXBfNC4xOS41X2FtZDY0LkFwcEltYWdlCm1US0RZVmV6QU1CQTIweXRwWUhVakJUWFcxaXgydTlKdFZQSUVVT0dDUEx3T3hrRXQzM0hvMVFwRVVHa3gvWlRHeVZsa2Fxa0w3VXQzcy9MdmR1OUN3PT0K",
#       "url": "https://github.com/tchapgouv/tchap-desktop/releases/download/tchap-4.19.5/Tchap-prod_4.19.5_amd64.AppImage"
#     },
#     "linux-x86_64-deb": {
#       "signature": "dW50cnVzdGVkIGNvbW1lbnQ6IHNpZ25hdHVyZSBmcm9tIHRhdXJpIHNlY3JldCBrZXkKUlVUUyt0VEJNWWVnUmlsalhGSWVRMkc0OFZuZ09PL3l6cS9reE1qa1ZHdDZGY1AwRytZbFJsWHdjOW5iVGd3anI3cEExNlVXNFRmanM2bHZoaXRQQ0VvNDZyY1lFQ2VHRmdBPQp0cnVzdGVkIGNvbW1lbnQ6IHRpbWVzdGFtcDoxNzc2MjYyOTIwCWZpbGU6VGNoYXBfNC4xOS41X2FtZDY0LmRlYgoraEE2T2w2MlVsUk4wZkQrT3Y0NVIvaDNRZW5OUTYzbE9SRTE4TjNmYVV0NHdkNm5NTmZJcVhYRzdFSmx2eUVRSXlQU2hxVEowNnVGT2RvUWFkeUNDZz09Cg==",
#       "url": "https://github.com/tchapgouv/tchap-desktop/releases/download/tchap-4.19.5/Tchap-prod_4.19.5_amd64.deb"
#     },
#     "linux-x86_64-rpm": {
#       "signature": "dW50cnVzdGVkIGNvbW1lbnQ6IHNpZ25hdHVyZSBmcm9tIHRhdXJpIHNlY3JldCBrZXkKUlVUUyt0VEJNWWVnUmx6SVZwUFArZFJBb3R5VzViUGhpaHo3MHJvQjVEaEJyaUIwTEFDeiszcmVEd09OL3cvSzRnU1hpOGFqd0hTeGpZVGY3SFdpdElEZ3BST1UvM0hBN2dRPQp0cnVzdGVkIGNvbW1lbnQ6IHRpbWVzdGFtcDoxNzc2MjYyOTIwCWZpbGU6VGNoYXAtNC4xOS41LTEueDg2XzY0LnJwbQpCUzVqcERabS9HelBlSWFXUE9KRnNMeG43ZUJhZ09RU3RyTG1nOC9NN2xBVlFPWUkyTVhjcVd0dXQxdHB0c0g5VXhVcG50ZjJTVjlHVTI4K2tRMU1BZz09Cg==",
#       "url": "https://github.com/tchapgouv/tchap-desktop/releases/download/tchap-4.19.5/Tchap-prod_4.19.5_x86_64.rpm"
#     },
#     "darwin-aarch64": {
#       "signature": "dW50cnVzdGVkIGNvbW1lbnQ6IHNpZ25hdHVyZSBmcm9tIHRhdXJpIHNlY3JldCBrZXkKUlVUUyt0VEJNWWVnUmhTdFBlQS96d3JQVi95aDJqVjZsdDAxMXROQkJ6NEp5SVliU2ZYTWtOU0hYV2lBTVVhY2RTQUozZEwyTUdEdE0ydlNtMDJ5d3N1Z1ZFdmk0NEJzRkFNPQp0cnVzdGVkIGNvbW1lbnQ6IHRpbWVzdGFtcDoxNzc2MjYyOTY1CWZpbGU6VGNoYXAuYXBwLnRhci5negpvMnJoT21ybkJvd082VVpKME43VS9WbnUzS1NtbHgvcjk1R0Myc1NVcHBoSHo5UzVxT3ZJelRSNWxNaGhYdTEySFJOb052eDU1c0NxbW5hSFVjSW1DUT09Cg==",
#       "url": "https://github.com/tchapgouv/tchap-desktop/releases/download/tchap-4.19.5/Tchap-prod_4.19.5_universal.app.tar.gz"
#     },
#     "darwin-x86_64": {
#       "signature": "dW50cnVzdGVkIGNvbW1lbnQ6IHNpZ25hdHVyZSBmcm9tIHRhdXJpIHNlY3JldCBrZXkKUlVUUyt0VEJNWWVnUmhTdFBlQS96d3JQVi95aDJqVjZsdDAxMXROQkJ6NEp5SVliU2ZYTWtOU0hYV2lBTVVhY2RTQUozZEwyTUdEdE0ydlNtMDJ5d3N1Z1ZFdmk0NEJzRkFNPQp0cnVzdGVkIGNvbW1lbnQ6IHRpbWVzdGFtcDoxNzc2MjYyOTY1CWZpbGU6VGNoYXAuYXBwLnRhci5negpvMnJoT21ybkJvd082VVpKME43VS9WbnUzS1NtbHgvcjk1R0Myc1NVcHBoSHo5UzVxT3ZJelRSNWxNaGhYdTEySFJOb052eDU1c0NxbW5hSFVjSW1DUT09Cg==",
#       "url": "https://github.com/tchapgouv/tchap-desktop/releases/download/tchap-4.19.5/Tchap-prod_4.19.5_universal.app.tar.gz"
#     },
#     "darwin-universal": {
#       "signature": "dW50cnVzdGVkIGNvbW1lbnQ6IHNpZ25hdHVyZSBmcm9tIHRhdXJpIHNlY3JldCBrZXkKUlVUUyt0VEJNWWVnUmhTdFBlQS96d3JQVi95aDJqVjZsdDAxMXROQkJ6NEp5SVliU2ZYTWtOU0hYV2lBTVVhY2RTQUozZEwyTUdEdE0ydlNtMDJ5d3N1Z1ZFdmk0NEJzRkFNPQp0cnVzdGVkIGNvbW1lbnQ6IHRpbWVzdGFtcDoxNzc2MjYyOTY1CWZpbGU6VGNoYXAuYXBwLnRhci5negpvMnJoT21ybkJvd082VVpKME43VS9WbnUzS1NtbHgvcjk1R0Myc1NVcHBoSHo5UzVxT3ZJelRSNWxNaGhYdTEySFJOb052eDU1c0NxbW5hSFVjSW1DUT09Cg==",
#       "url": "https://github.com/tchapgouv/tchap-desktop/releases/download/tchap-4.19.5/Tchap-prod_4.19.5_universal.app.tar.gz"
#     },
#     "darwin-aarch64-app": {
#       "signature": "dW50cnVzdGVkIGNvbW1lbnQ6IHNpZ25hdHVyZSBmcm9tIHRhdXJpIHNlY3JldCBrZXkKUlVUUyt0VEJNWWVnUmhTdFBlQS96d3JQVi95aDJqVjZsdDAxMXROQkJ6NEp5SVliU2ZYTWtOU0hYV2lBTVVhY2RTQUozZEwyTUdEdE0ydlNtMDJ5d3N1Z1ZFdmk0NEJzRkFNPQp0cnVzdGVkIGNvbW1lbnQ6IHRpbWVzdGFtcDoxNzc2MjYyOTY1CWZpbGU6VGNoYXAuYXBwLnRhci5negpvMnJoT21ybkJvd082VVpKME43VS9WbnUzS1NtbHgvcjk1R0Myc1NVcHBoSHo5UzVxT3ZJelRSNWxNaGhYdTEySFJOb052eDU1c0NxbW5hSFVjSW1DUT09Cg==",
#       "url": "https://github.com/tchapgouv/tchap-desktop/releases/download/tchap-4.19.5/Tchap-prod_4.19.5_universal.app.tar.gz"
#     },
#     "darwin-x86_64-app": {
#       "signature": "dW50cnVzdGVkIGNvbW1lbnQ6IHNpZ25hdHVyZSBmcm9tIHRhdXJpIHNlY3JldCBrZXkKUlVUUyt0VEJNWWVnUmhTdFBlQS96d3JQVi95aDJqVjZsdDAxMXROQkJ6NEp5SVliU2ZYTWtOU0hYV2lBTVVhY2RTQUozZEwyTUdEdE0ydlNtMDJ5d3N1Z1ZFdmk0NEJzRkFNPQp0cnVzdGVkIGNvbW1lbnQ6IHRpbWVzdGFtcDoxNzc2MjYyOTY1CWZpbGU6VGNoYXAuYXBwLnRhci5negpvMnJoT21ybkJvd082VVpKME43VS9WbnUzS1NtbHgvcjk1R0Myc1NVcHBoSHo5UzVxT3ZJelRSNWxNaGhYdTEySFJOb052eDU1c0NxbW5hSFVjSW1DUT09Cg==",
#       "url": "https://github.com/tchapgouv/tchap-desktop/releases/download/tchap-4.19.5/Tchap-prod_4.19.5_universal.app.tar.gz"
#     },
#     "darwin-universal-app": {
#       "signature": "dW50cnVzdGVkIGNvbW1lbnQ6IHNpZ25hdHVyZSBmcm9tIHRhdXJpIHNlY3JldCBrZXkKUlVUUyt0VEJNWWVnUmhTdFBlQS96d3JQVi95aDJqVjZsdDAxMXROQkJ6NEp5SVliU2ZYTWtOU0hYV2lBTVVhY2RTQUozZEwyTUdEdE0ydlNtMDJ5d3N1Z1ZFdmk0NEJzRkFNPQp0cnVzdGVkIGNvbW1lbnQ6IHRpbWVzdGFtcDoxNzc2MjYyOTY1CWZpbGU6VGNoYXAuYXBwLnRhci5negpvMnJoT21ybkJvd082VVpKME43VS9WbnUzS1NtbHgvcjk1R0Myc1NVcHBoSHo5UzVxT3ZJelRSNWxNaGhYdTEySFJOb052eDU1c0NxbW5hSFVjSW1DUT09Cg==",
#       "url": "https://github.com/tchapgouv/tchap-desktop/releases/download/tchap-4.19.5/Tchap-prod_4.19.5_universal.app.tar.gz"
#     },
#     "windows-x86_64": {
#       "signature": "dW50cnVzdGVkIGNvbW1lbnQ6IHNpZ25hdHVyZSBmcm9tIHRhdXJpIHNlY3JldCBrZXkKUlVUUyt0VEJNWWVnUm84alZialZCdnlZSDNZSDlhQS9sVHdMUlFXZGFlbzl0NDFNUEZvM3pBYkhISlVISGRmQnVUV2ZQZVdraUdsN1YyZ3lNWEFVUlAvZTY1emYyc1JrU2dvPQp0cnVzdGVkIGNvbW1lbnQ6IHRpbWVzdGFtcDoxNzc2MjYzMzY0CWZpbGU6VGNoYXBfNC4xOS41X3g2NF9mci1GUi5tc2kKTmhtQ2VJbCs4L2wvemxyYTZ6aG5TMnI1UFBONGdMOXhqcHpQY3VOWmZ0TTcrcDQzSFF4RTJ5ZTA4cmhJa2g0QVVydkZ3REp0Wm1uelZzZ3lObHczQWc9PQo=",
#       "url": "https://github.com/tchapgouv/tchap-desktop/releases/download/tchap-4.19.5/Tchap-prod_4.19.5_x64.msi"
#     },
#     "windows-x86_64-msi": {
#       "signature": "dW50cnVzdGVkIGNvbW1lbnQ6IHNpZ25hdHVyZSBmcm9tIHRhdXJpIHNlY3JldCBrZXkKUlVUUyt0VEJNWWVnUm84alZialZCdnlZSDNZSDlhQS9sVHdMUlFXZGFlbzl0NDFNUEZvM3pBYkhISlVISGRmQnVUV2ZQZVdraUdsN1YyZ3lNWEFVUlAvZTY1emYyc1JrU2dvPQp0cnVzdGVkIGNvbW1lbnQ6IHRpbWVzdGFtcDoxNzc2MjYzMzY0CWZpbGU6VGNoYXBfNC4xOS41X3g2NF9mci1GUi5tc2kKTmhtQ2VJbCs4L2wvemxyYTZ6aG5TMnI1UFBONGdMOXhqcHpQY3VOWmZ0TTcrcDQzSFF4RTJ5ZTA4cmhJa2g0QVVydkZ3REp0Wm1uelZzZ3lObHczQWc9PQo=",
#       "url": "https://github.com/tchapgouv/tchap-desktop/releases/download/tchap-4.19.5/Tchap-prod_4.19.5_x64.msi"
#     },
#     "windows-x86_64-nsis": {
#       "signature": "dW50cnVzdGVkIGNvbW1lbnQ6IHNpZ25hdHVyZSBmcm9tIHRhdXJpIHNlY3JldCBrZXkKUlVUUyt0VEJNWWVnUnVvOEorY3ZmRlpMVnhSN0dNT0tTM3RMamZ3N0FuZWpNa2QwS2JFWjI2SG9pTVM4aEl4ZlVVMEorWXN6V3Fpcnd4YjJaZHdKbVV0WlBodjd2Vyt2dlFBPQp0cnVzdGVkIGNvbW1lbnQ6IHRpbWVzdGFtcDoxNzc2MjYzMzY0CWZpbGU6VGNoYXBfNC4xOS41X3g2NC1zZXR1cC5leGUKTTVlb2lJa2pReXpOM3lVVVVmQTh4WFBKMEZtY1ZLTVlNY1hlZk5FYWY4TTJzSzF4SHc5aitRUmlOM3pZLzJIbTBNbEZBSmJVSFlJeTRTUTA1Y1U5Q2c9PQo=",
#       "url": "https://github.com/tchapgouv/tchap-desktop/releases/download/tchap-4.19.5/Tchap-prod_4.19.5_x64.exe"
#     }
#   }
# }

# It consider that a release.json file is available which list the assets from a tag version
# 
# This is run in the CI. If you want to test locally, you have to generate your github token and input the correct api url
#  curl -s \
#     -H "Authorization: Bearer ${{ secrets.GITHUB_TOKEN }}" \
#     https://api.github.com/repos/${{ github.repository }}/releases/tags/${{ github.ref_name }} \
#     > release.json

# $1 first argument should be tag name  github.ref_name" 
# $2 second argument should be the env
VERSION=$(echo $1 | grep -oE '[0-9]+\.[0-9]+\.[0-9]+')
ENV=$2
RELEASE_FILE=release.json
PUB_DATE=$(jq -r '.created_at' $RELEASE_FILE) # get date from release json file

# platform → filename pattern
declare -A MAP=(
  ["linux-x86_64"]="Tchap-"$ENV"_"$VERSION"_amd64.AppImage"
  ["linux-x86_64-appimage"]="Tchap-"$ENV"_"$VERSION"_amd64.AppImage"
  ["linux-x86_64-deb"]="Tchap-${ENV}_${VERSION}_amd64.deb"
  ["linux-x86_64-rpm"]="Tchap-${ENV}_${VERSION}_x86_64.rpm"
  ["darwin-aarch64"]="Tchap-${ENV}_${VERSION}_universal.app.tar.gz"
  ["darwin-x86_64"]="Tchap-${ENV}_${VERSION}_universal.app.tar.gz"
  ["darwin-aarch64-app"]="Tchap-${ENV}_${VERSION}_universal.app.tar.gz"
  ["darwin-universal"]="Tchap-${ENV}_${VERSION}_universal.app.tar.gz"
  ["darwin-x86_64-app"]="Tchap-${ENV}_${VERSION}_universal.app.tar.gz"
  ["windows-x86_64-msi"]="Tchap-${ENV}_${VERSION}_x64.msi"
  ["windows-x86_64"]="Tchap-${ENV}_${VERSION}_x64.msi"
  ["windows-x86_64-nsis"]="Tchap-${ENV}_${VERSION}_x64.exe"
)

declare -A BASE_URL_MAP=(
  ["dev"]="https://tchap.incubateur.net"
  ["preprod"]="https://beta.tchap.gouv.fr"
  ["prod"]="https://tchap.gouv.fr"
)

jq -n \
--arg version "$VERSION" \
--arg pub_date "$PUB_DATE" \
'
{
    version: $version,
    notes: "",
    pub_date: $pub_date,
    platforms: {}
}
' > latest_${ENV}.json

for platform in "${!MAP[@]}"; do
    filename="${MAP[$platform]}"
    echo $filename
    case "$platform" in
        # for windows we use our own self hosted files
        windows-x86_64-msi|windows-x86_64-nsis|windows-x86_64)
        asset_url="${BASE_URL_MAP[${ENV}]}/tchap-desktop/${filename}"

        sig_url=$(jq -r --arg p "$filename" '
            .assets[] | select(.name | test($p + "\\.sig$")) | .browser_download_url
        ' $RELEASE_FILE | head -n1)
        ;;
        *)
        # default behavior for all other platforms
        asset_url=$(jq -r --arg p "$filename" '
            .assets[] | select(.name | test($p)) | .browser_download_url
        ' $RELEASE_FILE | head -n1)

        sig_url=$(jq -r --arg p "$filename" '
            .assets[] | select(.name | test($p + "\\.sig$")) | .browser_download_url
        ' $RELEASE_FILE | head -n1)
        ;;
    esac
    # Fail fast if binary missing
    if [ -z "$asset_url" ]; then
        echo "❌ Missing asset for $platform"
        exit 1
    fi

    signature=""

    if [ -n "$sig_url" ]; then
        echo "Fetching signature for $platform"

        # download
        signature=$(curl -sL "$sig_url")
    else
        echo "⚠️ No signature found for $platform"
    fi

    # create temporary file, in order not to write and read directly from output.json
    tmp=$(mktemp)

    jq \
        --arg p "$platform" \
        --arg url "$asset_url" \
        --arg sig "$signature" \
        '.platforms[$p] = {url: $url, signature: $sig}' \
        latest_${ENV}.json > "$tmp"

    mv "$tmp" latest_${ENV}.json
done

echo "Generated JSON:"
cat latest_${ENV}.json
