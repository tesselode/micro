use std::time::Duration;

use crate::conversions::frame_to_seconds;

use super::VisRunner;

const BEGINNING_OF_CHAPTER_THRESHOLD: Duration = Duration::from_secs(2);

impl VisRunner {
	pub fn go_to_chapter(&mut self, chapter_index: usize) -> anyhow::Result<()> {
		let Some(chapters) = self.visualizer.chapters() else {
			return Ok(());
		};
		self.seek(chapters[chapter_index].start_frame)?;
		Ok(())
	}

	pub fn go_to_next_chapter(&mut self) -> anyhow::Result<()> {
		let Some(chapters) = self.visualizer.chapters() else {
			return Ok(());
		};
		let current_chapter_index = chapters
			.index_at_frame(self.current_frame())
			.expect("no current chapter");
		if current_chapter_index >= chapters.len() - 1 {
			return Ok(());
		}
		self.go_to_chapter(current_chapter_index + 1)?;
		Ok(())
	}

	pub fn go_to_previous_chapter(&mut self) -> anyhow::Result<()> {
		let Some(chapters) = self.visualizer.chapters() else {
			return Ok(());
		};
		let current_chapter_index = chapters
			.index_at_frame(self.current_frame())
			.expect("no current chapter");
		let current_chapter = &chapters[current_chapter_index];
		let frames_since_start_of_chapter = self.current_frame() - current_chapter.start_frame;
		let time_since_start_of_chapter = Duration::from_secs_f64(frame_to_seconds(
			frames_since_start_of_chapter,
			self.visualizer.frame_rate(),
		));
		if current_chapter_index == 0
			|| time_since_start_of_chapter > BEGINNING_OF_CHAPTER_THRESHOLD
		{
			self.seek(current_chapter.start_frame)?;
		} else {
			self.go_to_chapter(current_chapter_index - 1)?;
		}
		Ok(())
	}
}
