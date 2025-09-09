
<p align="center">
  <a href="https://github.com/tchapgouv">
    <img alt="tchap-logo" src="./src-tauri/icons/128x128@2x.png" width="300" />
  </a>
</p>

<p align="center">
  Bienvenue sur Tchap! Le système de messagerie instantanée du secteur public français
</p>

<p align="center">
  <a href="https://tchap.numerique.gouv.fr">
    Site web de présentation
  </a> - 
  <a href="mailto:contact@tchap.beta.gouv.fr">
    Contactez-nous
  </a>
</p>

Tchap desktop is the desktop app of [tchap web](https://github.com/tchapgouv/tchap-web-v4) which allows you to chat through the matrix protocol for the French public service.

# Tchap Desktop

Desktop client for Tchap web build on top of Tauri ( https://tauri.app ). Code is inspired by [element desktop ](https://github.com/element-hq/element-desktop) which used electron as his backend.

Tchap-desktop is strongly dependant of tchap-web since it is only the desktop wrapper around tchap-web.
A `TauriPlatform` has been created in tchap-web (which is a soft fork of [element-web](https://github.com/element-hq/element-web)) in order to support Tauri as his backend pltform.

## Prerequisites

- Install prerequisites https://v2.tauri.app/start/prerequisites/)

- Install sqlcipher for your platform, on Macos : brew install sqlcipher

- Install Node and npm using a version manager like nvm. The frontend required a node version = 20.

- If you want to build the project locally, on windows platform you may  need to manually put the sqlcipher dll in the `src-tauri/` folder. See more about the install workflow in the github workflow section. (`.github/workflows`)

## Dev local using local frontend

- Live testing with tchap-web (symbolic link doesnt work)

- Go to your frontend folder

```
# On tchap only this branch is compatible with tauri otherwise the tauri platform wont be detected
git checkout desktop-search-ipc

yarn install

yarn start
```

| For now only the branch "desktop-search-ipc" on the frontend repository support tauri as his backend.

- Go back to your tauri app and run 

```
cargo tauri dev

```

- It will automatically use your frontend configure on the url in the `tauri.conf.json` file. So modify the `build: devUrl` to match your dev frontend


## Dev using a specific web archive version
- Tchap web is compatible with Tauri plaftorm from the version 4.15.2 and above.

- You need to remove `build: devUrl` from the `tauri.conf.json` file. Otherwise it will wait for a local frontend to be running.

- you can change in `package.json` the version of tchap-web that you want to use in `tchapConfig.tchap-web_version` and `tchapConfig.tchap-web_version`. 

- Then you can run

```
npm install

npm run fetch-package -- dev

cargo install tauri-cli --version "^2.0.0" --locked

# You can remove "devUrl": "http://localhost:8080" in `tauri.conf.json` to use tchap-web in tchap-desktop/src

cargo tauri dev

```

## Dev using a github branch from a remote repository
- You need to remove `build: devUrl` from the `tauri.conf.json` file. Otherwise it will wait for a local frontend to be running.

- In `package.json` you need to put  `tchapConfig.tchap-web_github.use_github : true` and complete `tchapConfig.tchap-web_github.branch` and `tchapConfig.tchap-web_github.repo` 

- You can also select the environment in `tchapConfig.tchap-web_github.env` that you want to use. This will determine which config.json to select for the build. If no value is set (prod, preprod or dev), it will use prod as default.

- Then you can run the same step as the dev using a specific web archive version


## Build
```
npm install

npm run fetch-package -- prod

cargo tauri build

```



### Github workflow
- Publish : This workflow will be trigger when a new tag is created.

- Test on build: This will only test the build of the app. From the build of the webapp to the build of the tauri app.

- To build tauri we use the `build-tauri.yml` workflow. This workflow will build the app for windows platform. It will use the `tauri.conf.prod.json` file that we pass as an argument to the workflow in order to use the correct sqlcipher dll.

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

## Updater
To update clients automatically, we use the updater plugin.


## Troubleshoot

### 'sqlcipher' not found on MacOs
If you have the following error 
```
error: linking with `cc` failed: exit status: 1
..
ld: library 'sqlcipher' not found
```

You can export LIBRARY_PATH and C_INCLUDE_PATH to point to your sqlcipher installation
```
export LIBRARY_PATH=/opt/homebrew/Cellar/sqlcipher/4.6.1/lib
export C_INCLUDE_PATH=/opt/homebrew/Cellar/sqlcipher/4.6.1/include
```


### compile sqlcipher on windows

Be sure to install all Microsoft C++ Build Tools dependencies listed here : https://tauri.app/start/prerequisites/#microsoft-c-build-tools


Sqlcipher is installed with vcpkg tools and copied to `src-tauri` folder

```
git clone https://github.com/microsoft/vcpkg.git
cd vcpkg
set "VCPKG_ROOT=%cd%"
.\bootstrap-vcpkg.bat
.\vcpkg integrate install
.\vcpkg install openssl:x64-windows-static
.\vcpkg.exe install sqlcipher:x64-windows-static #not sure if needed
.\vcpkg.exe install sqlcipher:x64-windows

echo %VCPKG_ROOT%

cd ..\tchap-desktop
xcopy /y %VCPKG_ROOT%\installed\x64-windows\bin\sqlcipher.dll .\src-tauri\
xcopy /y %VCPKG_ROOT%\installed\x64-windows\bin\libcrypto-3-x64.dll .\src-tauri\
xcopy /y %VCPKG_ROOT%\installed\x64-windows\bin\libssl-3-x64.dll .\src-tauri\
xcopy /y %VCPKG_ROOT%\installed\x64-windows\lib\sqlcipher.lib .\src-tauri\
```
