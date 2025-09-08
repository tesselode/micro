pub struct Entities(hecs::World);

impl Entities {
	pub(crate) fn new() -> Self {
		Self(hecs::World::new())
	}
}
