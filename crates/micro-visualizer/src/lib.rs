mod chapters;
mod conversions;
mod vis_runner;

pub use chapters::*;
pub use conversions::*;
pub use micro::*;

use std::{path::PathBuf, time::Duration};

use micro::{egui::Ui, graphics::Canvas, math::UVec2};
use vis_runner::VisRunner;

pub fn run<T: Visualizer>(mut visualizer_constructor: impl FnMut(&mut Micro) -> T) {
	micro::run(
		MicroSettings {
			window_title: "Micro Visualizer".into(),
			window_mode: WindowMode::Windowed {
				size: UVec2::new(1920, 1080),
			},
			resizable: true,
			..Default::default()
		},
		|micro| {
			let visualizer = Box::new(visualizer_constructor(micro));
			VisRunner::new(micro, visualizer)
		},
	)
}

#[allow(unused_variables)]
pub trait Visualizer: 'static {
	fn audio_path(&self) -> PathBuf;

	fn frame_rate(&self) -> u64 {
		60
	}

	fn video_resolution(&self) -> UVec2 {
		UVec2::new(3840, 2160)
	}

	fn chapters(&self) -> Option<&Chapters> {
		None
	}

	fn ui(&mut self, micro: &mut Micro, egui_ctx: &micro::egui::Context, vis_info: VisualizerInfo) {
	}

	fn menu(&mut self, micro: &mut Micro, ui: &mut Ui, vis_info: VisualizerInfo) {}

	fn event(&mut self, micro: &mut Micro, vis_info: VisualizerInfo, event: Event) {}

	fn update(&mut self, micro: &mut Micro, vis_info: VisualizerInfo, delta_time: Duration) {}

	fn draw(&mut self, micro: &mut Micro, vis_info: VisualizerInfo, main_canvas: &Canvas);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VisualizerInfo {
	pub resolution: UVec2,
	pub current_frame: u64,
	pub current_time: Duration,
	pub current_chapter_index: Option<usize>,
	pub current_chapter_frame: Option<u64>,
	pub current_chapter_time: Option<Duration>,
	pub num_frames: u64,
}
