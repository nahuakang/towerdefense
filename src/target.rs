mod components;
mod systems;

use bevy::prelude::*;
pub use components::*;
use systems::*;

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Target>()
            .register_type::<Health>()
            .add_system(move_targets)
            .add_system(target_death);
    }
}
