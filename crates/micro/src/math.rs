//! Math types and utilities. Mostly re-exports from [`glam`].

mod cardinal_direction;
mod circle;
mod clock_direction;
mod irect;
mod lerp;
mod line_segment;
mod polygon;
mod ray;
mod rect;
mod to_index;
mod urect;

pub use cardinal_direction::*;
pub use circle::*;
pub use clock_direction::*;
pub use irect::*;
pub use lerp::*;
pub use line_segment::*;
pub use polygon::*;
pub use ray::*;
pub use rect::*;
pub use to_index::*;
pub use urect::*;

pub use glam::*;
