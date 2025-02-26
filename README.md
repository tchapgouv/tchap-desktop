
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