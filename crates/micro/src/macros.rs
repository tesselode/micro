#[macro_export]
macro_rules! with_transform {
	($transform:expr, $body:block) => {{
		let _scope = $crate::push_transform($transform);
		$body
	}};
}

#[macro_export]
macro_rules! with_camera {
	($camera:expr, $body:block) => {{
		let _scope = $crate::use_3d_camera($camera);
		$body
	}};
}

#[macro_export]
macro_rules! write_to_stencil {
	($action:expr, $body:block) => {{
		let _scope = $crate::write_to_stencil($action);
		$body
	}};
}

#[macro_export]
macro_rules! use_stencil {
	($test:expr, $reference:expr, $body:block) => {{
		let _scope = $crate::use_stencil($test, $reference);
		$body
	}};
}

#[macro_export]
macro_rules! with_canvas {
	($canvas:expr, $body:block) => {{
		let _scope = $canvas.render_to();
		$body
	}};
}
