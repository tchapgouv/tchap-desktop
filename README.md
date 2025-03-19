
# Tchap Desktop

Client Desktop Tchap avec Tauri ( https://tauri.app )

## Dev local

- Install prerequisites https://v2.tauri.app/start/prerequisites/)

- Live testing with tchap-web (symbolic link doesnt work)

- Go to the frontend folder

```
yarn install
yarn start
```

- Go back to your tauri app and run 

```
cargo tauri dev

```

- It will automatically use your frontend configure on the url in the `tauri.conf.json` file. So modify the `build: devUrl` to match your dev frontend


## Dev using a specific web archive version

- you can change in `package.json` the version of tchap-web that you want to use in `tchapConfig.tchap-web_version` and `tchapConfig.tchap-web_version`. 

- Then you can run

```
npm install

npm run fetch-package

cargo tauri dev

```

## Dev using a github branch reoo

- In `package.json` you need to put  `tchapConfig.tchap-web_github.use_github : true` and complete `tchapConfig.tchap-web_github.branch` and `tchapConfig.tchap-web_github.repo` 

- You can also select the environment in `tchapConfig.tchap-web_github.env` that you want to use. This will determine which config.json to select for the build. If no value is set (prod, preprod or dev), it will use prod as default.

- Then you can run the same step as the dev using a specific web archive version


## Build
```
npm install

npm run fetch-package

cargo tauri build

```

### Github workflow
- Publish : This workflow will be trigger when a new tag is created.

- Test on build: This will only test the build of the app. From the build of the webapp to the build of the tauri app.

- TODO : Run tests

## Create a release 
- Create a new branch from `master`.
- The release version is taken from the tauri-conf file. So before creating a new tag for release, you need to run the `update-version.sh` script with the new version. This script will update the version in the tauri-conf file, the package.json and the updater plugin in the tauri-conf file.
- The updater url is generated with the new version and will be used to auto update the app. A PR will need to be created to update the url of the conf file served by the nginx backend.
- Check carefully which archive of the frontend webapp you want to use inside the `package.json` file. It can be a specific version, a specific branch or the latest version on the `master` branch.
- Commit, push your branch and create a PR.
- Once the PR is merged, you can create the release tag.
- The tag need to follow the format `tchap-*` and be identical to the version in the `package.json` file.
- The workflow will be triggered and will build the app, create a release on github and upload the app to the release.
- The workflow will also update the `tchap-web` archive used in the app to the latest version on the `master` branch of the `tchap-web` repository.