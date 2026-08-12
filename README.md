# Acer Monitor Control Native GUI (`acer_monitor_gui`) 🖥️⚡

[![Rust](https://img.shields.io/badge/Rust-2021-orange.svg)](https://www.rust-lang.org/)
[![GUI](https://img.shields.io/badge/GUI-egui%20%7C%20eframe-purple.svg)](https://github.com/emilk/egui)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20Windows-blue.svg)]()

> A 100% native Rust, zero-dependency desktop GUI application for controlling Acer (and generic VESA MCCS) monitors via DDC/CI on Linux and Windows. Built using `egui` and `eframe` for hardware-accelerated, sub-millisecond responsiveness without any webview, electron, or JavaScript overhead.

---

## ✨ Features

* **🖥️ Native Hardware Sliders**: Adjust Brightness (0-100%) and Contrast (0-100%) with live percentage readouts and quick preset pills (`100%`, `80%`, `50%`, `20%`).
* **🔊 Audio Volume & Mute**: Direct volume slider with instant Mute toggle button.
* **🎛️ One-Touch Hardware OSD Mode Cards**: Apply native monitor presets (*Standard*, *ECO Saver*, *HDR Game Mode*, *Action*, *Racing*, *Sports*, *Reading*, *Movie*, *User*).
* **🔌 Active Input Signal Switcher**: One-click switching between *DisplayPort*, *HDMI 1*, *HDMI 2*, and *Auto Switch*.
* **🎮 Gaming & Vision Hardware Tuning**:
  - **Black Boost Level Slider**: Fine-tune shadow visibility in dark gaming scenes (0-10).
  - **Blue Light Filter Selectors**: Quick eye-care presets (Off, 50%, 60%, 70%, 80%).
  - **Hardware OverDrive Switch**: Select Off, Normal, or Extreme (2).
  - **AimPoint Crosshair Overlay**: Cycle native monitor crosshair overlays.
* **💡 Real-Time Power & Energy Meter**: Displays live estimated wattage draw (~15.2W) and annual electricity cost (~$6.68/yr).
* **🎨 Interactive Diagnostic Test Pattern Canvas**: Built-in canvas rendering alignment grid, step gradients, and pure RGB test colors for display panel testing.
* **⚙️ Multi-Monitor Support**: Switch between connected displays (`Monitor 0`, `Monitor 1`, `All Monitors`).
* **🚀 100% Pure Native Rust**: Compiled directly to a single native executable (~5 MB) with zero Electron or Web View runtime footprint.

---

## 📦 Installation & Running

### Linux

Ensure `i2c-dev` kernel module is loaded:
```bash
sudo modprobe i2c-dev
```

Run native GUI app:
```bash
cargo run --release
```

### Windows

Run native GUI app in PowerShell:
```powershell
cargo run --release
```

### Cross-Compiling

```bash
# Cross-compile Windows executable on Linux
cargo build --release --target x86_64-pc-windows-gnu

# Linux ARM64 / Raspberry Pi
cargo build --release --target aarch64-unknown-linux-gnu
```

---

## 📄 License

Licensed under the [MIT License](LICENSE).
