use crate::{acer, energy, monitor::MonitorSet};
use eframe::egui;

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
}

impl Default for AcerMonitorApp {
    fn default() -> Self {
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
            status_msg: "Ready (DDC/CI Connected)".to_string(),
            active_pattern: None,
        }
    }
}

impl eframe::App for AcerMonitorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Dark Glassmorphic Theme Styling
        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill = egui::Color32::from_rgb(11, 15, 25);
        visuals.window_fill = egui::Color32::from_rgb(20, 26, 42);
        visuals.widgets.active.bg_fill = egui::Color32::from_rgb(0, 229, 255);
        visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(124, 77, 255);
        ctx.set_visuals(visuals);

        // Top Header Bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.heading(egui::RichText::new("🖥️ Acer Monitor Control").strong().color(egui::Color32::from_rgb(0, 229, 255)));
                ui.label(egui::RichText::new("Native Hardware GUI").small().color(egui::Color32::GRAY));

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("🔓 Unlock OSD").clicked() {
                        self.status_msg = "Unlocked OSD keys".to_string();
                        let _ = with_target(self.selected_target, |mon| {
                            let _ = acer::key_lock(mon, false);
                            let _ = acer::power_key(mon, false);
                            Ok(())
                        });
                    }
                    if ui.button("⚡ Power Off").clicked() {
                        self.status_msg = "Power off sent".to_string();
                        let _ = with_target(self.selected_target, |mon| acer::power_mode(mon, false));
                    }
                    ui.label(egui::RichText::new(&self.status_msg).small().color(egui::Color32::from_rgb(0, 230, 118)));
                });
            });
            ui.add_space(8.0);
        });

        // Main Dashboard Body
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add_space(10.0);

                // Target Selector Tabs
                ui.horizontal(|ui| {
                    ui.label("Target Display:");
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

                ui.separator();
                ui.add_space(10.0);

                // Grid Layout: 2 Columns
                ui.columns(2, |cols| {
                    // Column 1: Brightness, Contrast, Volume & Energy
                    cols[0].group(|ui| {
                        ui.heading("☀️ Brightness & Display Controls");
                        ui.add_space(6.0);

                        // Brightness Slider
                        ui.horizontal(|ui| {
                            ui.label("Brightness:");
                            let slider = ui.add(egui::Slider::new(&mut self.brightness, 0..=100).suffix("%"));
                            if slider.changed() {
                                let b = self.brightness;
                                let _ = with_target(self.selected_target, |mon| acer::brightness(mon, b));
                            }
                        });

                        // Quick Brightness Presets
                        ui.horizontal(|ui| {
                            if ui.button("☀️ 100%").clicked() {
                                self.brightness = 100;
                                let _ = with_target(self.selected_target, |mon| acer::brightness(mon, 100));
                            }
                            if ui.button("⚖️ 80%").clicked() {
                                self.brightness = 80;
                                let _ = with_target(self.selected_target, |mon| acer::brightness(mon, 80));
                            }
                            if ui.button("🔉 50%").clicked() {
                                self.brightness = 50;
                                let _ = with_target(self.selected_target, |mon| acer::brightness(mon, 50));
                            }
                            if ui.button("🌙 20%").clicked() {
                                self.brightness = 20;
                                let _ = with_target(self.selected_target, |mon| acer::brightness(mon, 20));
                            }
                        });

                        ui.add_space(10.0);

                        // Contrast Slider
                        ui.horizontal(|ui| {
                            ui.label("Contrast:");
                            let slider = ui.add(egui::Slider::new(&mut self.contrast, 0..=100).suffix("%"));
                            if slider.changed() {
                                let c = self.contrast;
                                let _ = with_target(self.selected_target, |mon| acer::contrast(mon, c));
                            }
                        });

                        ui.add_space(10.0);

                        // Volume & Mute
                        ui.heading("🔊 Monitor Audio Volume");
                        ui.horizontal(|ui| {
                            let mute_btn_text = if self.is_muted { "🔇 Muted" } else { "🔊 Mute" };
                            if ui.button(mute_btn_text).clicked() {
                                self.is_muted = !self.is_muted;
                                let m = self.is_muted;
                                let _ = with_target(self.selected_target, |mon| acer::mute(mon, m));
                            }
                            let slider = ui.add(egui::Slider::new(&mut self.volume, 0..=100).suffix("%"));
                            if slider.changed() {
                                let v = self.volume;
                                let _ = with_target(self.selected_target, |mon| acer::volume(mon, v));
                            }
                        });

                        ui.add_space(12.0);

                        // Real-Time Energy Calculator
                        ui.heading("💡 Real-Time Power Draw");
                        let (wattage, _kwh, cost) = energy::calculate_power(self.brightness);
                        ui.label(egui::RichText::new(format!("Live Power: {:.1} W   |   Est. Yearly Cost: ${:.2}/yr", wattage, cost)).strong().color(egui::Color32::from_rgb(0, 230, 118)));
                    });

                    // Column 2: OSD Modes, Inputs & Hardware Tuning
                    cols[1].group(|ui| {
                        ui.heading("🎛️ Native Hardware OSD Presets");
                        ui.add_space(6.0);

                        egui::Grid::new("presets_grid").num_columns(3).spacing([8.0, 8.0]).show(ui, |ui| {
                            if ui.button("⚖️ Standard").clicked() {
                                self.active_preset = "Standard".into();
                                let _ = with_target(self.selected_target, |mon| acer::display_mode(mon, 1));
                            }
                            if ui.button("🌿 ECO Saver").clicked() {
                                self.active_preset = "ECO".into();
                                let _ = with_target(self.selected_target, |mon| acer::display_mode(mon, 2));
                            }
                            if ui.button("⚡ HDR Mode").clicked() {
                                self.active_preset = "HDR".into();
                                let _ = with_target(self.selected_target, |mon| acer::display_mode(mon, 11));
                            }
                            ui.end_row();

                            if ui.button("🎯 Action").clicked() {
                                self.active_preset = "Action".into();
                                let _ = with_target(self.selected_target, |mon| acer::display_mode(mon, 5));
                            }
                            if ui.button("🏎️ Racing").clicked() {
                                self.active_preset = "Racing".into();
                                let _ = with_target(self.selected_target, |mon| acer::display_mode(mon, 6));
                            }
                            if ui.button("⚽ Sports").clicked() {
                                self.active_preset = "Sports".into();
                                let _ = with_target(self.selected_target, |mon| acer::display_mode(mon, 7));
                            }
                            ui.end_row();

                            if ui.button("📚 Reading").clicked() {
                                self.active_preset = "Reading".into();
                                let _ = with_target(self.selected_target, |mon| mon.set_vcp(0xDC, 0x02));
                            }
                            if ui.button("🎬 Movie").clicked() {
                                self.active_preset = "Movie".into();
                                let _ = with_target(self.selected_target, |mon| mon.set_vcp(0xDC, 0x03));
                            }
                            if ui.button("🎨 User Mode").clicked() {
                                self.active_preset = "User".into();
                                let _ = with_target(self.selected_target, |mon| acer::display_mode(mon, 0));
                            }
                            ui.end_row();
                        });

                        ui.add_space(10.0);

                        // Input Source Switcher
                        ui.heading("🔌 Active Input Source");
                        ui.horizontal(|ui| {
                            if ui.selectable_label(self.active_input == "DisplayPort", "💻 DisplayPort").clicked() {
                                self.active_input = "DisplayPort".into();
                                let _ = with_target(self.selected_target, |mon| acer::input(mon, 0x0F));
                            }
                            if ui.selectable_label(self.active_input == "HDMI 1", "🎮 HDMI 1").clicked() {
                                self.active_input = "HDMI 1".into();
                                let _ = with_target(self.selected_target, |mon| acer::input(mon, 0x11));
                            }
                            if ui.selectable_label(self.active_input == "HDMI 2", "📺 HDMI 2").clicked() {
                                self.active_input = "HDMI 2".into();
                                let _ = with_target(self.selected_target, |mon| acer::input(mon, 0x12));
                            }
                            if ui.selectable_label(self.active_input == "Auto", "🔄 Auto").clicked() {
                                self.active_input = "Auto".into();
                                let _ = with_target(self.selected_target, |mon| acer::input(mon, 0x01));
                            }
                        });

                        ui.add_space(10.0);

                        // Gaming & Vision Hardware Tuning
                        ui.heading("🎮 Gaming & Vision Tuning");
                        ui.horizontal(|ui| {
                            ui.label("Black Boost:");
                            let slider = ui.add(egui::Slider::new(&mut self.black_boost, 0..=10));
                            if slider.changed() {
                                let bb = self.black_boost;
                                let _ = with_target(self.selected_target, |mon| acer::black_boost(mon, bb));
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("Blue Light:");
                            if ui.selectable_label(self.blue_light == 0, "Off").clicked() {
                                self.blue_light = 0;
                                let _ = with_target(self.selected_target, |mon| acer::blue_light(mon, 0));
                            }
                            if ui.selectable_label(self.blue_light == 1, "50%").clicked() {
                                self.blue_light = 1;
                                let _ = with_target(self.selected_target, |mon| acer::blue_light(mon, 1));
                            }
                            if ui.selectable_label(self.blue_light == 2, "60%").clicked() {
                                self.blue_light = 2;
                                let _ = with_target(self.selected_target, |mon| acer::blue_light(mon, 2));
                            }
                            if ui.selectable_label(self.blue_light == 3, "70%").clicked() {
                                self.blue_light = 3;
                                let _ = with_target(self.selected_target, |mon| acer::blue_light(mon, 3));
                            }
                            if ui.selectable_label(self.blue_light == 4, "80%").clicked() {
                                self.blue_light = 4;
                                let _ = with_target(self.selected_target, |mon| acer::blue_light(mon, 4));
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label("OverDrive:");
                            if ui.selectable_label(self.overdrive == 0, "Off").clicked() {
                                self.overdrive = 0;
                                let _ = with_target(self.selected_target, |mon| acer::overdrive(mon, 0));
                            }
                            if ui.selectable_label(self.overdrive == 1, "Normal").clicked() {
                                self.overdrive = 1;
                                let _ = with_target(self.selected_target, |mon| acer::overdrive(mon, 1));
                            }
                            if ui.selectable_label(self.overdrive == 2, "Extreme").clicked() {
                                self.overdrive = 2;
                                let _ = with_target(self.selected_target, |mon| acer::overdrive(mon, 2));
                            }
                            if ui.button("🎯 Cycle AimPoint").clicked() {
                                let _ = with_target(self.selected_target, |mon| acer::aim_type(mon, 1));
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

fn with_target<F>(target_idx: usize, mut f: F) -> Result<(), String>
where
    F: FnMut(&mut crate::monitor::Monitor) -> Result<(), String>,
{
    let mut set = MonitorSet::enumerate()?;
    if target_idx == 99 {
        for mon in set.monitors_mut() {
            let _ = f(mon);
        }
        Ok(())
    } else {
        let spec_str = target_idx.to_string();
        if let Ok(mon) = set.pick_mut_by_specifier(Some(&spec_str)) {
            let _ = f(mon);
        }
        Ok(())
    }
}
