#!/usr/bin/env bash
# Acer Monitor GUI Clean Uninstaller (Linux/macOS)
set -e

echo "🗑️ Uninstalling Acer Monitor GUI Desktop Application..."

# 1. Stop running processes
echo "  • Closing running GUI instances..."
pkill -f "acer_monitor_gui" 2>/dev/null || true

# 2. Remove binary executables and symlinks
echo "  • Removing executable binaries..."
sudo rm -f /usr/local/bin/acer_gui /usr/local/bin/acer_monitor_gui 2>/dev/null || true
rm -f "$HOME/.local/bin/acer_gui" "$HOME/.local/bin/acer_monitor_gui" 2>/dev/null || true

# 3. Remove Desktop Launcher Entry
echo "  • Removing desktop application launcher..."
rm -f "$HOME/.local/share/applications/acer-monitor-gui.desktop"

# 4. Update desktop database
if command -v update-desktop-database &> /dev/null; then
    update-desktop-database "$HOME/.local/share/applications" 2>/dev/null || true
fi

echo "✅ Acer Monitor GUI Desktop Application has been completely uninstalled!"
