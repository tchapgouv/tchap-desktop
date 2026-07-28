{
  lib,
  stdenv,
  fetchzip,
  element-desktop,
  makeDesktopItem,
}:

let
  tchapVersion = "4.21.4";
  webArchive = "tchap-4.21.4-prod-20260825.tar.gz";
  variant = builtins.fromJSON (builtins.readFile ./variant.json);

  commandLineArgs =
    "--no-update" + lib.optionalString stdenv.hostPlatform.isLinux " --password-store=gnome-libsecret";

  tchap-web = fetchzip {
    name = "tchap-web-${tchapVersion}";
    url = "https://github.com/tchapgouv/tchap-web-v4/releases/download/tchap-${tchapVersion}/${webArchive}";
    hash = "sha256-ZEKWdNKupWu3q8VwSxVbgQhJYURTG5ha54e8qL/d0mE=";
  };

  element-desktop-tchap = element-desktop.override {
    element-web = tchap-web;
    inherit commandLineArgs;
  };
in
element-desktop-tchap.overrideAttrs (oldAttrs: {
  pname = "tchap-desktop";
  name = "tchap-desktop-${tchapVersion}";

  env = (oldAttrs.env or { }) // {
    VARIANT_PATH = "${./variant.json}";
  };

  postPatch =
    (oldAttrs.postPatch or "")
    + lib.optionalString stdenv.hostPlatform.isLinux ''
      cp ${tchap-web}/vector-icons/512.png apps/desktop/build/icon.png
    ''
    + lib.optionalString stdenv.hostPlatform.isDarwin ''
      cp ${tchap-web}/vector-icons/1024.png \
        apps/desktop/build/icon.icon/Assets/element.png
    '';

  installPhase =
    if stdenv.hostPlatform.isDarwin then
      ''
        runHook preInstall

        mkdir -p "$out/Applications" "$out/bin"
        mv dist/mac*/${lib.escapeShellArg "${variant.productName}.app"} "$out/Applications"

        app="$out/Applications/${variant.productName}.app"
        executable="$app/Contents/MacOS/${variant.productName}"

        ln -s ${tchap-web} "$app/Contents/Resources/webapp"

        wrapProgram "$executable" \
          --add-flags ${lib.escapeShellArg commandLineArgs}
        makeWrapper "$executable" "$out/bin/${variant.name}"

        runHook postInstall
      ''
    else
      oldAttrs.installPhase;

  postInstall =
    (oldAttrs.postInstall or "")
    + lib.optionalString stdenv.hostPlatform.isLinux ''
      mv "$out/bin/element-desktop" "$out/bin/tchap-desktop"

      rm "$out/share/icons/hicolor/512x512/apps/element.png"
      install -Dm644 ${tchap-web}/vector-icons/512.png \
        "$out/share/element/build/icon.png"
      ln -s "$out/share/element/build/icon.png" \
        "$out/share/icons/hicolor/512x512/apps/tchap.png"
    '';

  desktopItems = lib.optionals stdenv.hostPlatform.isLinux [
    (makeDesktopItem {
      name = "tchap-desktop";
      exec = "tchap-desktop %u";
      icon = "tchap";
      desktopName = "Tchap";
      genericName = "Messagerie instantanée";
      comment = "Messagerie instantanée du secteur public français";
      categories = [
        "Network"
        "InstantMessaging"
        "Chat"
      ];
      startupWMClass = "Tchap";
      mimeTypes = [ "x-scheme-handler/tchap" ];
    })
  ];

  passthru = (oldAttrs.passthru or { }) // {
    inherit tchap-web tchapVersion;
    elementDesktopVersion = element-desktop.version;
  };

  meta = oldAttrs.meta // {
    description = "Tchap web client packaged with the Element Desktop Electron shell";
    homepage = "https://github.com/tchapgouv/tchap-desktop";
    license = [
      lib.licenses.agpl3Plus
      lib.licenses.gpl3Plus
    ];
    platforms = element-desktop.meta.platforms;
    mainProgram = "tchap-desktop";
  };
})
