use bevy::prelude::*;

mod components;
mod systems;
mod ui;

pub use components::*;
use systems::*;
use ui::*;

use crate::GameState;

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Tower>()
            .add_systems(
                (tower_shooting, tower_button_clicked, create_ui_on_selection)
                    .in_set(OnUpdate(GameState::Gameplay)),
            )
            .add_system(despawn_towers.in_schedule(OnExit(GameState::Gameplay)));
    }
}
