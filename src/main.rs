mod acer;
mod app;
mod ddc;
mod edid;
mod energy;
mod monitor;

use app::AcerMonitorApp;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Acer Monitor Control - Native Desktop GUI")
            .with_inner_size([960.0, 720.0])
            .with_min_inner_size([720.0, 540.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Acer Monitor Control",
        options,
        Box::new(|_cc| Box::new(AcerMonitorApp::default())),
    )
}
