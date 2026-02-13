pub use cosmic_text::{
	Align as TextAlign, Stretch as TextStretch, Style as TextStyle, Weight as TextWeight,
};

use cosmic_text::{Attrs, Family, LetterSpacing, Metrics, Shaping};
use glam::Mat4;
use image::{Rgba, RgbaImage};
use palette::LinSrgba;

use crate::{
	Context,
	color::ColorConstants,
	graphics::{
		BlendMode,
		texture::{Texture, TextureSettings},
	},
	standard_draw_param_methods,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Text {
	texture: Texture,

	// params
	/// The transform to use when drawing this text.
	pub transform: Mat4,
	/// The blend color to use when drawing this text.
	pub color: LinSrgba,
	/// The blend mode to use when drawing this text.
	pub blend_mode: BlendMode,
}

impl Text {
	standard_draw_param_methods!();

	pub fn size(&self) -> glam::UVec2 {
		self.texture.size()
	}

	pub fn draw(&self, ctx: &mut Context) {
		let _span = tracy_client::span!();
		self.texture
			.transformed(self.transform)
			.color(self.color)
			.blend_mode(self.blend_mode)
			.draw(ctx);
	}

	fn new(ctx: &mut Context, builder: TextBuilder) -> Self {
		let _span = tracy_client::span!();
		let mut buffer = cosmic_text::Buffer::new(
			&mut ctx.font_system,
			Metrics::relative(builder.font_size, builder.line_height),
		);
		let mut buffer = buffer.borrow_with(&mut ctx.font_system);
		let (buffer_width, align) = match builder.horizontal_sizing {
			TextHorizontalSizing::Min => (None, None),
			TextHorizontalSizing::Fixed { width, align } => (Some(width), Some(align)),
		};
		buffer.set_size(buffer_width, None);
		buffer.set_text(
			&builder.text,
			&Attrs {
				family: Family::Name(&builder.font_family),
				stretch: builder.stretch,
				style: builder.style,
				weight: builder.weight,
				letter_spacing_opt: builder.letter_spacing.map(LetterSpacing),
				..Attrs::new()
			},
			Shaping::Advanced,
			align,
		);
		buffer.shape_until_scroll(true);
		let texture_width = buffer_width
			.map(|width| width.ceil() as u32)
			.unwrap_or_else(|| {
				buffer
					.layout_runs()
					.fold(0.0f32, |previous, run| previous.max(run.line_w))
					.ceil() as u32
			});
		let texture_height = {
			let mut height = 0;
			buffer.draw(
				&mut ctx.swash_cache,
				cosmic_text::Color::rgb(0xff, 0xff, 0xff),
				|_, y, _, h, _| {
					let bottom = (y + h as i32) as u32;
					height = height.max(bottom);
				},
			);
			height
		};
		let mut image = RgbaImage::new(texture_width, texture_height);
		buffer.draw(
			&mut ctx.swash_cache,
			cosmic_text::Color::rgb(0xff, 0xff, 0xff),
			|x, y, w, h, color| {
				for pixel_x in x as u32..(x + w as i32) as u32 {
					for pixel_y in y as u32..(y + h as i32) as u32 {
						if pixel_x < texture_width && pixel_y < texture_height {
							image.put_pixel(pixel_x, pixel_y, Rgba::from(color.as_rgba()));
						}
					}
				}
			},
		);
		let texture = Texture::from_image(ctx, &image, builder.texture_settings);
		Self {
			texture,
			transform: Mat4::IDENTITY,
			color: LinSrgba::WHITE,
			blend_mode: BlendMode::default(),
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextBuilder {
	pub font_family: String,
	pub text: String,
	pub font_size: f32,
	pub line_height: f32,
	pub stretch: TextStretch,
	pub style: TextStyle,
	pub weight: TextWeight,
	pub letter_spacing: Option<f32>,
	pub horizontal_sizing: TextHorizontalSizing,
	pub texture_settings: TextureSettings,
}

impl TextBuilder {
	pub fn new(font_family: impl Into<String>, text: impl Into<String>) -> Self {
		Self {
			font_family: font_family.into(),
			text: text.into(),
			font_size: 16.0,
			line_height: 1.0,
			stretch: TextStretch::Normal,
			style: TextStyle::Normal,
			weight: TextWeight::NORMAL,
			letter_spacing: None,
			horizontal_sizing: TextHorizontalSizing::default(),
			texture_settings: TextureSettings::default(),
		}
	}

	pub fn font_family(self, font_family: impl Into<String>) -> Self {
		Self {
			font_family: font_family.into(),
			..self
		}
	}

	pub fn text(self, text: impl Into<String>) -> Self {
		Self {
			text: text.into(),
			..self
		}
	}

	pub fn font_size(self, font_size: f32) -> Self {
		Self { font_size, ..self }
	}

	pub fn line_height(self, line_height: f32) -> Self {
		Self {
			line_height,
			..self
		}
	}

	pub fn stretch(self, stretch: TextStretch) -> Self {
		Self { stretch, ..self }
	}

	pub fn style(self, style: TextStyle) -> Self {
		Self { style, ..self }
	}

	pub fn weight(self, weight: TextWeight) -> Self {
		Self { weight, ..self }
	}

	pub fn letter_spacing(self, letter_spacing: impl Into<Option<f32>>) -> Self {
		Self {
			letter_spacing: letter_spacing.into(),
			..self
		}
	}

	pub fn horizontal_sizing(self, horizontal_sizing: TextHorizontalSizing) -> Self {
		Self {
			horizontal_sizing,
			..self
		}
	}

	pub fn texture_settings(self, texture_settings: TextureSettings) -> Self {
		Self {
			texture_settings,
			..self
		}
	}

	pub fn build(self, ctx: &mut Context) -> Text {
		Text::new(ctx, self)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TextHorizontalSizing {
	#[default]
	Min,
	Fixed {
		width: f32,
		align: TextAlign,
	},
}
