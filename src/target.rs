mod components;
mod events;
mod systems;

use bevy::prelude::*;
pub use components::*;
pub use events::*;
use systems::*;

use crate::GameState;

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Target>()
            .register_type::<Health>()
            .add_event::<TargetDeathEvent>()
            .add_systems((move_targets, target_death).in_set(OnUpdate(GameState::Gameplay)))
            .add_system(despawn_targets.in_schedule(OnExit(GameState::Gameplay)));
    }
}
