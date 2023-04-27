use self::context::GameplayContext;

use super::Scene;

mod context;

pub struct Gameplay {
	gameplay_ctx: GameplayContext,
}

impl Gameplay {
	pub fn new() -> Self {
		Self {
			gameplay_ctx: GameplayContext::new(),
		}
	}
}

impl Scene for Gameplay {}
