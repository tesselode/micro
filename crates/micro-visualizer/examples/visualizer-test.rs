use std::path::PathBuf;

use micro::{
	egui::Ui,
	graphics::{mesh::Mesh, Canvas, RenderToCanvasSettings},
	math::Rect,
	Micro,
};
use micro_visualizer::{Visualizer, VisualizerInfo};

struct TestVisualizer;

impl TestVisualizer {
	pub fn new(_micro: &mut Micro) -> Self {
		Self
	}
}

impl Visualizer for TestVisualizer {
	fn audio_path(&self) -> PathBuf {
		"test.flac".into()
	}

	fn menu(&mut self, _micro: &mut Micro, ui: &mut Ui, _vis_info: VisualizerInfo) {
		ui.label("hello!");
	}

	fn draw(&mut self, micro: &mut Micro, vis_info: VisualizerInfo, main_canvas: &Canvas) {
		main_canvas.render_to(micro, RenderToCanvasSettings::default(), |micro| {
			Mesh::rectangle(
				micro,
				Rect::new((50.0 + vis_info.current_frame as f32, 50.0), (100.0, 150.0)),
			)
			.draw(micro);
		});
	}
}

fn main() {
	micro_visualizer::run(TestVisualizer::new)
}
