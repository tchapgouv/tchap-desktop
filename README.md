
# Tchap Desktop

Client Desktop Tchap avec Tauri ( https://tauri.app )

## Dev

- Install prerequisites https://v2.tauri.app/start/prerequisites/)

- Live testing with tchap-web
- Create a symbolic link to tchap-web-v4 webapp folder into 
```
ln -s $PATH_TO_TCHAP_WEB_WEBAPP/* ./src/
```
- if the webapp folder doesnt exist you need to run first in tchap-web-v4
```
CONFIG=dev ./scripts/tchap/package-tchap.sh
```

- Then come back to tchap-desktop
```
npm install

npm run fetch-package

cargo tauri dev

```

- you can change in `package.json` the version of tchap-web that you want to use in `tchapConfig.tchap-web_version` and `tchapConfig.tchap-web_version`. Make the change before running `fetch-package` script

Then you will be able to build your tchap webb and have the modification directly here


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