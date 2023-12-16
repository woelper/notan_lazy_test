use notan::egui::{self, *};
use notan::prelude::*;

#[notan_main]
fn main() -> Result<(), String> {
    let win = WindowConfig::new()
        .set_vsync(true)
        .set_lazy_loop(true)
        .set_high_dpi(true);
    notan::init()
        .add_config(win)
        .add_config(EguiConfig)
        .draw(draw)
        .build()
}

fn draw(app: &mut App, gfx: &mut Graphics, plugins: &mut Plugins) {
    let mut output = plugins.egui(|ctx| {
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Egui Plugin Example");
            ui.spinner();
        });
    });
    output.clear_color(Color::BLACK);

    if output.needs_repaint() {
        gfx.render(&output);
    }
    app.window().request_frame();
}