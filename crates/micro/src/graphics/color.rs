use bytemuck::{Pod, Zeroable};
use derive_more::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

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

	pub fn rgba8(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
		Self::new(
			red as f32 / 255.0,
			green as f32 / 255.0,
			blue as f32 / 255.0,
			alpha as f32 / 255.0,
		)
	}

	pub fn rgb8(red: u8, green: u8, blue: u8) -> Self {
		Self::rgba8(red, green, blue, 255)
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
