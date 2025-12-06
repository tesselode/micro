use kira::Decibels;
use micro::{
	egui::{ComboBox, InnerResponse, Slider, Ui},
	Context,
};

use crate::conversions::frame_to_seconds;

use super::{LiveResolution, Mode, VisRunner};

impl VisRunner {
	pub fn render_main_menu_contents(
		&mut self,
		ctx: &mut Context,
		ui: &mut Ui,
	) -> Result<(), anyhow::Error> {
		self.render_play_pause_button(ui)?;
		self.render_seekbar(ui)?;
		ui.separator();
		self.render_chapter_switcher(ui)?;
		ui.label("Volume");
		ui.add(
			Slider::new(
				&mut self.volume.0,
				Decibels::SILENCE.0..=Decibels::IDENTITY.0,
			)
			.show_value(false),
		);
		ui.separator();
		if !matches!(self.mode, Mode::Rendering { .. }) {
			let mut selected_resolution_index = self.live_resolution as usize;
			ComboBox::new("resolution", "").show_index(
				ui,
				&mut selected_resolution_index,
				LiveResolution::NUM_RESOLUTIONS,
				|index| LiveResolution::from(index).label(),
			);
			self.live_resolution = LiveResolution::from(selected_resolution_index);
		}
		if ui.button("Render").clicked() {
			self.show_rendering_window = true;
		}
		self.visualizer.menu(ctx, ui, self.vis_info())?;
		Ok(())
	}

	pub fn render_rendering_window(
		&mut self,
		ctx: &mut Context,
		egui_ctx: &micro::egui::Context,
	) -> anyhow::Result<()> {
		let response = micro::egui::Window::new("Rendering")
			.open(&mut self.show_rendering_window)
			.show(egui_ctx, |ui| {
				let mut rendering_started = false;
				if let Some(chapters) = self.visualizer.chapters() {
					ComboBox::new("start_chapter_index", "Start Chapter Index").show_index(
						ui,
						&mut self.rendering_settings.start_chapter_index,
						chapters.len(),
						|i| &chapters[i].name,
					);
					ComboBox::new("end_chapter_index", "End Chapter Index").show_index(
						ui,
						&mut self.rendering_settings.end_chapter_index,
						chapters.len(),
						|i| &chapters[i].name,
					);
				}
				if ui.button("Render").clicked() {
					rendering_started = true;
				}
				rendering_started
			});
		if let Some(InnerResponse {
			inner: Some(true), ..
		}) = response
		{
			self.render(ctx)?;
		}
		Ok(())
	}

	fn render_play_pause_button(&mut self, ui: &mut Ui) -> Result<(), anyhow::Error> {
		if matches!(self.mode, Mode::Rendering { .. }) {
			return Ok(());
		}
		let play_pause_button_text = if self.playing() { "Pause" } else { "Play" };
		if ui.button(play_pause_button_text).clicked() {
			self.toggle_playback()?;
		};
		Ok(())
	}

	fn render_seekbar(&mut self, ui: &mut Ui) -> Result<(), anyhow::Error> {
		let mut frame = self.current_frame();
		let (start_frame, end_frame) = if let Some(chapters) = self.visualizer.chapters() {
			let current_chapter_index = chapters
				.index_at_frame(self.current_frame())
				.expect("no current chapter");
			let current_chapter = &chapters[current_chapter_index];
			let start_frame = current_chapter.start_frame;
			let end_frame = chapters
				.end_frame(current_chapter_index)
				.unwrap_or(self.num_frames);
			(start_frame, end_frame)
		} else {
			(0, self.num_frames)
		};
		let slider_response = &ui.add(
			Slider::new(&mut frame, start_frame..=end_frame).custom_formatter(|frame, _| {
				format!(
					"{} / {}",
					format_time(frame_to_seconds(frame as u64, self.visualizer.frame_rate())),
					format_time(frame_to_seconds(
						self.num_frames,
						self.visualizer.frame_rate()
					))
				)
			}),
		);
		if slider_response.drag_stopped() && !matches!(self.mode, Mode::Rendering { .. }) {
			self.seek(frame)?;
		};
		Ok(())
	}

	fn render_chapter_switcher(&mut self, ui: &mut Ui) -> anyhow::Result<()> {
		let Some(chapters) = self.visualizer.chapters() else {
			return Ok(());
		};
		let current_frame = self.current_frame();
		let current_chapter_index = chapters
			.index_at_frame(current_frame)
			.expect("no current chapter");
		if matches!(self.mode, Mode::Rendering { .. }) {
			ui.label(&chapters[current_chapter_index].name);
		} else {
			let mut selected = current_chapter_index;
			let response =
				ComboBox::new("chapter", "")
					.show_index(ui, &mut selected, chapters.len(), |i| &chapters[i].name);
			if response.changed() {
				self.go_to_chapter(selected)?;
			}
		}
		if ui.button("<<").clicked() {
			self.go_to_previous_chapter()?;
		}
		if ui.button(">>").clicked() {
			self.go_to_next_chapter()?;
		}
		ui.separator();
		Ok(())
	}
}

fn format_time(time: f64) -> String {
	let seconds = time % 60.0;
	let minutes = (time / 60.0).floor() % 60.0;
	let hours = (time / (60.0 * 60.0)).floor();
	format!("{}:{:0>2}:{:0>5.2}", hours, minutes, seconds)
}
