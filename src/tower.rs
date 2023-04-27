use bevy::prelude::*;

mod components;
mod systems;

pub use components::*;
use systems::*;

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Tower>().add_system(tower_shooting);
    }
}
