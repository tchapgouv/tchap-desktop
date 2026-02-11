{ lib
, stdenv
, rustPlatform
, fetchFromGitHub
, fetchurl
, cargo-tauri
, nodejs
, fetchNpmDeps
, libayatana-appindicator
, pkg-config
, npmHooks
, makeBinaryWrapper
, jq
, moreutils
, openssl
, sqlcipher
, webkitgtk_4_1
, gtk3
, cairo
, gdk-pixbuf
, glib
, dbus
, librsvg
, gst_all_1
, libsoup_3
, wrapGAppsHook3
, desktop-file-utils
, glib-networking
}:

let
  metadata = lib.importJSON ../package.json;
  tchapWebMeta = 
  let
    config = metadata.tchapConfig.prod;
  in {
    version = config.tchap-web_version;
    archiveName = config.tchap-web_archive_name;
    repo = config.tchap-web_github.repo;
  };
  
  # Télécharger l'archive tchap-web de manière pure
  tchapWebArchive = fetchurl {
    url = "${tchapWebMeta.repo}/releases/download/tchap-${tchapWebMeta.version}/${tchapWebMeta.archiveName}";
    hash = "sha256-rp1CoJyJC0faSe/QkfxGHeFGMP8GO0pGIs1cdtEUPaE="; # À remplir après la première tentative
  };
  
in
rustPlatform.buildRustPackage rec {
  pname = "tchap-desktop";
  inherit (metadata) version;

  src = fetchFromGitHub {
    owner = "tchapgouv";
    repo = "tchap-desktop";
    rev = "tchap-${version}";
    hash = "sha256-iroiJvQ8r8Dd+DQrLYeXRMHeEYZaQu8dIH0zoh1YqDE="; # À remplir avec: nix-prefetch-url --unpack https://github.com/tchapgouv/tchap-desktop/archive/refs/tags/tchap-${version}.tar.gz
  };

  # Dépendances npm (à la racine du projet)
  npmDeps = fetchNpmDeps {
    src = "${src}";
    hash = "sha256-4wLw6jFEny2OPtGqf9UEkEy29LwOczSHusiUgQSoaKA="; # À remplir après la première tentative de build
  };

  # Le code Rust est dans src-tauri
  cargoRoot = "src-tauri";
  buildAndTestSubdir = "src-tauri";

  cargoHash = "sha256-alf+8byB1HoZNQf9tMMRSNsX5JxlR/uVm8bgbjOxYnc="; # À remplir avec le hash après la première tentative de build

  # Phase de préparation
  postPatch = ''
    # Placer l'archive tchap-web pré-téléchargée au bon endroit
    mkdir -p archives
    cp ${tchapWebArchive} archives/${tchapWebMeta.archiveName}
    jq '
      .bundle.createUpdaterArtifacts = false |
      .plugins.updater = {"active": false, "pubkey": "", "endpoints": []}
    ' \
    src-tauri/tauri.conf.json | sponge src-tauri/tauri.conf.json
  '' 
  # NOTE: libayatana should use pkg-config…
    + lib.optionalString stdenv.hostPlatform.isLinux ''
    substituteInPlace $cargoDepsCopy/libappindicator-sys-*/src/lib.rs \
      --replace "libayatana-appindicator3.so.1" "${libayatana-appindicator}/lib/libayatana-appindicator3.so.1" \
  '';

  preConfigure = ''
    # Les dépendances npm sont déjà installées par npmConfigHook
    
    # "Télécharger" l'archive (en fait déjà présente)
    npm run fetch-package -- prod || true
    
    # Si le script a échoué, extraire manuellement l'archive
    if [ ! -d "src" ]; then
      mkdir -p src
      tar -xzf archives/${tchapWebMeta.archiveName} -C src --strip-components=1
    fi
  '';

  nativeBuildInputs = [
    cargo-tauri.hook
    pkg-config
    nodejs
    npmHooks.npmConfigHook
    makeBinaryWrapper
    jq
    moreutils
  ];

  buildInputs = [
    openssl
    sqlcipher
    webkitgtk_4_1
    gtk3
    cairo
    gdk-pixbuf
    glib
    dbus
    librsvg
    libsoup_3
    gst_all_1.gstreamer
    gst_all_1.gst-plugins-base
    gst_all_1.gst-plugins-good
    gst_all_1.gst-plugins-bad
  ] ++ lib.optionals stdenv.hostPlatform.isLinux
    [
      libayatana-appindicator
      wrapGAppsHook3 desktop-file-utils
    ];

  # Nécessaire pour tauri-plugin-deep-link.
  # GIO_EXTRA_MODULES est nécessaire pour avoir le support TLS.
  # https://github.com/tauri-apps/tauri/issues/11647
  preFixup = ''
    gappsWrapperArgs+=(
      --prefix PATH : "${lib.makeBinPath [ desktop-file-utils ]}"
      --prefix GIO_EXTRA_MODULES : ${glib-networking}/lib/gio/modules
    )
  '';

  # Variables d'environnement
  env = {
    # Pour sqlcipher
    SQLCIPHER_LIB_DIR = "${sqlcipher}/lib";
    SQLCIPHER_INCLUDE_DIR = "${sqlcipher}/include";
  } // lib.optionalAttrs stdenv.hostPlatform.isDarwin {
    LIBRARY_PATH = "${sqlcipher}/lib";
    C_INCLUDE_PATH = "${sqlcipher}/include";
  };

  # Désactiver les tests (pas de tests dans ce projet)
  doCheck = false;

  # Phase d'installation
  postInstall = lib.optionalString stdenv.hostPlatform.isLinux ''
    # Les icônes sont dans src-tauri/icons depuis la racine
    install -Dm644 src-tauri/icons/icon.png $out/share/icons/hicolor/128x128/apps/tchap.png
    
    # Créer le fichier .desktop
    mkdir -p $out/share/applications
    cat > $out/share/applications/tchap.desktop <<EOF
    [Desktop Entry]
    Name=Tchap
    Comment=Messagerie instantanée du secteur public français
    Exec=$out/bin/tchap-desktop
    Icon=tchap
    Type=Application
    Categories=Network;InstantMessaging;
    MimeType=x-scheme-handler/tchap;
    EOF
  '';

  meta = with lib; {
    description = "Application de bureau Tchap - Messagerie du secteur public français";
    homepage = "https://github.com/tchapgouv/tchap-desktop";
    license = licenses.asl20;
    platforms = platforms.linux ++ platforms.darwin;
    mainProgram = "tchap-desktop";
  };
}
