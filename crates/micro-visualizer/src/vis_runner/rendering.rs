use std::process::{Command, Stdio};

use kira::sound::streaming::StreamingSoundData;
use micro::{graphics::PresentMode, Context};
use rfd::FileDialog;

use crate::conversions::frame_to_seconds;

use super::{Mode, VisRunner};

impl VisRunner {
	pub fn render(&mut self, ctx: &mut Context) -> anyhow::Result<()> {
		let Some(video_path) = FileDialog::new()
			.set_directory(std::env::current_exe().unwrap())
			.add_filter("mp4 video", &["mp4"])
			.save_file()
		else {
			return Ok(());
		};
		let (start_frame, end_frame) = if let Some(chapters) = self.visualizer.chapters() {
			let start_frame = chapters[self.rendering_settings.start_chapter_index].start_frame;
			let end_frame = chapters
				.end_frame(self.rendering_settings.end_chapter_index)
				.unwrap_or(self.num_frames);
			(start_frame, end_frame)
		} else {
			(0, self.num_frames)
		};
		let start_time = frame_to_seconds(start_frame, self.visualizer.frame_rate());
		let ffmpeg_process = Command::new("ffmpeg")
			.stdin(Stdio::piped())
			.arg("-y")
			.arg("-f")
			.arg("rawvideo")
			.arg("-vcodec")
			.arg("rawvideo")
			.arg("-s")
			.arg(format!(
				"{}x{}",
				self.visualizer.video_resolution().x,
				self.visualizer.video_resolution().y
			))
			.arg("-pix_fmt")
			.arg("rgba")
			.arg("-r")
			.arg(self.visualizer.frame_rate().to_string())
			.arg("-i")
			.arg("-")
			.arg("-ss")
			.arg(format!("{}s", start_time))
			.arg("-i")
			.arg(self.visualizer.audio_path())
			.arg("-b:a")
			.arg("320k")
			.arg("-c:v")
			.arg("libx264")
			.arg("-r")
			.arg(self.visualizer.frame_rate().to_string())
			.arg("-shortest")
			.arg(video_path)
			.spawn()?;
		self.mode = Mode::Rendering {
			end_frame,
			current_frame: start_frame,
			ffmpeg_process,
		};
		ctx.set_present_mode(PresentMode::AutoNoVsync);
		Ok(())
	}

	pub fn on_rendering_finished(&mut self, ctx: &mut Context) -> Result<(), anyhow::Error> {
		self.mode = Mode::Stopped {
			data: Some(StreamingSoundData::from_file(self.visualizer.audio_path())?),
			start_frame: 0,
		};
		ctx.set_present_mode(PresentMode::AutoVsync);
		Ok(())
	}
}
