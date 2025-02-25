
# Tchap Desktop

Client Desktop Tchap avec Tauri ( https://tauri.app )

## Dev local

- Install prerequisites https://v2.tauri.app/start/prerequisites/)

- Live testing with tchap-web (symbolic link doesnt work)

- Go to the frontend folder and find  the webapp folder

- If it doesnt exist you need to run first 
```
CONFIG=dev ./scripts/tchap/package-tchap.sh
```

- copy the generated content into `./src`

- Then come back to tchap-desktop

```
cargo tauri dev

```
## Dev using a specific web archive version

- you can change in `package.json` the version of tchap-web that you want to use in `tchapConfig.tchap-web_version` and `tchapConfig.tchap-web_version`. 

- Then you can run

```
npm install

npm run fetch-package

cargo tauri dev

```


## Build
```
npm install

npm run fetch-package

cargo tauri build

```

### Github workflow
- 

## Release version
- The release version is taken from the tauri-conf file. So before creating a new tag for release, you have to update this version
- The tag need to follow the format `tchap-*`