use std::error::Error;

use micro::{
	clear,
	graphics::{
		text::{Font, FontSettings, LayoutSettings, Text, TextFragment},
		texture::{Texture, TextureSettings},
		Canvas, CanvasSettings, ColorConstants,
	},
	render_to_canvas, window_size, ContextSettings, State,
};
use palette::LinSrgba;

fn main() {
	micro::run(ContextSettings::default(), MainState::new);
}

struct MainState {
	text: Text,
}

impl MainState {
	pub fn new() -> Result<Self, Box<dyn Error>> {
		let main_font = Font::from_file(
			"resources/consoleet_ter32bv.otf",
			FontSettings {
				chars: FontSettings::default().chars,
				..Default::default()
			},
		)?;
		let fallback_font = Font::from_file(
			"resources/NotoSansJP-Regular.ttf",
			FontSettings {
				chars: FontSettings::default().chars + "心の底から感謝です",
				..Default::default()
			},
		)?;
		let text = text_with_fallback_fonts(
			&[&main_font, &fallback_font],
			"Test text 心の底から感謝です",
			LayoutSettings::default(),
		);
		Ok(Self { text })
	}
}

impl State<Box<dyn Error>> for MainState {
	fn draw(&mut self) -> Result<(), Box<dyn Error>> {
		clear(LinSrgba::BLACK);
		self.text.draw();
		Ok(())
	}
}

fn text_with_fallback_fonts(fonts: &[&Font], text: &str, settings: LayoutSettings) -> Text {
	let fragments = text
		.chars()
		.map(|char| TextFragment {
			font_index: first_font_that_has_char(fonts, char).unwrap_or(fonts.len() - 1),
			text: char.into(),
		})
		.collect::<Vec<_>>();
	Text::with_multiple_fonts(fonts, &fragments, settings)
}

fn first_font_that_has_char(fonts: &[&Font], c: char) -> Option<usize> {
	fonts
		.iter()
		.enumerate()
		.find(|(_, font)| font.has_glyph(c))
		.map(|(i, _)| i)
}
