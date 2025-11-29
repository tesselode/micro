mod chapters;
mod rendering;
mod ui;

use std::{io::Write, process::Child, time::Duration};

use kira::{
	sound::{
		streaming::{StreamingSoundData, StreamingSoundHandle},
		FromFileError, PlaybackPosition, PlaybackState,
	},
	AudioManager, AudioManagerSettings, Decibels, Tween,
};
use micro::{
	graphics::{Canvas, CanvasSettings},
	input::Scancode,
	math::{UVec2, Vec2},
	App, Context, Event,
};

use crate::{
	conversions::{frame_to_seconds, seconds_to_frames, seconds_to_frames_i64},
	Visualizer, VisualizerInfo,
};

const FINISHED_SEEK_DETECTION_THRESHOLD: Duration = Duration::from_millis(100);

pub struct VisRunner {
	visualizer: Box<dyn Visualizer>,
	audio_manager: AudioManager,
	mode: Mode,
	num_frames: u64,
	previous_frame: u64,
	canvas: Canvas,
	live_resolution: LiveResolution,
	rendering_settings: RenderingSettings,
	show_rendering_window: bool,
	volume: Decibels,
}

impl VisRunner {
	pub fn new(ctx: &mut Context, visualizer: Box<dyn Visualizer>) -> anyhow::Result<Self> {
		let audio_manager = AudioManager::new(AudioManagerSettings::default())?;
		let sound_data = StreamingSoundData::from_file(visualizer.audio_path())?;
		let num_frames =
			seconds_to_frames(sound_data.duration().as_secs_f64(), visualizer.frame_rate());
		let canvas = Canvas::new(
			ctx,
			visualizer.video_resolution(),
			CanvasSettings {
				readable: true,
				..Default::default()
			},
		);
		let rendering_settings = if let Some(chapters) = visualizer.chapters() {
			RenderingSettings {
				start_chapter_index: 0,
				end_chapter_index: chapters.len() - 1,
			}
		} else {
			RenderingSettings::default()
		};
		Ok(VisRunner {
			visualizer,
			audio_manager,
			mode: Mode::Stopped {
				data: Some(sound_data),
				start_frame: 0,
			},
			num_frames,
			previous_frame: u64::MAX,
			canvas,
			live_resolution: LiveResolution::Full,
			rendering_settings,
			show_rendering_window: false,
			volume: Decibels::IDENTITY,
		})
	}

	fn playing(&self) -> bool {
		match &self.mode {
			Mode::Stopped { .. } => false,
			Mode::PlayingOrPaused { sound, .. } => sound.state() == PlaybackState::Playing,
			Mode::Rendering { .. } => false,
		}
	}

	fn current_frame(&self) -> u64 {
		match &self.mode {
			Mode::Stopped { start_frame, .. } => *start_frame,
			Mode::PlayingOrPaused {
				sound,
				in_progress_seek,
			} => in_progress_seek.unwrap_or_else(|| {
				seconds_to_frames(sound.position(), self.visualizer.frame_rate())
			}),
			Mode::Rendering { current_frame, .. } => *current_frame,
		}
	}

	fn play_or_resume(&mut self) -> anyhow::Result<()> {
		match &mut self.mode {
			Mode::Stopped { data, start_frame } => {
				let mut data = data.take().unwrap();
				data.settings.start_position = PlaybackPosition::Seconds(frame_to_seconds(
					*start_frame,
					self.visualizer.frame_rate(),
				));
				self.mode = Mode::PlayingOrPaused {
					sound: self.audio_manager.play(data)?,
					in_progress_seek: None,
				};
			}
			Mode::PlayingOrPaused { sound, .. } => {
				sound.resume(Tween::default());
			}
			Mode::Rendering { .. } => unreachable!("not supported in rendering mode"),
		}
		Ok(())
	}

	fn pause(&mut self) -> anyhow::Result<()> {
		if let Mode::PlayingOrPaused { sound, .. } = &mut self.mode {
			sound.pause(Tween::default());
		}
		Ok(())
	}

	fn toggle_playback(&mut self) -> anyhow::Result<()> {
		if self.playing() {
			self.pause()?;
		} else {
			self.play_or_resume()?;
		}
		Ok(())
	}

	fn seek(&mut self, frame: u64) -> anyhow::Result<()> {
		match &mut self.mode {
			Mode::Stopped { start_frame, .. } => {
				*start_frame = frame;
			}
			Mode::PlayingOrPaused {
				sound,
				in_progress_seek,
			} => {
				sound.seek_to(frame_to_seconds(frame, self.visualizer.frame_rate()));
				*in_progress_seek = Some(frame);
			}
			Mode::Rendering { .. } => unreachable!("not supported in rendering mode"),
		}
		Ok(())
	}

	fn seek_by(&mut self, delta: i64) -> anyhow::Result<()> {
		let frame = (self.current_frame() as i64 + delta).clamp(0, self.num_frames as i64);
		self.seek(frame as u64)
	}

	fn seek_by_seconds(&mut self, delta: f64) -> anyhow::Result<()> {
		let delta_frames = seconds_to_frames_i64(delta, self.visualizer.frame_rate());
		self.seek_by(delta_frames)
	}

	fn vis_info(&self) -> VisualizerInfo {
		let current_frame = self.current_frame();
		VisualizerInfo {
			resolution: self.current_resolution(),
			current_frame,
			current_time: Duration::from_secs_f64(frame_to_seconds(
				current_frame,
				self.visualizer.frame_rate(),
			)),
			current_chapter_index: self
				.visualizer
				.chapters()
				.and_then(|chapters| chapters.index_at_frame(current_frame)),
		}
	}

	fn current_resolution(&self) -> UVec2 {
		if matches!(self.mode, Mode::Rendering { .. }) {
			self.visualizer.video_resolution()
		} else {
			self.visualizer.video_resolution() / self.live_resolution.as_divisor()
		}
	}
}

impl App for VisRunner {
	fn debug_ui(
		&mut self,
		ctx: &mut Context,
		egui_ctx: &micro::egui::Context,
	) -> Result<(), anyhow::Error> {
		self.render_main_menu(ctx, egui_ctx)?;
		self.render_rendering_window(ctx, egui_ctx)?;
		self.visualizer.ui(ctx, egui_ctx, self.vis_info())?;
		Ok(())
	}

	fn event(&mut self, ctx: &mut Context, event: Event) -> Result<(), anyhow::Error> {
		if let Event::KeyPressed { key, .. } = event {
			match key {
				Scancode::Space => self.toggle_playback()?,
				Scancode::Left => self.seek_by_seconds(-10.0)?,
				Scancode::Right => self.seek_by_seconds(10.0)?,
				Scancode::Comma => self.go_to_previous_chapter()?,
				Scancode::Period => self.go_to_next_chapter()?,
				_ => {}
			}
		}

		self.visualizer.event(ctx, self.vis_info(), event)?;

		Ok(())
	}

	fn update(&mut self, ctx: &mut Context, delta_time: Duration) -> Result<(), anyhow::Error> {
		if self.canvas.size() != self.current_resolution() {
			self.canvas = Canvas::new(
				ctx,
				self.current_resolution(),
				CanvasSettings {
					readable: true,
					..Default::default()
				},
			);
		}

		if let Mode::PlayingOrPaused {
			sound,
			in_progress_seek,
		} = &mut self.mode
		{
			sound.set_volume(self.volume, Tween::default());
			if let Some(in_progress_seek_destination) = in_progress_seek {
				let detection_threshold_frames = seconds_to_frames_i64(
					FINISHED_SEEK_DETECTION_THRESHOLD.as_secs_f64(),
					self.visualizer.frame_rate(),
				);
				let sound_position_frames =
					seconds_to_frames_i64(sound.position(), self.visualizer.frame_rate());
				if (sound_position_frames - *in_progress_seek_destination as i64).abs()
					<= detection_threshold_frames
				{
					*in_progress_seek = None;
				}
			}
			if sound.state() == PlaybackState::Stopped {
				self.mode = Mode::Stopped {
					data: Some(StreamingSoundData::from_file(self.visualizer.audio_path())?),
					start_frame: 0,
				};
			}
		}

		self.visualizer.update(ctx, self.vis_info(), delta_time)?;

		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> Result<(), anyhow::Error> {
		let current_frame = self.current_frame();
		if current_frame != self.previous_frame {
			self.visualizer.draw(ctx, self.vis_info(), &self.canvas)?;
			self.previous_frame = current_frame;
		}
		let max_horizontal_scale = ctx.window_size().x as f32 / self.canvas.size().x as f32;
		let max_vertical_scale = ctx.window_size().y as f32 / self.canvas.size().y as f32;
		let scale = max_horizontal_scale.min(max_vertical_scale);
		self.canvas
			.translated_2d(-self.canvas.size().as_vec2() / 2.0)
			.scaled_2d(Vec2::splat(scale))
			.translated_2d(ctx.window_size().as_vec2() / 2.0)
			.draw(ctx);
		Ok(())
	}

	fn post_draw(&mut self, ctx: &mut Context) -> anyhow::Result<()> {
		if let Mode::Rendering {
			end_frame,
			current_frame,
			ffmpeg_process,
		} = &mut self.mode
		{
			let mut should_stop_rendering = false;
			self.canvas.read(ctx, |data| -> anyhow::Result<()> {
				let ffmpeg_stdin = ffmpeg_process.stdin.as_mut().unwrap();
				let write_result = ffmpeg_stdin.write_all(data);
				if write_result.is_err() {
					should_stop_rendering = true;
				} else {
					*current_frame += 1;
					if *current_frame > *end_frame {
						should_stop_rendering = true;
					}
				}
				Ok(())
			})?;
			if should_stop_rendering {
				self.stop_rendering(ctx)?;
			}
		}
		Ok(())
	}
}

#[allow(clippy::large_enum_variant)]
enum Mode {
	Stopped {
		data: Option<StreamingSoundData<FromFileError>>,
		start_frame: u64,
	},
	PlayingOrPaused {
		sound: StreamingSoundHandle<FromFileError>,
		in_progress_seek: Option<u64>,
	},
	Rendering {
		end_frame: u64,
		current_frame: u64,
		ffmpeg_process: Child,
	},
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum LiveResolution {
	Full,
	Half,
	Quarter,
}

impl LiveResolution {
	const NUM_RESOLUTIONS: usize = 3;

	fn as_divisor(self) -> u32 {
		match self {
			LiveResolution::Full => 1,
			LiveResolution::Half => 2,
			LiveResolution::Quarter => 4,
		}
	}

	fn label(self) -> &'static str {
		match self {
			LiveResolution::Full => "Full",
			LiveResolution::Half => "Half",
			LiveResolution::Quarter => "Quarter",
		}
	}
}

impl From<usize> for LiveResolution {
	fn from(value: usize) -> Self {
		match value {
			0 => Self::Full,
			1 => Self::Half,
			2 => Self::Quarter,
			_ => panic!("invalid LiveResolution"),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct RenderingSettings {
	start_chapter_index: usize,
	end_chapter_index: usize,
}
