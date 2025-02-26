pub use palette::*;

pub trait ColorConstants {
	const WHITE: Self;
	const BLACK: Self;
	const RED: Self;
	const GREEN: Self;
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
