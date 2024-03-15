#[macro_export]
macro_rules! push_transform {
	($transform:expr) => {
		let _scope = $crate::push_transform($transform);
	};
	($transform:expr, $body:expr) => {{
		let _scope = $crate::push_transform($transform);
		$body
	}};
}

#[macro_export]
macro_rules! push_translation_2d {
	($translation_2d:expr) => {
		let _scope = $crate::push_translation_2d($translation_2d);
	};
	($translation_2d:expr, $body:expr) => {{
		let _scope = $crate::push_translation_2d($translation_2d);
		$body
	}};
}

#[macro_export]
macro_rules! push_translation_3d {
	($translation_3d:expr) => {
		let _scope = $crate::push_translation_3d($translation_3d);
	};
	($translation_3d:expr, $body:expr) => {{
		let _scope = $crate::push_translation_3d($translation_3d);
		$body
	}};
}

#[macro_export]
macro_rules! push_translation_x {
	($translation_x:expr) => {
		let _scope = $crate::push_translation_x($translation_x);
	};
	($translation_x:expr, $body:expr) => {{
		let _scope = $crate::push_translation_x($translation_x);
		$body
	}};
}

#[macro_export]
macro_rules! push_translation_y {
	($translation_y:expr) => {
		let _scope = $crate::push_translation_y($translation_y);
	};
	($translation_y:expr, $body:expr) => {{
		let _scope = $crate::push_translation_y($translation_y);
		$body
	}};
}

#[macro_export]
macro_rules! push_translation_z {
	($translation_z:expr) => {
		let _scope = $crate::push_translation_z($translation_z);
	};
	($translation_z:expr, $body:expr) => {{
		let _scope = $crate::push_translation_z($translation_z);
		$body
	}};
}

#[macro_export]
macro_rules! push_scale_2d {
	($scale_2d:expr) => {
		let _scope = $crate::push_scale_2d($scale_2d);
	};
	($scale_2d:expr, $body:expr) => {{
		let _scope = $crate::push_scale_2d($scale_2d);
		$body
	}};
}

#[macro_export]
macro_rules! push_scale_3d {
	($scale_3d:expr) => {
		let _scope = $crate::push_scale_3d($scale_3d);
	};
	($scale_3d:expr, $body:expr) => {{
		let _scope = $crate::push_scale_3d($scale_3d);
		$body
	}};
}

#[macro_export]
macro_rules! push_rotation_x {
	($rotation_x:expr) => {
		let _scope = $crate::push_rotation_x($rotation_x);
	};
	($rotation_x:expr, $body:expr) => {{
		let _scope = $crate::push_rotation_x($rotation_x);
		$body
	}};
}

#[macro_export]
macro_rules! push_rotation_y {
	($rotation_y:expr) => {
		let _scope = $crate::push_rotation_y($rotation_y);
	};
	($rotation_y:expr, $body:expr) => {{
		let _scope = $crate::push_rotation_y($rotation_y);
		$body
	}};
}

#[macro_export]
macro_rules! push_rotation_z {
	($rotation_z:expr) => {
		let _scope = $crate::push_rotation_z($rotation_z);
	};
	($rotation_z:expr, $body:expr) => {{
		let _scope = $crate::push_rotation_z($rotation_z);
		$body
	}};
}

#[macro_export]
macro_rules! write_to_stencil {
	($action:expr) => {
		let _scope = $crate::write_to_stencil($action);
	};
	($action:expr, $body:expr) => {{
		let _scope = $crate::write_to_stencil($action);
		$body
	}};
}

#[macro_export]
macro_rules! use_3d_camera {
	($camera:expr) => {
		let _scope = $crate::use_3d_camera($camera);
	};
	($camera:expr, $body:expr) => {{
		let _scope = $crate::use_3d_camera($camera);
		$body
	}};
}

#[macro_export]
macro_rules! use_stencil {
	($test:expr, $reference:expr) => {
		let _scope = $crate::use_stencil($test, $reference);
	};
	($test:expr, $reference:expr, $body:expr) => {{
		let _scope = $crate::use_stencil($test, $reference);
		$body
	}};
}

#[macro_export]
macro_rules! render_to_canvas {
	($canvas:expr) => {
		let _scope = $canvas.render_to();
	};
	($canvas:expr, $body:expr) => {{
		let _scope = $canvas.render_to();
		$body
	}};
}
