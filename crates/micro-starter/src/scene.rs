use crate::globals::Globals;

pub mod gameplay;

pub type SceneChange = micro::scene::SceneChange<Globals, anyhow::Error>;
