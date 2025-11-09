//! Micro is an SDL3 and wgpu-based framework for developing video games and interactive
//! applications.

#![allow(clippy::large_enum_variant)]
#![warn(missing_docs)]
#![allow(clippy::tabs_in_doc_comments)]

mod app;
pub mod color;
mod context;
mod egui_integration;
mod event;
pub mod graphics;
pub mod input;
mod log;
pub mod math;
mod time;
pub mod tween;
mod window;

pub use app::*;
pub use context::*;
pub use event::*;
pub use time::*;
pub use window::*;

pub use egui;
pub use image;
