use hecs::Entity;
use micro_ecs::EntityWillDespawn;

#[derive(Debug, Clone, PartialEq)]
pub enum GameEvent {
	EntityWillDespawn(Entity),
}

impl From<EntityWillDespawn> for GameEvent {
	fn from(EntityWillDespawn(entity): EntityWillDespawn) -> Self {
		Self::EntityWillDespawn(entity)
	}
}
