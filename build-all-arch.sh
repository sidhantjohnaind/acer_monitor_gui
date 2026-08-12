#!/usr/bin/env bash
# Multi-Architecture & Multi-OS Master Release Builder for Acer Monitor GUI
set -e

mkdir -p dist

echo "🚀 Building [1/4] AMD64 Linux Native Binary & AppImage..."
cargo build --release --target x86_64-unknown-linux-gnu
cp -f target/x86_64-unknown-linux-gnu/release/acer_monitor_gui dist/acer_monitor_gui-amd64-linux
chmod +x build-appimage.sh && ./build-appimage.sh
cp -f Acer_Monitor_GUI-x86_64.AppImage dist/Acer_Monitor_GUI-amd64.AppImage

echo "🚀 Building [2/4] Windows 11 / 10 AMD64 Native EXE..."
cargo build --release --target x86_64-pc-windows-gnu
cp -f target/x86_64-pc-windows-gnu/release/acer_monitor_gui.exe dist/acer_monitor_gui-amd64-win11.exe

if command -v aarch64-linux-gnu-gcc &> /dev/null; then
    echo "🚀 Building [3/4] ARM64 Linux (Raspberry Pi 4/5 & ARM64 Servers)..."
    CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc \
    cargo build --release --target aarch64-unknown-linux-gnu
    cp -f target/aarch64-unknown-linux-gnu/release/acer_monitor_gui dist/acer_monitor_gui-arm64-linux
fi

if command -v riscv64-linux-gnu-gcc &> /dev/null; then
    echo "🚀 Building [4/4] RISC-V 64 Linux..."
    CARGO_TARGET_RISCV64GC_UNKNOWN_LINUX_GNU_LINKER=riscv64-linux-gnu-gcc \
    cargo build --release --target riscv64gc-unknown-linux-gnu
    cp -f target/riscv64gc-unknown-linux-gnu/release/acer_monitor_gui dist/acer_monitor_gui-riscv64-linux
fi

echo "✅ All multi-architecture binaries packaged in dist/:"
ls -lh dist/
