mod components;
mod systems;

use bevy::prelude::*;
pub use components::*;
use systems::*;

use crate::GameState;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Bullet>()
            .register_type::<Lifetime>()
            .add_systems(
                (bullet_collision, move_bullets, bullet_despawn)
                    .in_set(OnUpdate(GameState::Gameplay)),
            )
            .add_system(despawn_bullets.in_schedule(OnExit(GameState::Gameplay)));
    }
}
