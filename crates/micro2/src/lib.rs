#![allow(clippy::large_enum_variant)]

mod app;
pub mod color;
mod context;
mod event;
pub mod graphics;
pub mod input;
mod log;
pub mod math;
mod time;
pub mod tween;
mod window;

pub use app::*;
pub use context::{Context, ContextSettings, run};
pub use event::*;
pub use time::*;
pub use window::*;

pub use egui as debug_ui;
pub use image;
