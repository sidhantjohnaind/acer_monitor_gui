#!/usr/bin/env bash
# Script to build standalone Linux .AppImage package for Acer Monitor GUI
set -e

echo "📦 Building release binary..."
cargo build --release

APPDIR="AppDir"
rm -rf "$APPDIR"
mkdir -p "$APPDIR/usr/bin"
mkdir -p "$APPDIR/usr/share/icons/hicolor/128x128/apps"

echo "📂 Structuring AppDir..."
cp target/release/acer_monitor_gui "$APPDIR/usr/bin/acer_monitor_gui"

# Create Desktop Entry
cat <<EOF > "$APPDIR/acer-monitor-gui.desktop"
[Desktop Entry]
Name=Acer Monitor GUI
Comment=Native Hardware OSD Control Suite for Acer Monitors
Exec=acer_monitor_gui
Icon=acer_monitor_gui
Terminal=false
Type=Application
Categories=Settings;HardwareSettings;System;
EOF

# Create AppRun entrypoint script
cat <<'EOF' > "$APPDIR/AppRun"
#!/bin/bash
HERE="$(dirname "$(readlink -f "${0}")")"
export PATH="${HERE}/usr/bin:${PATH}"
export LD_LIBRARY_PATH="${HERE}/usr/lib:${LD_LIBRARY_PATH}"
exec "${HERE}/usr/bin/acer_monitor_gui" "$@"
EOF
chmod +x "$APPDIR/AppRun"

# Create a default display icon
curl -sL https://raw.githubusercontent.com/google/material-design-icons/master/png/hardware/desktop_windows/materialicons/48dp/2x/baseline_desktop_windows_black_48dp.png -o "$APPDIR/acer_monitor_gui.png" || touch "$APPDIR/acer_monitor_gui.png"

echo "⚡ Generating Acer_Monitor_GUI-x86_64.AppImage..."
ARCH=x86_64 /tmp/appimagetool-bin/AppRun "$APPDIR" "Acer_Monitor_GUI-x86_64.AppImage"

echo "✅ AppImage created successfully: Acer_Monitor_GUI-x86_64.AppImage"
