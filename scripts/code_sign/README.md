## Sign artifacts
After creating a release with the standard process. The 
Depending on your platform, choose to use `sign-release_linux.sh` or `sign-release_mac.sh`.

This will create the releases with our signed certificate, that can be found in code_sign/releases/VERSION

## Release process after signature
Once the build artifact are re-signed, it render obsolète the updater plugin signature. In other word, the signature from the latest_*.json files are not correct anymore. Since, the signature from this file is used to match the update to download, we need to update those signature.

This is the role of the `update-updater-sig.sh` script. It takes the new artifacts and sign them again with minisign and the corresponding private key for the environmenent. The key used to sign should correspond to the public key (correct key pair) fill in tauri.conf.json for the updater.


## Finalize the release
- Upload the newly signed artifact and updated latest.*.json file into the correct github release 
- Create the PR on tchap-infra side in order to push the new signed artifacts on our server
