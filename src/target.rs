mod components;
mod events;
mod resources;
mod systems;

use bevy::prelude::*;
pub use components::*;
pub use events::*;
use resources::*;
use systems::*;

use crate::GameState;

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Target>()
            .register_type::<Health>()
            .insert_resource(TargetPath {
                waypoints: vec![
                    Vec2::new(6.0, 2.0),
                    Vec2::new(6.0, 6.0),
                    Vec2::new(9.0, 9.0),
                ],
            })
            .add_event::<TargetDeathEvent>()
            .add_systems(
                (move_targets, hurt_player.after(move_targets), target_death)
                    .in_set(OnUpdate(GameState::Gameplay)),
            )
            .add_system(despawn_targets.in_schedule(OnExit(GameState::Gameplay)));
    }
}
