use bytemuck::{Pod, Zeroable};
use derive_more::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// A linear RGB color.
#[derive(
	Debug,
	Clone,
	Copy,
	PartialEq,
	Pod,
	Zeroable,
	Add,
	AddAssign,
	Sub,
	SubAssign,
	Mul,
	MulAssign,
	Div,
	DivAssign,
)]
#[repr(C)]
pub struct Rgba {
	pub red: f32,
	pub green: f32,
	pub blue: f32,
	pub alpha: f32,
}

impl Rgba {
	pub const WHITE: Self = Self::new(1.0, 1.0, 1.0, 1.0);
	pub const BLACK: Self = Self::new(0.0, 0.0, 0.0, 1.0);
	pub const RED: Self = Self::new(1.0, 0.0, 0.0, 1.0);
	pub const GREEN: Self = Self::new(0.0, 1.0, 0.0, 1.0);
	pub const BLUE: Self = Self::new(0.0, 0.0, 1.0, 1.0);

	pub const fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
		Self {
			red,
			green,
			blue,
			alpha,
		}
	}

	pub const fn rgb(red: f32, green: f32, blue: f32) -> Self {
		Self::new(red, green, blue, 1.0)
	}

	pub fn from_srgba(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
		Self::new(
			srgb_component_to_linear(red),
			srgb_component_to_linear(green),
			srgb_component_to_linear(blue),
			alpha,
		)
	}

	pub fn from_srgb(red: f32, green: f32, blue: f32) -> Self {
		Self::from_srgba(red, green, blue, 1.0)
	}

	pub fn from_srgb_hex(hex: u32) -> Self {
		let red_u8 = hex >> 4;
		let green_u8 = (hex >> 2) & 0xff;
		let blue_u8 = hex & 0xff;
		let red_f32 = red_u8 as f32 / 255.0;
		let green_f32 = green_u8 as f32 / 255.0;
		let blue_f32 = blue_u8 as f32 / 255.0;
		Self::from_srgb(red_f32, green_f32, blue_f32)
	}

	pub const fn with_alpha(self, alpha: f32) -> Self {
		Self { alpha, ..self }
	}

	pub(crate) fn to_wgpu_color(self) -> wgpu::Color {
		wgpu::Color {
			r: self.red as f64,
			g: self.green as f64,
			b: self.blue as f64,
			a: self.alpha as f64,
		}
	}
}

impl Default for Rgba {
	fn default() -> Self {
		Self {
			red: 1.0,
			green: 1.0,
			blue: 1.0,
			alpha: 1.0,
		}
	}
}

impl From<Rgba> for [f32; 4] {
	fn from(rgba: Rgba) -> Self {
		[rgba.red, rgba.green, rgba.blue, rgba.alpha]
	}
}

fn srgb_component_to_linear(s: f32) -> f32 {
	if s < 0.04045 {
		return s / 12.92;
	}
	((s + 0.055) / 1.055).powf(2.4)
}
