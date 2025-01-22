
# Tchap Desktop

Client Desktop Tchap avec Tauri ( https://tauri.app )

## Dev

- Télécharger une release de Tchap Web et placer le contenu de dist dans src
```
curl https://github.com/tchapgouv/tchap-web-v4/releases/download/tchap-4.13.0/tchap-4.13.0-prod-20250115.tar.gz
tar -xvf tchap-4.13.0/tchap-4.13.0-prod-20250115.tar.gz
mv dist src
```

- Tester localement (installer les prérequis au préalable https://v2.tauri.app/start/prerequisites/)
```
cargo tauri dev
```

- Compiler tchap pour votre système
```
cargo tauri build
```


