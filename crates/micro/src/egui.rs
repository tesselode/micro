pub use egui::*;
use palette::LinSrgba;

pub fn color_preview(ui: &mut Ui, color: impl Into<LinSrgba>) {
	let color = color.into();
	color_picker::show_color(
		ui,
		egui::Rgba::from_rgba_unmultiplied(color.red, color.green, color.blue, color.alpha),
		egui::vec2(50.0, ui.available_height()),
	)
	.on_hover_ui(|ui| {
		ui.label(format!(
			"{}, {}, {}, {}",
			color.red, color.green, color.blue, color.alpha
		));
	});
}
