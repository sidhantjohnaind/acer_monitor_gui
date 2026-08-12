# Acer Monitor GUI Desktop Clean Uninstaller for Windows PowerShell
Write-Host "🗑️ Uninstalling Acer Monitor GUI Desktop Application..." -ForegroundColor Cyan

# 1. Stop running GUI instances
Stop-Process -Name "acer_monitor_gui" -ErrorAction SilentlyContinue

# 2. Remove installation folder
$targetDir = "$env:LOCALAPPDATA\Programs\AcerMonitorGUI"
if (Test-Path $targetDir) {
    Remove-Item -Path $targetDir -Recurse -Force
    Write-Host "  • Removed $targetDir" -ForegroundColor Yellow
}

# 3. Remove Start Menu Shortcut
$shortcutPath = "$env:APPDATA\Microsoft\Windows\Start Menu\Programs\Acer Monitor GUI.lnk"
if (Test-Path $shortcutPath) {
    Remove-Item -Path $shortcutPath -Force
    Write-Host "  • Removed Start Menu Shortcut" -ForegroundColor Yellow
}

Write-Host "✅ Acer Monitor GUI Desktop Application has been completely uninstalled!" -ForegroundColor Green
