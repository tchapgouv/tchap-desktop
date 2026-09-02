{
  lib,
  stdenv,
  runCommand,
  dbus,
  desktop-file-utils,
  xvfb-run,
  package,
}:

runCommand "tchap-desktop-smoke-test"
  {
    nativeBuildInputs = lib.optionals stdenv.hostPlatform.isLinux [
      dbus
      desktop-file-utils
      xvfb-run
    ];
  }
  (
    if stdenv.hostPlatform.isLinux then
      ''
        test -x ${package}/bin/tchap-desktop
        test -e ${package}/share/icons/hicolor/512x512/apps/tchap.png
        test -L ${package}/share/element/webapp

        desktopFile=${package}/share/applications/tchap-desktop.desktop
        desktop-file-validate "$desktopFile"
        grep -Fq 'Exec=tchap-desktop %u' "$desktopFile"
        grep -Fq 'MimeType=x-scheme-handler/tchap' "$desktopFile"

        wrapper=${package}/bin/tchap-desktop
        grep -Fq -- '--no-update' "$wrapper"
        grep -Fq -- '--password-store=gnome-libsecret' "$wrapper"

        export HOME="$TMPDIR/home"
        export XDG_CACHE_HOME="$TMPDIR/cache"
        export XDG_CONFIG_HOME="$TMPDIR/config"
        export XDG_DATA_HOME="$TMPDIR/data"
        export XDG_RUNTIME_DIR="$TMPDIR/runtime"
        mkdir -p "$HOME" "$XDG_CACHE_HOME" "$XDG_CONFIG_HOME" "$XDG_DATA_HOME" "$XDG_RUNTIME_DIR"
        chmod 700 "$XDG_RUNTIME_DIR"

        dbus-daemon \
          --config-file=${dbus}/share/dbus-1/session.conf \
          --print-address=3 \
          --print-pid=4 \
          --fork \
          3>dbus-address \
          4>dbus-pid
        export DBUS_SESSION_BUS_ADDRESS="$(cat dbus-address)"
        dbusPid="$(cat dbus-pid)"
        trap 'kill "$dbusPid" 2>/dev/null || true' EXIT

        set +e
        timeout 15s xvfb-run -a \
          "$wrapper" --no-sandbox --disable-gpu --disable-dev-shm-usage \
          >startup.log 2>&1
        status=$?
        set -e

        kill "$dbusPid"
        trap - EXIT

        if [ "$status" -ne 124 ]; then
          cat startup.log
          echo "Tchap Desktop exited during startup with status $status" >&2
          exit 1
        fi

        touch "$out"
      ''
    else
      ''
        app=${package}/Applications/Tchap.app

        test -x ${package}/bin/tchap-desktop
        test -x "$app/Contents/MacOS/Tchap"
        test -L "$app/Contents/Resources/webapp"

        touch "$out"
      ''
  )
