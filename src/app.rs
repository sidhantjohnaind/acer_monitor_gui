use crate::{energy, monitor::MonitorSet};
use eframe::egui;
use std::sync::mpsc::{channel, Sender};
use std::thread;

pub enum DdcCommand {
    SetBrightness(usize, u32),
    SetContrast(usize, u32),
    SetVolume(usize, u32),
    SetMute(usize, bool),
    SetPreset(usize, u8),
    SetInput(usize, u8),
    SetBlackBoost(usize, u32),
    SetBlueLight(usize, u32),
    SetOverDrive(usize, u32),
    UnlockOSD(usize),
    PowerOff(usize),
}

pub struct AcerMonitorApp {
    brightness: u32,
    contrast: u32,
    volume: u32,
    is_muted: bool,
    black_boost: u32,
    blue_light: u32,
    overdrive: u32,
    selected_target: usize, // 0 = Mon 0, 1 = Mon 1, 99 = All
    active_preset: String,
    active_input: String,
    status_msg: String,
    active_pattern: Option<&'static str>,
    cmd_tx: Sender<DdcCommand>,
}

impl Default for AcerMonitorApp {
    fn default() -> Self {
        let (tx, rx) = channel::<DdcCommand>();

        // Spawn background DDC worker thread to eliminate UI choppiness
        thread::spawn(move || {
            let mut monitor_set: Option<MonitorSet> = None;

            while let Ok(cmd) = rx.recv() {
                if monitor_set.is_none() {
                    monitor_set = MonitorSet::enumerate().ok();
                }

                if let Some(set) = monitor_set.as_mut() {
                    match cmd {
                        DdcCommand::SetBrightness(target, val) => {
                            let _ = exec_on_set(set, target, |m| crate::acer::brightness(m, val));
                        }
                        DdcCommand::SetContrast(target, val) => {
                            let _ = exec_on_set(set, target, |m| crate::acer::contrast(m, val));
                        }
                        DdcCommand::SetVolume(target, val) => {
                            let _ = exec_on_set(set, target, |m| crate::acer::volume(m, val));
                        }
                        DdcCommand::SetMute(target, val) => {
                            let _ = exec_on_set(set, target, |m| crate::acer::mute(m, val));
                        }
                        DdcCommand::SetPreset(target, val) => {
                            let _ = exec_on_set(set, target, |m| crate::acer::display_mode(m, val.into()));
                        }
                        DdcCommand::SetInput(target, val) => {
                            let _ = exec_on_set(set, target, |m| crate::acer::input(m, val.into()));
                        }

                        DdcCommand::SetBlackBoost(target, val) => {
                            let _ = exec_on_set(set, target, |m| crate::acer::black_boost(m, val));
                        }
                        DdcCommand::SetBlueLight(target, val) => {
                            let _ = exec_on_set(set, target, |m| crate::acer::blue_light(m, val));
                        }
                        DdcCommand::SetOverDrive(target, val) => {
                            let _ = exec_on_set(set, target, |m| crate::acer::overdrive(m, val));
                        }
                        DdcCommand::UnlockOSD(target) => {
                            let _ = exec_on_set(set, target, |m| {
                                let _ = crate::acer::key_lock(m, false);
                                crate::acer::power_key(m, false)
                            });
                        }
                        DdcCommand::PowerOff(target) => {
                            let _ = exec_on_set(set, target, |m| crate::acer::power_mode(m, false));
                        }
                    }
                }
            }
        });

        Self {
            brightness: 80,
            contrast: 50,
            volume: 50,
            is_muted: false,
            black_boost: 5,
            blue_light: 0,
            overdrive: 1,
            selected_target: 0,
            active_preset: "Standard".to_string(),
            active_input: "DisplayPort".to_string(),
            status_msg: "60 FPS Native UI (Async I2C Enabled)".to_string(),
            active_pattern: None,
            cmd_tx: tx,
        }
    }
}

fn exec_on_set<F>(set: &mut MonitorSet, target: usize, mut f: F) -> Result<(), String>
where
    F: FnMut(&mut crate::monitor::Monitor) -> Result<(), String>,
{
    if target == 99 {
        for mon in set.monitors_mut() {
            let _ = f(mon);
        }
        Ok(())
    } else {
        let spec_str = target.to_string();
        if let Ok(mon) = set.pick_mut_by_specifier(Some(&spec_str)) {
            let _ = f(mon);
        }
        Ok(())
    }
}

impl eframe::App for AcerMonitorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Dark Glassmorphic Visual Theme
        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill = egui::Color32::from_rgb(11, 15, 25);
        visuals.window_fill = egui::Color32::from_rgb(20, 26, 42);
        visuals.widgets.active.bg_fill = egui::Color32::from_rgb(0, 229, 255);
        visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(124, 77, 255);
        visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(15, 20, 32);
        ctx.set_visuals(visuals);

        // Top Navigation Header
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.heading(egui::RichText::new("🖥️ Acer Monitor Control").strong().size(18.0).color(egui::Color32::from_rgb(0, 229, 255)));
                ui.label(egui::RichText::new("Native Hardware GUI").small().color(egui::Color32::GRAY));

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("🔓 Unlock OSD").clicked() {
                        self.status_msg = "Sent OSD Key Unlock".to_string();
                        let _ = self.cmd_tx.send(DdcCommand::UnlockOSD(self.selected_target));
                    }
                    if ui.button("⚡ Power Off").clicked() {
                        self.status_msg = "Sent Power Off Command".to_string();
                        let _ = self.cmd_tx.send(DdcCommand::PowerOff(self.selected_target));
                    }
                    ui.label(egui::RichText::new(&self.status_msg).small().color(egui::Color32::from_rgb(0, 230, 118)));
                });
            });
            ui.add_space(8.0);
        });

        // Main Central Dashboard Panel
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add_space(10.0);

                // Target Display Selection Tabs
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Target Display:").strong());
                    if ui.selectable_label(self.selected_target == 0, "Monitor 0 (VG271U)").clicked() {
                        self.selected_target = 0;
                    }
                    if ui.selectable_label(self.selected_target == 1, "Monitor 1 (Secondary)").clicked() {
                        self.selected_target = 1;
                    }
                    if ui.selectable_label(self.selected_target == 99, "⚡ All Monitors").clicked() {
                        self.selected_target = 99;
                    }
                });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                // Two Column Grid Dashboard
                ui.columns(2, |cols| {
                    // Left Column: Brightness, Contrast, Volume, Energy
                    cols[0].group(|ui| {
                        ui.heading("☀️ Brightness & Display Controls");
                        ui.add_space(6.0);

                        // Brightness Slider (Instant Smooth UI Update)
                        ui.horizontal(|ui| {
                            ui.label("Brightness:");
                            let slider = ui.add(egui::Slider::new(&mut self.brightness, 0..=100).suffix("%"));
                            if slider.changed() {
                                let _ = self.cmd_tx.send(DdcCommand::SetBrightness(self.selected_target, self.brightness));
                            }
                        });

                        // Quick Brightness Presets
                        ui.horizontal(|ui| {
                            if ui.button("☀️ 100%").clicked() {
                                self.brightness = 100;
                                let _ = self.cmd_tx.send(DdcCommand::SetBrightness(self.selected_target, 100));
                            }
                            if ui.button("⚖️ 80%").clicked() {
                                self.brightness = 80;
                                let _ = self.cmd_tx.send(DdcCommand::SetBrightness(self.selected_target, 80));
                            }
                            if ui.button("🔉 50%").clicked() {
                                self.brightness = 50;
                                let _ = self.cmd_tx.send(DdcCommand::SetBrightness(self.selected_target, 50));
                            }
                            if ui.button("🌙 20%").clicked() {
                                self.brightness = 20;
                                let _ = self.cmd_tx.send(DdcCommand::SetBrightness(self.selected_target, 20));
                            }
                        });

                        ui.add_space(10.0);

                        // Contrast Slider
                        ui.horizontal(|ui| {
                            ui.label("Contrast:");
                            let slider = ui.add(egui::Slider::new(&mut self.contrast, 0..=100).suffix("%"));
                            if slider.changed() {
                                let _ = self.cmd_tx.send(DdcCommand::SetContrast(self.selected_target, self.contrast));
                            }
                        });

                        ui.add_space(12.0);

                        // Audio Volume Controls
                        ui.heading("🔊 Monitor Audio Volume");
                        ui.horizontal(|ui| {
                            let mute_btn_text = if self.is_muted { "🔇 Muted" } else { "🔊 Mute" };
                            if ui.button(mute_btn_text).clicked() {
                                self.is_muted = !self.is_muted;
                                let _ = self.cmd_tx.send(DdcCommand::SetMute(self.selected_target, self.is_muted));
                            }
                            let slider = ui.add(egui::Slider::new(&mut self.volume, 0..=100).suffix("%"));
                            if slider.changed() {
                                let _ = self.cmd_tx.send(DdcCommand::SetVolume(self.selected_target, self.volume));
                            }
                        });

                        ui.add_space(14.0);

                        // Real-Time Energy Meter
                        ui.heading("💡 Real-Time Power Draw");
                        let (wattage, _kwh, cost) = energy::calculate_power(self.brightness);
                        ui.label(egui::RichText::new(format!("Live Power: {:.1} W   |   Est. Yearly Cost: ${:.2}/yr", wattage, cost)).strong().color(egui::Color32::from_rgb(0, 230, 118)));
                    });

                    // Right Column: Presets, Input Source, Gaming Tuning
                    cols[1].group(|ui| {
                        ui.heading("🎛️ Native Hardware OSD Presets");
                        ui.add_space(6.0);

                        egui::Grid::new("presets_grid").num_columns(4).spacing([8.0, 8.0]).show(ui, |ui| {
                            if ui.button("⚖️ Standard").clicked() {
                                self.active_preset = "Standard".into();
                                let _ = self.cmd_tx.send(DdcCommand::SetPreset(self.selected_target, 1));
                            }
                            if ui.button("🌿 ECO Saver").clicked() {
                                self.active_preset = "ECO".into();
                                let _ = self.cmd_tx.send(DdcCommand::SetPreset(self.selected_target, 2));
                            }
                            if ui.button("🎨 Graphics").clicked() {
                                self.active_preset = "Graphics".into();
                                let _ = self.cmd_tx.send(DdcCommand::SetPreset(self.selected_target, 3));
                            }
                            if ui.button("⚡ HDR Mode").clicked() {
                                self.active_preset = "HDR".into();
                                let _ = self.cmd_tx.send(DdcCommand::SetPreset(self.selected_target, 11));
                            }
                            ui.end_row();

                            if ui.button("🎯 Action").clicked() {
                                self.active_preset = "Action".into();
                                let _ = self.cmd_tx.send(DdcCommand::SetPreset(self.selected_target, 5));
                            }
                            if ui.button("🏎️ Racing").clicked() {
                                self.active_preset = "Racing".into();
                                let _ = self.cmd_tx.send(DdcCommand::SetPreset(self.selected_target, 6));
                            }
                            if ui.button("⚽ Sports").clicked() {
                                self.active_preset = "Sports".into();
                                let _ = self.cmd_tx.send(DdcCommand::SetPreset(self.selected_target, 7));
                            }
                            if ui.button("👤 User Mode").clicked() {
                                self.active_preset = "User".into();
                                let _ = self.cmd_tx.send(DdcCommand::SetPreset(self.selected_target, 0));
                            }
                            ui.end_row();
                        });


                        ui.add_space(10.0);

                        // Input Source Switcher
                        ui.heading("🔌 Active Input Source");
                        ui.horizontal(|ui| {
                            if ui.selectable_label(self.active_input == "DisplayPort", "💻 DisplayPort").clicked() {
                                self.active_input = "DisplayPort".into();
                                let _ = self.cmd_tx.send(DdcCommand::SetInput(self.selected_target, 0x0F));
                            }
                            if ui.selectable_label(self.active_input == "HDMI 1", "🎮 HDMI 1").clicked() {
                                self.active_input = "HDMI 1".into();
                                let _ = self.cmd_tx.send(DdcCommand::SetInput(self.selected_target, 0x11));
                            }
                            if ui.selectable_label(self.active_input == "HDMI 2", "📺 HDMI 2").clicked() {
                                self.active_input = "HDMI 2".into();
                                let _ = self.cmd_tx.send(DdcCommand::SetInput(self.selected_target, 0x12));
                            }
                            if ui.selectable_label(self.active_input == "Auto", "🔄 Auto").clicked() {
                                self.active_input = "Auto".into();
                                let _ = self.cmd_tx.send(DdcCommand::SetInput(self.selected_target, 0x01));
                            }
                        });

                        ui.add_space(10.0);

                        // Gaming & Vision Hardware Tuning
                        ui.heading("🎮 Gaming & Vision Tuning");
                        ui.horizontal(|ui| {
                            ui.label("Black Boost:");
                            let slider = ui.add(egui::Slider::new(&mut self.black_boost, 0..=10));
                            if slider.changed() {
                                let _ = self.cmd_tx.send(DdcCommand::SetBlackBoost(self.selected_target, self.black_boost));
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("Blue Light:");
                            if ui.selectable_label(self.blue_light == 0, "Off").clicked() {
                                self.blue_light = 0;
                                let _ = self.cmd_tx.send(DdcCommand::SetBlueLight(self.selected_target, 0));
                            }
                            if ui.selectable_label(self.blue_light == 1, "50%").clicked() {
                                self.blue_light = 1;
                                let _ = self.cmd_tx.send(DdcCommand::SetBlueLight(self.selected_target, 1));
                            }
                            if ui.selectable_label(self.blue_light == 2, "60%").clicked() {
                                self.blue_light = 2;
                                let _ = self.cmd_tx.send(DdcCommand::SetBlueLight(self.selected_target, 2));
                            }
                            if ui.selectable_label(self.blue_light == 3, "70%").clicked() {
                                self.blue_light = 3;
                                let _ = self.cmd_tx.send(DdcCommand::SetBlueLight(self.selected_target, 3));
                            }
                            if ui.selectable_label(self.blue_light == 4, "80%").clicked() {
                                self.blue_light = 4;
                                let _ = self.cmd_tx.send(DdcCommand::SetBlueLight(self.selected_target, 4));
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("OverDrive:");
                            if ui.selectable_label(self.overdrive == 0, "Off").clicked() {
                                self.overdrive = 0;
                                let _ = self.cmd_tx.send(DdcCommand::SetOverDrive(self.selected_target, 0));
                            }
                            if ui.selectable_label(self.overdrive == 1, "Normal").clicked() {
                                self.overdrive = 1;
                                let _ = self.cmd_tx.send(DdcCommand::SetOverDrive(self.selected_target, 1));
                            }
                            if ui.selectable_label(self.overdrive == 2, "Extreme").clicked() {
                                self.overdrive = 2;
                                let _ = self.cmd_tx.send(DdcCommand::SetOverDrive(self.selected_target, 2));
                            }
                        });
                    });
                });

                ui.add_space(14.0);
                ui.separator();

                // Diagnostic Test Pattern Generator Canvas
                ui.group(|ui| {
                    ui.heading("🎨 Diagnostic Test Pattern Canvas");
                    ui.horizontal(|ui| {
                        if ui.button("📐 Grid").clicked() { self.active_pattern = Some("grid"); }
                        if ui.button("📈 Gradient").clicked() { self.active_pattern = Some("gradient"); }
                        if ui.button("🔴 Red").clicked() { self.active_pattern = Some("red"); }
                        if ui.button("🟢 Green").clicked() { self.active_pattern = Some("green"); }
                        if ui.button("🔵 Blue").clicked() { self.active_pattern = Some("blue"); }
                        if ui.button("⚪ White").clicked() { self.active_pattern = Some("white"); }
                        if ui.button("⬛ Clear").clicked() { self.active_pattern = None; }
                    });

                    ui.add_space(6.0);
                    let (response, painter) = ui.allocate_painter(egui::vec2(ui.available_width(), 120.0), egui::Sense::hover());
                    let rect = response.rect;

                    painter.rect_filled(rect, 4.0, egui::Color32::from_rgb(15, 20, 32));

                    if let Some(pat) = self.active_pattern {
                        match pat {
                            "red" => { painter.rect_filled(rect, 4.0, egui::Color32::RED); }
                            "green" => { painter.rect_filled(rect, 4.0, egui::Color32::GREEN); }
                            "blue" => { painter.rect_filled(rect, 4.0, egui::Color32::BLUE); }
                            "white" => { painter.rect_filled(rect, 4.0, egui::Color32::WHITE); }
                            "grid" => {
                                for x in (rect.min.x as i32..rect.max.x as i32).step_by(30) {
                                    painter.line_segment([egui::pos2(x as f32, rect.min.y), egui::pos2(x as f32, rect.max.y)], (1.0, egui::Color32::GRAY));
                                }
                                for y in (rect.min.y as i32..rect.max.y as i32).step_by(30) {
                                    painter.line_segment([egui::pos2(rect.min.x, y as f32), egui::pos2(rect.max.x, y as f32)], (1.0, egui::Color32::GRAY));
                                }
                            }
                            "gradient" => {
                                let steps = 20;
                                let step_w = rect.width() / steps as f32;
                                for i in 0..steps {
                                    let v = (i as f32 / steps as f32 * 255.0) as u8;
                                    let r = egui::Rect::from_min_size(
                                        egui::pos2(rect.min.x + i as f32 * step_w, rect.min.y),
                                        egui::vec2(step_w, rect.height()),
                                    );
                                    painter.rect_filled(r, 0.0, egui::Color32::from_gray(v));
                                }
                            }
                            _ => {}
                        }
                    }
                });
            });
        });
    }
}
