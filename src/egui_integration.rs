use std::collections::HashMap;

use egui::{FullOutput, RawInput};
use glam::IVec2;
use palette::{LinSrgba, Srgba};

use crate::{
	graphics::{
		mesh::{Mesh, Vertex},
		stencil::{StencilAction, StencilTest},
		texture::{Texture, TextureSettings},
		DrawParams,
	},
	input::Scancode,
	Context,
};

pub fn egui_raw_input(ctx: &Context, events: &[sdl2::event::Event]) -> RawInput {
	let modifiers = egui::Modifiers {
		alt: ctx.is_key_down(Scancode::LAlt) || ctx.is_key_down(Scancode::RAlt),
		ctrl: ctx.is_key_down(Scancode::LCtrl) || ctx.is_key_down(Scancode::RCtrl),
		shift: ctx.is_key_down(Scancode::LShift) || ctx.is_key_down(Scancode::RShift),
		mac_cmd: ctx.is_key_down(Scancode::LGui) || ctx.is_key_down(Scancode::RGui),
		command: ctx.is_key_down(Scancode::LGui) || ctx.is_key_down(Scancode::RGui),
	};
	let scaling_factor = egui_scaling_factor(ctx);
	RawInput {
		screen_rect: Some(egui::Rect::from_min_size(
			Default::default(),
			glam_vec2_to_egui_vec2(ctx.window_size().as_vec2()) / scaling_factor,
		)),
		modifiers,
		events: events
			.iter()
			.cloned()
			.filter_map(|event| sdl2_event_to_egui_event(ctx, event, modifiers))
			.collect(),
		pixels_per_point: Some(scaling_factor),
		..Default::default()
	}
}

pub fn draw_egui_output(
	ctx: &mut Context,
	egui_ctx: &egui::Context,
	output: FullOutput,
	textures: &mut HashMap<egui::TextureId, Texture>,
) {
	patch_textures(&output, textures, ctx);
	let scaling_factor = egui_scaling_factor(ctx);
	ctx.clear_stencil();
	for clipped_primitive in egui_ctx.tessellate(output.shapes) {
		match clipped_primitive.primitive {
			egui::epaint::Primitive::Mesh(mesh) => {
				ctx.write_to_stencil(StencilAction::Replace(1), |ctx| {
					let clip_rect_points = egui_rect_to_micro_rect(clipped_primitive.clip_rect);
					let clip_rect_pixels = crate::math::Rect::new(
						clip_rect_points.top_left * scaling_factor,
						clip_rect_points.bottom_right() * scaling_factor,
					);
					Mesh::rectangle(ctx, clip_rect_pixels).draw(ctx, DrawParams::new());
				});
				ctx.with_stencil(StencilTest::Equal, 1, |ctx| {
					let texture_id = mesh.texture_id;
					egui_mesh_to_micro_mesh(ctx, mesh).draw_textured(
						ctx,
						textures.get(&texture_id).expect("missing egui texture"),
						DrawParams::new().scaled(glam::Vec2::splat(scaling_factor)),
					);
				});
				ctx.clear_stencil();
			}
			egui::epaint::Primitive::Callback(_) => unimplemented!(),
		}
	}
	for texture_id in output.textures_delta.free {
		textures.remove(&texture_id);
	}
}

fn patch_textures(
	output: &FullOutput,
	textures: &mut HashMap<egui::TextureId, Texture>,
	ctx: &mut Context,
) {
	for (texture_id, delta) in &output.textures_delta.set {
		if let Some(texture) = textures.get(texture_id) {
			let top_left = delta
				.pos
				.map(|[x, y]| IVec2::new(x as i32, y as i32))
				.unwrap_or_default();
			texture.replace(top_left, &egui_image_data_to_micro_image_data(&delta.image))
		} else {
			textures.insert(
				*texture_id,
				Texture::from_image_data(
					ctx,
					&egui_image_data_to_micro_image_data(&delta.image),
					TextureSettings::default(),
				),
			);
		}
	}
}

fn sdl2_event_to_egui_event(
	ctx: &Context,
	event: sdl2::event::Event,
	modifiers: egui::Modifiers,
) -> Option<egui::Event> {
	let scaling_factor = egui_scaling_factor(ctx);
	match event {
		sdl2::event::Event::KeyDown {
			scancode, repeat, ..
		} => Some(egui::Event::Key {
			key: sdl2_scancode_to_egui_key(scancode?)?,
			pressed: true,
			modifiers,
			repeat,
		}),
		sdl2::event::Event::KeyUp {
			scancode, repeat, ..
		} => Some(egui::Event::Key {
			key: sdl2_scancode_to_egui_key(scancode?)?,
			pressed: false,
			modifiers,
			repeat,
		}),
		sdl2::event::Event::TextInput { text, .. } => Some(egui::Event::Text(text)),
		sdl2::event::Event::MouseMotion { x, y, .. } => Some(egui::Event::PointerMoved(
			egui::pos2(x as f32 / scaling_factor, y as f32 / scaling_factor),
		)),
		sdl2::event::Event::MouseButtonDown {
			mouse_btn, x, y, ..
		} => Some(egui::Event::PointerButton {
			pos: egui::pos2(x as f32 / scaling_factor, y as f32 / scaling_factor),
			button: sdl2_mouse_button_to_egui_pointer_button(mouse_btn)?,
			pressed: true,
			modifiers,
		}),
		sdl2::event::Event::MouseButtonUp {
			mouse_btn, x, y, ..
		} => Some(egui::Event::PointerButton {
			pos: egui::pos2(x as f32 / scaling_factor, y as f32 / scaling_factor),
			button: sdl2_mouse_button_to_egui_pointer_button(mouse_btn)?,
			pressed: false,
			modifiers,
		}),
		sdl2::event::Event::MouseWheel { x, y, .. } => {
			Some(egui::Event::Scroll(egui::vec2(x as f32, y as f32)))
		}
		_ => None,
	}
}

fn sdl2_scancode_to_egui_key(scancode: sdl2::keyboard::Scancode) -> Option<egui::Key> {
	match scancode {
		sdl2::keyboard::Scancode::A => Some(egui::Key::A),
		sdl2::keyboard::Scancode::B => Some(egui::Key::B),
		sdl2::keyboard::Scancode::C => Some(egui::Key::C),
		sdl2::keyboard::Scancode::D => Some(egui::Key::D),
		sdl2::keyboard::Scancode::E => Some(egui::Key::E),
		sdl2::keyboard::Scancode::F => Some(egui::Key::F),
		sdl2::keyboard::Scancode::G => Some(egui::Key::G),
		sdl2::keyboard::Scancode::H => Some(egui::Key::H),
		sdl2::keyboard::Scancode::I => Some(egui::Key::I),
		sdl2::keyboard::Scancode::J => Some(egui::Key::J),
		sdl2::keyboard::Scancode::K => Some(egui::Key::K),
		sdl2::keyboard::Scancode::L => Some(egui::Key::L),
		sdl2::keyboard::Scancode::M => Some(egui::Key::M),
		sdl2::keyboard::Scancode::N => Some(egui::Key::N),
		sdl2::keyboard::Scancode::O => Some(egui::Key::O),
		sdl2::keyboard::Scancode::P => Some(egui::Key::P),
		sdl2::keyboard::Scancode::Q => Some(egui::Key::Q),
		sdl2::keyboard::Scancode::R => Some(egui::Key::R),
		sdl2::keyboard::Scancode::S => Some(egui::Key::S),
		sdl2::keyboard::Scancode::T => Some(egui::Key::T),
		sdl2::keyboard::Scancode::U => Some(egui::Key::U),
		sdl2::keyboard::Scancode::V => Some(egui::Key::V),
		sdl2::keyboard::Scancode::W => Some(egui::Key::W),
		sdl2::keyboard::Scancode::X => Some(egui::Key::X),
		sdl2::keyboard::Scancode::Y => Some(egui::Key::Y),
		sdl2::keyboard::Scancode::Z => Some(egui::Key::Z),
		sdl2::keyboard::Scancode::Num1 => Some(egui::Key::Num1),
		sdl2::keyboard::Scancode::Num2 => Some(egui::Key::Num2),
		sdl2::keyboard::Scancode::Num3 => Some(egui::Key::Num3),
		sdl2::keyboard::Scancode::Num4 => Some(egui::Key::Num4),
		sdl2::keyboard::Scancode::Num5 => Some(egui::Key::Num5),
		sdl2::keyboard::Scancode::Num6 => Some(egui::Key::Num6),
		sdl2::keyboard::Scancode::Num7 => Some(egui::Key::Num7),
		sdl2::keyboard::Scancode::Num8 => Some(egui::Key::Num8),
		sdl2::keyboard::Scancode::Num9 => Some(egui::Key::Num9),
		sdl2::keyboard::Scancode::Num0 => Some(egui::Key::Num0),
		sdl2::keyboard::Scancode::Return => Some(egui::Key::Enter),
		sdl2::keyboard::Scancode::Escape => Some(egui::Key::Escape),
		sdl2::keyboard::Scancode::Backspace => Some(egui::Key::Backspace),
		sdl2::keyboard::Scancode::Tab => Some(egui::Key::Tab),
		sdl2::keyboard::Scancode::Space => Some(egui::Key::Space),
		sdl2::keyboard::Scancode::Minus => Some(egui::Key::Minus),
		sdl2::keyboard::Scancode::Equals => Some(egui::Key::PlusEquals),
		sdl2::keyboard::Scancode::F1 => Some(egui::Key::F1),
		sdl2::keyboard::Scancode::F2 => Some(egui::Key::F2),
		sdl2::keyboard::Scancode::F3 => Some(egui::Key::F3),
		sdl2::keyboard::Scancode::F4 => Some(egui::Key::F4),
		sdl2::keyboard::Scancode::F5 => Some(egui::Key::F5),
		sdl2::keyboard::Scancode::F6 => Some(egui::Key::F6),
		sdl2::keyboard::Scancode::F7 => Some(egui::Key::F7),
		sdl2::keyboard::Scancode::F8 => Some(egui::Key::F8),
		sdl2::keyboard::Scancode::F9 => Some(egui::Key::F9),
		sdl2::keyboard::Scancode::F10 => Some(egui::Key::F10),
		sdl2::keyboard::Scancode::F11 => Some(egui::Key::F11),
		sdl2::keyboard::Scancode::F12 => Some(egui::Key::F12),
		sdl2::keyboard::Scancode::Insert => Some(egui::Key::Insert),
		sdl2::keyboard::Scancode::Home => Some(egui::Key::Home),
		sdl2::keyboard::Scancode::PageUp => Some(egui::Key::PageUp),
		sdl2::keyboard::Scancode::Delete => Some(egui::Key::Delete),
		sdl2::keyboard::Scancode::End => Some(egui::Key::End),
		sdl2::keyboard::Scancode::PageDown => Some(egui::Key::PageDown),
		sdl2::keyboard::Scancode::Right => Some(egui::Key::ArrowRight),
		sdl2::keyboard::Scancode::Left => Some(egui::Key::ArrowLeft),
		sdl2::keyboard::Scancode::Down => Some(egui::Key::ArrowDown),
		sdl2::keyboard::Scancode::Up => Some(egui::Key::ArrowUp),
		sdl2::keyboard::Scancode::KpMinus => Some(egui::Key::Minus),
		sdl2::keyboard::Scancode::KpPlus => Some(egui::Key::PlusEquals),
		sdl2::keyboard::Scancode::KpEnter => Some(egui::Key::Enter),
		sdl2::keyboard::Scancode::Kp1 => Some(egui::Key::Num1),
		sdl2::keyboard::Scancode::Kp2 => Some(egui::Key::Num2),
		sdl2::keyboard::Scancode::Kp3 => Some(egui::Key::Num3),
		sdl2::keyboard::Scancode::Kp4 => Some(egui::Key::Num4),
		sdl2::keyboard::Scancode::Kp5 => Some(egui::Key::Num5),
		sdl2::keyboard::Scancode::Kp6 => Some(egui::Key::Num6),
		sdl2::keyboard::Scancode::Kp7 => Some(egui::Key::Num7),
		sdl2::keyboard::Scancode::Kp8 => Some(egui::Key::Num8),
		sdl2::keyboard::Scancode::Kp9 => Some(egui::Key::Num9),
		sdl2::keyboard::Scancode::Kp0 => Some(egui::Key::Num0),
		sdl2::keyboard::Scancode::F13 => Some(egui::Key::F13),
		sdl2::keyboard::Scancode::F14 => Some(egui::Key::F14),
		sdl2::keyboard::Scancode::F15 => Some(egui::Key::F15),
		sdl2::keyboard::Scancode::F16 => Some(egui::Key::F16),
		sdl2::keyboard::Scancode::F17 => Some(egui::Key::F17),
		sdl2::keyboard::Scancode::F18 => Some(egui::Key::F18),
		sdl2::keyboard::Scancode::F19 => Some(egui::Key::F19),
		sdl2::keyboard::Scancode::F20 => Some(egui::Key::F20),
		_ => None,
	}
}

fn sdl2_mouse_button_to_egui_pointer_button(
	mouse_button: sdl2::mouse::MouseButton,
) -> Option<egui::PointerButton> {
	match mouse_button {
		sdl2::mouse::MouseButton::Left => Some(egui::PointerButton::Primary),
		sdl2::mouse::MouseButton::Middle => Some(egui::PointerButton::Middle),
		sdl2::mouse::MouseButton::Right => Some(egui::PointerButton::Secondary),
		sdl2::mouse::MouseButton::X1 => Some(egui::PointerButton::Extra1),
		sdl2::mouse::MouseButton::X2 => Some(egui::PointerButton::Extra2),
		_ => None,
	}
}

pub fn egui_took_sdl2_event(egui_ctx: &egui::Context, event: &sdl2::event::Event) -> bool {
	match event {
		sdl2::event::Event::KeyDown { .. }
		| sdl2::event::Event::KeyUp { .. }
		| sdl2::event::Event::TextEditing { .. }
		| sdl2::event::Event::TextInput { .. } => egui_ctx.wants_keyboard_input(),
		sdl2::event::Event::MouseMotion { .. }
		| sdl2::event::Event::MouseButtonDown { .. }
		| sdl2::event::Event::MouseButtonUp { .. }
		| sdl2::event::Event::MouseWheel { .. } => egui_ctx.wants_pointer_input(),
		_ => false,
	}
}

fn glam_vec2_to_egui_vec2(v: glam::Vec2) -> egui::Vec2 {
	egui::vec2(v.x, v.y)
}

fn egui_pos2_to_glam_vec2(v: egui::Pos2) -> glam::Vec2 {
	glam::Vec2::new(v.x, v.y)
}

fn egui_color32_to_palette_lin_srgba(v: egui::epaint::Color32) -> LinSrgba {
	Srgba::new(v.r(), v.g(), v.b(), v.a()).into_linear()
}

fn egui_rect_to_micro_rect(v: egui::Rect) -> crate::math::Rect {
	crate::math::Rect::new(egui_pos2_to_glam_vec2(v.min), egui_pos2_to_glam_vec2(v.max))
}

fn egui_mesh_to_micro_mesh(ctx: &Context, egui_mesh: egui::Mesh) -> Mesh {
	let vertices = egui_mesh
		.vertices
		.iter()
		.copied()
		.map(egui_vertex_to_micro_vertex)
		.collect::<Vec<_>>();
	Mesh::new(ctx, &vertices, &egui_mesh.indices)
}

fn egui_vertex_to_micro_vertex(vertex: egui::epaint::Vertex) -> Vertex {
	Vertex {
		position: egui_pos2_to_glam_vec2(vertex.pos),
		texture_coords: egui_pos2_to_glam_vec2(vertex.uv),
		color: egui_color32_to_palette_lin_srgba(vertex.color),
	}
}

fn egui_image_data_to_micro_image_data(
	image_data: &egui::ImageData,
) -> crate::graphics::image_data::ImageData {
	let size = glam::UVec2::new(image_data.width() as u32, image_data.height() as u32);
	let mut pixels = vec![];
	match image_data {
		egui::ImageData::Color(color_image) => {
			for pixel in &color_image.pixels {
				pixels.push(pixel.r());
				pixels.push(pixel.g());
				pixels.push(pixel.b());
				pixels.push(pixel.a());
			}
		}
		egui::ImageData::Font(font_image) => {
			for pixel in font_image.srgba_pixels(None) {
				pixels.push(pixel.r());
				pixels.push(pixel.g());
				pixels.push(pixel.b());
				pixels.push(pixel.a());
			}
		}
	}
	crate::graphics::image_data::ImageData { size, pixels }
}

fn egui_scaling_factor(ctx: &Context) -> f32 {
	let Ok(monitor_resolution) = ctx.monitor_resolution() else {
		return 1.0
	};
	(monitor_resolution.y as f32 / 1080.0).max(1.0)
}
