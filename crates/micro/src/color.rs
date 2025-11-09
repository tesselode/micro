//! Color-related types. Mostly re-exports from [`palette`].

pub use palette::*;

/// An extension trait that adds useful constants to [`palette`] types.
pub trait ColorConstants {
	/// Pure white (255, 255, 255).
	const WHITE: Self;
	/// Pure black (0, 0, 0).
	const BLACK: Self;
	/// Pure red (255, 0, 0).
	const RED: Self;
	/// Pure green (0, 255, 0).
	const GREEN: Self;
	/// Pure blue (0, 0, 255).
	const BLUE: Self;
}

impl ColorConstants for LinSrgba {
	const WHITE: Self = LinSrgba::new(1.0, 1.0, 1.0, 1.0);
	const BLACK: Self = LinSrgba::new(0.0, 0.0, 0.0, 1.0);
	const RED: Self = LinSrgba::new(1.0, 0.0, 0.0, 1.0);
	const GREEN: Self = LinSrgba::new(0.0, 1.0, 0.0, 1.0);
	const BLUE: Self = LinSrgba::new(0.0, 0.0, 1.0, 1.0);
}

impl ColorConstants for LinSrgb {
	const WHITE: Self = LinSrgb::new(1.0, 1.0, 1.0);
	const BLACK: Self = LinSrgb::new(0.0, 0.0, 0.0);
	const RED: Self = LinSrgb::new(1.0, 0.0, 0.0);
	const GREEN: Self = LinSrgb::new(0.0, 1.0, 0.0);
	const BLUE: Self = LinSrgb::new(0.0, 0.0, 1.0);
}

pub(crate) fn lin_srgb_to_wgpu_color(lin_srgb: LinSrgb) -> wgpu::Color {
	wgpu::Color {
		r: lin_srgb.red.into(),
		g: lin_srgb.green.into(),
		b: lin_srgb.blue.into(),
		a: 1.0,
	}
}

pub(crate) fn lin_srgba_to_wgpu_color(lin_srgba: LinSrgba) -> wgpu::Color {
	wgpu::Color {
		r: lin_srgba.red.into(),
		g: lin_srgba.green.into(),
		b: lin_srgba.blue.into(),
		a: lin_srgba.alpha.into(),
	}
}
