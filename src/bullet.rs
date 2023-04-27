mod components;
mod systems;

use bevy::prelude::*;
pub use components::*;
use systems::*;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Bullet>()
            .register_type::<Lifetime>()
            .add_system(bullet_collision)
            .add_system(move_bullets)
            .add_system(bullet_despawn);
    }
}
