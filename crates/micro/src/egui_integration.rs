use std::{collections::HashMap, time::Duration};

use egui::{FullOutput, RawInput, ViewportId, ViewportInfo};
use glam::uvec2;
use image::ImageBuffer;
use palette::{LinSrgba, Srgba};
use winit::{event::MouseButton, keyboard::KeyCode};

use crate::{
	Event, Push,
	context::Context,
	graphics::{
		Vertex2d,
		mesh::Mesh,
		texture::{Texture, TextureSettings},
	},
	input::MouseScrollDelta,
};

pub fn egui_raw_input(ctx: &Context, events: &[Event], delta_time: Duration) -> RawInput {
	let modifiers = egui::Modifiers {
		alt: ctx.is_key_down(KeyCode::AltLeft) || ctx.is_key_down(KeyCode::AltRight),
		ctrl: ctx.is_key_down(KeyCode::ControlLeft) || ctx.is_key_down(KeyCode::ControlRight),
		shift: ctx.is_key_down(KeyCode::ShiftLeft) || ctx.is_key_down(KeyCode::ShiftRight),
		mac_cmd: ctx.is_key_down(KeyCode::SuperLeft) || ctx.is_key_down(KeyCode::SuperRight),
		command: ctx.is_key_down(KeyCode::SuperLeft) || ctx.is_key_down(KeyCode::SuperRight),
	};
	let scaling_factor = ctx.window_scale();
	RawInput {
		viewports: std::iter::once((
			ViewportId::ROOT,
			ViewportInfo {
				native_pixels_per_point: Some(scaling_factor),
				..Default::default()
			},
		))
		.collect(),
		screen_rect: Some(egui::Rect::from_min_size(
			Default::default(),
			glam_vec2_to_egui_vec2(ctx.window_size().as_vec2()) / scaling_factor,
		)),
		modifiers,
		events: events
			.iter()
			.cloned()
			.filter_map(|event| micro_event_to_egui_event(ctx, event, modifiers))
			.collect(),
		predicted_dt: delta_time.as_secs_f32(),
		..Default::default()
	}
}

pub fn draw_egui_output(
	ctx: &mut Context,
	egui_ctx: &egui::Context,
	output: FullOutput,
	textures: &mut HashMap<egui::TextureId, Texture>,
) {
	patch_textures(ctx, &output, textures);
	let scaling_factor = ctx.window_scale();
	for clipped_primitive in egui_ctx.tessellate(output.shapes, scaling_factor) {
		match clipped_primitive.primitive {
			egui::epaint::Primitive::Mesh(mesh) => {
				let texture_id = mesh.texture_id;
				let clip_rect_points = egui_rect_to_micro_rect(clipped_primitive.clip_rect);
				let clip_rect_pixels = crate::math::Rect::from_corners(
					clip_rect_points.top_left * scaling_factor,
					clip_rect_points.bottom_right() * scaling_factor,
				)
				.as_urect();
				let ctx = &mut ctx.push(Push {
					scissor_rect: Some(Some(clip_rect_pixels)),
					..Default::default()
				});
				egui_mesh_to_micro_mesh(ctx, mesh)
					.texture(textures.get(&texture_id).expect("missing egui texture"))
					.scaled_2d(glam::Vec2::splat(scaling_factor))
					.draw(ctx);
			}
			egui::epaint::Primitive::Callback(..) => unimplemented!(),
		}
	}
	for texture_id in output.textures_delta.free {
		textures.remove(&texture_id);
	}
}

fn patch_textures(
	ctx: &mut Context,
	output: &FullOutput,
	textures: &mut HashMap<egui::TextureId, Texture>,
) {
	for (texture_id, delta) in &output.textures_delta.set {
		if let Some(texture) = textures.get_mut(texture_id) {
			let top_left = delta
				.pos
				.map(|[x, y]| uvec2(x as u32, y as u32))
				.unwrap_or_default();
			let bottom_right =
				top_left + uvec2(delta.image.size()[0] as u32, delta.image.size()[1] as u32);
			if bottom_right.x >= texture.size().x || bottom_right.y >= texture.size().y {
				*texture = texture.resized(ctx, bottom_right);
			}
			texture.replace(
				ctx,
				top_left,
				&egui_image_data_to_image_buffer(&delta.image),
			);
		} else {
			textures.insert(
				*texture_id,
				Texture::from_image(
					ctx,
					&egui_image_data_to_image_buffer(&delta.image),
					TextureSettings::default(),
				),
			);
		}
	}
}

fn micro_event_to_egui_event(
	ctx: &Context,
	event: Event,
	modifiers: egui::Modifiers,
) -> Option<egui::Event> {
	let scaling_factor = ctx.window_scale();
	match event {
		Event::KeyPressed { key, is_repeat, .. } => Some(egui::Event::Key {
			key: micro_key_code_to_egui_key(key)?,
			physical_key: None,
			pressed: true,
			modifiers,
			repeat: is_repeat,
		}),
		Event::KeyReleased(key) => Some(egui::Event::Key {
			key: micro_key_code_to_egui_key(key)?,
			physical_key: None,
			pressed: false,
			modifiers,
			repeat: false,
		}),
		Event::TextInput(text) => Some(egui::Event::Text(text)),
		Event::CursorPositionChanged(position) => Some(egui::Event::PointerMoved(egui::pos2(
			position.x / scaling_factor,
			position.y / scaling_factor,
		))),
		Event::MouseButtonPressed { button, position } => Some(egui::Event::PointerButton {
			pos: egui::pos2(position.x / scaling_factor, position.y / scaling_factor),
			button: micro_mouse_button_to_egui_pointer_button(button)?,
			pressed: true,
			modifiers,
		}),
		Event::MouseButtonReleased { button, position } => Some(egui::Event::PointerButton {
			pos: egui::pos2(position.x / scaling_factor, position.y / scaling_factor),
			button: micro_mouse_button_to_egui_pointer_button(button)?,
			pressed: false,
			modifiers,
		}),
		Event::MouseWheelMoved(delta) => Some(egui::Event::MouseWheel {
			unit: match delta {
				MouseScrollDelta::Line(..) => egui::MouseWheelUnit::Line,
				MouseScrollDelta::Pixel(..) => egui::MouseWheelUnit::Point,
			},
			delta: match delta {
				MouseScrollDelta::Line(delta) | MouseScrollDelta::Pixel(delta) => {
					egui::vec2(delta.x, delta.y)
				}
			},
			modifiers,
		}),
		_ => None,
	}
}

fn micro_key_code_to_egui_key(key: KeyCode) -> Option<egui::Key> {
	match key {
		KeyCode::KeyA => Some(egui::Key::A),
		KeyCode::KeyB => Some(egui::Key::B),
		KeyCode::KeyC => Some(egui::Key::C),
		KeyCode::KeyD => Some(egui::Key::D),
		KeyCode::KeyE => Some(egui::Key::E),
		KeyCode::KeyF => Some(egui::Key::F),
		KeyCode::KeyG => Some(egui::Key::G),
		KeyCode::KeyH => Some(egui::Key::H),
		KeyCode::KeyI => Some(egui::Key::I),
		KeyCode::KeyJ => Some(egui::Key::J),
		KeyCode::KeyK => Some(egui::Key::K),
		KeyCode::KeyL => Some(egui::Key::L),
		KeyCode::KeyM => Some(egui::Key::M),
		KeyCode::KeyN => Some(egui::Key::N),
		KeyCode::KeyO => Some(egui::Key::O),
		KeyCode::KeyP => Some(egui::Key::P),
		KeyCode::KeyQ => Some(egui::Key::Q),
		KeyCode::KeyR => Some(egui::Key::R),
		KeyCode::KeyS => Some(egui::Key::S),
		KeyCode::KeyT => Some(egui::Key::T),
		KeyCode::KeyU => Some(egui::Key::U),
		KeyCode::KeyV => Some(egui::Key::V),
		KeyCode::KeyW => Some(egui::Key::W),
		KeyCode::KeyX => Some(egui::Key::X),
		KeyCode::KeyY => Some(egui::Key::Y),
		KeyCode::KeyZ => Some(egui::Key::Z),
		KeyCode::Digit1 => Some(egui::Key::Num1),
		KeyCode::Digit2 => Some(egui::Key::Num2),
		KeyCode::Digit3 => Some(egui::Key::Num3),
		KeyCode::Digit4 => Some(egui::Key::Num4),
		KeyCode::Digit5 => Some(egui::Key::Num5),
		KeyCode::Digit6 => Some(egui::Key::Num6),
		KeyCode::Digit7 => Some(egui::Key::Num7),
		KeyCode::Digit8 => Some(egui::Key::Num8),
		KeyCode::Digit9 => Some(egui::Key::Num9),
		KeyCode::Digit0 => Some(egui::Key::Num0),
		KeyCode::Enter => Some(egui::Key::Enter),
		KeyCode::Escape => Some(egui::Key::Escape),
		KeyCode::Backspace => Some(egui::Key::Backspace),
		KeyCode::Tab => Some(egui::Key::Tab),
		KeyCode::Space => Some(egui::Key::Space),
		KeyCode::Minus => Some(egui::Key::Minus),
		KeyCode::Equal => Some(egui::Key::Equals),
		KeyCode::F1 => Some(egui::Key::F1),
		KeyCode::F2 => Some(egui::Key::F2),
		KeyCode::F3 => Some(egui::Key::F3),
		KeyCode::F4 => Some(egui::Key::F4),
		KeyCode::F5 => Some(egui::Key::F5),
		KeyCode::F6 => Some(egui::Key::F6),
		KeyCode::F7 => Some(egui::Key::F7),
		KeyCode::F8 => Some(egui::Key::F8),
		KeyCode::F9 => Some(egui::Key::F9),
		KeyCode::F10 => Some(egui::Key::F10),
		KeyCode::F11 => Some(egui::Key::F11),
		KeyCode::F12 => Some(egui::Key::F12),
		KeyCode::Insert => Some(egui::Key::Insert),
		KeyCode::Home => Some(egui::Key::Home),
		KeyCode::PageUp => Some(egui::Key::PageUp),
		KeyCode::Delete => Some(egui::Key::Delete),
		KeyCode::End => Some(egui::Key::End),
		KeyCode::PageDown => Some(egui::Key::PageDown),
		KeyCode::ArrowRight => Some(egui::Key::ArrowRight),
		KeyCode::ArrowLeft => Some(egui::Key::ArrowLeft),
		KeyCode::ArrowDown => Some(egui::Key::ArrowDown),
		KeyCode::ArrowUp => Some(egui::Key::ArrowUp),
		KeyCode::NumpadSubtract => Some(egui::Key::Minus),
		KeyCode::NumpadAdd => Some(egui::Key::Plus),
		KeyCode::NumpadEnter => Some(egui::Key::Enter),
		KeyCode::Numpad1 => Some(egui::Key::Num1),
		KeyCode::Numpad2 => Some(egui::Key::Num2),
		KeyCode::Numpad3 => Some(egui::Key::Num3),
		KeyCode::Numpad4 => Some(egui::Key::Num4),
		KeyCode::Numpad5 => Some(egui::Key::Num5),
		KeyCode::Numpad6 => Some(egui::Key::Num6),
		KeyCode::Numpad7 => Some(egui::Key::Num7),
		KeyCode::Numpad8 => Some(egui::Key::Num8),
		KeyCode::Numpad9 => Some(egui::Key::Num9),
		KeyCode::Numpad0 => Some(egui::Key::Num0),
		KeyCode::F13 => Some(egui::Key::F13),
		KeyCode::F14 => Some(egui::Key::F14),
		KeyCode::F15 => Some(egui::Key::F15),
		KeyCode::F16 => Some(egui::Key::F16),
		KeyCode::F17 => Some(egui::Key::F17),
		KeyCode::F18 => Some(egui::Key::F18),
		KeyCode::F19 => Some(egui::Key::F19),
		KeyCode::F20 => Some(egui::Key::F20),
		_ => None,
	}
}

fn micro_mouse_button_to_egui_pointer_button(
	mouse_button: MouseButton,
) -> Option<egui::PointerButton> {
	match mouse_button {
		MouseButton::Left => Some(egui::PointerButton::Primary),
		MouseButton::Middle => Some(egui::PointerButton::Middle),
		MouseButton::Right => Some(egui::PointerButton::Secondary),
		MouseButton::Back => Some(egui::PointerButton::Extra1),
		MouseButton::Forward => Some(egui::PointerButton::Extra2),
		_ => None,
	}
}

pub fn egui_took_event(egui_ctx: &egui::Context, event: &Event) -> bool {
	match event {
		Event::KeyPressed { .. } | Event::KeyReleased(..) | Event::TextInput(..) => {
			egui_ctx.wants_keyboard_input()
		}
		Event::CursorPositionChanged(..)
		| Event::MouseButtonPressed { .. }
		| Event::MouseButtonReleased { .. }
		| Event::MouseWheelMoved { .. } => egui_ctx.wants_pointer_input(),
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
	crate::math::Rect::from_corners(egui_pos2_to_glam_vec2(v.min), egui_pos2_to_glam_vec2(v.max))
}

fn egui_mesh_to_micro_mesh(ctx: &mut Context, egui_mesh: egui::Mesh) -> Mesh<Vertex2d> {
	let vertices = egui_mesh
		.vertices
		.iter()
		.copied()
		.map(egui_vertex_to_micro_vertex_2d)
		.collect::<Vec<_>>();
	Mesh::new(ctx, &vertices, &egui_mesh.indices)
}

fn egui_vertex_to_micro_vertex_2d(vertex: egui::epaint::Vertex) -> Vertex2d {
	Vertex2d {
		position: egui_pos2_to_glam_vec2(vertex.pos),
		texture_coords: egui_pos2_to_glam_vec2(vertex.uv),
		color: egui_color32_to_palette_lin_srgba(vertex.color),
	}
}

fn egui_image_data_to_image_buffer(
	image_data: &egui::ImageData,
) -> ImageBuffer<image::Rgba<u8>, Vec<u8>> {
	match image_data {
		egui::ImageData::Color(color_image) => ImageBuffer::from_fn(
			image_data.width() as u32,
			image_data.height() as u32,
			|x, y| {
				image::Rgba(
					color_image.pixels[coords_to_index(x, y, image_data.width() as u32)].to_array(),
				)
			},
		),
	}
}

fn coords_to_index(x: u32, y: u32, width: u32) -> usize {
	(x + width * y) as usize
}
